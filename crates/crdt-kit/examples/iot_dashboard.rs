//! Example: IoT Sensor Dashboard with offline-first sync.
//!
//! Simulates a fleet of edge sensors (temperature, humidity, motion) that
//! collect data independently and sync to a central dashboard when connectivity
//! returns. Demonstrates every CRDT type in a realistic IoT scenario.
//!
//! Run: `cargo run --example iot_dashboard`

use crdt_kit::clock::{HybridClock, HybridTimestamp};
use crdt_kit::prelude::*;

fn main() {
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║   crdt-kit — IoT Sensor Dashboard Demo              ║");
    println!("╚══════════════════════════════════════════════════════╝\n");

    sensor_readings();
    device_registry();
    alert_log();
    config_management();
    firmware_tracking();
    full_dashboard_sync();
}

/// Scenario 1: Aggregate sensor readings from multiple edge nodes.
///
/// Each sensor node counts events independently. GCounter merges them
/// without double-counting, even if sync happens multiple times.
fn sensor_readings() {
    println!("━━━ 1. SENSOR EVENT COUNTERS (GCounter + PNCounter) ━━━\n");

    // Motion sensors at three factory gates
    let mut gate_a = GCounter::new(1);
    let mut gate_b = GCounter::new(2);
    let mut gate_c = GCounter::new(3);

    // Events happen while sensors are offline
    gate_a.increment_by(147);
    gate_b.increment_by(89);
    gate_c.increment_by(203);

    println!("  Gate A detections: {}", gate_a.value());
    println!("  Gate B detections: {}", gate_b.value());
    println!("  Gate C detections: {}", gate_c.value());

    // Dashboard syncs with all gates
    let mut dashboard = GCounter::new(100);
    dashboard.merge(&gate_a);
    dashboard.merge(&gate_b);
    dashboard.merge(&gate_c);

    println!("  ─────────────────────────────────");
    println!("  Dashboard total:   {} detections", dashboard.value());
    assert_eq!(dashboard.value(), 439);

    // Idempotent: re-syncing doesn't double-count
    dashboard.merge(&gate_a);
    dashboard.merge(&gate_b);
    println!("  After re-sync:     {} (idempotent)", dashboard.value());
    assert_eq!(dashboard.value(), 439);

    // Delta sync: only send what's new
    gate_a.increment_by(12); // 12 new events at gate A
    let delta = gate_a.delta(&dashboard);
    dashboard.apply_delta(&delta);
    println!("  After delta sync:  {} (+12 from gate A)", dashboard.value());
    assert_eq!(dashboard.value(), 451);

    // Tank level: items added and consumed
    println!("\n  Water tank level (PNCounter):");
    let mut fill_sensor = PNCounter::new(10);
    let mut drain_sensor = PNCounter::new(11);

    for _ in 0..500 {
        fill_sensor.increment(); // 500L filled
    }
    for _ in 0..320 {
        drain_sensor.decrement(); // 320L drained
    }

    fill_sensor.merge(&drain_sensor);
    println!("  Fill: +500L, Drain: -320L = {}L net", fill_sensor.value());
    assert_eq!(fill_sensor.value(), 180);
    println!();
}

/// Scenario 2: Device registry using add-wins semantics.
///
/// When a device is removed from the fleet but simultaneously re-registers
/// on another gateway, the add wins — preventing accidental de-provisioning.
fn device_registry() {
    println!("━━━ 2. DEVICE REGISTRY (AWMap + ORSet) ━━━\n");

    // Active device set tracked by two gateways
    let mut gw_north = ORSet::new(1);
    let mut gw_south = ORSet::new(2);

    gw_north.insert("sensor-001");
    gw_north.insert("sensor-002");
    gw_north.insert("sensor-003");
    gw_south.insert("sensor-004");
    gw_south.insert("sensor-005");

    println!("  North gateway: {} devices", gw_north.len());
    println!("  South gateway: {} devices", gw_south.len());

    // Admin decommissions sensor-002 from north
    gw_north.remove(&"sensor-002");
    println!("  Admin removes sensor-002 from north");

    // Meanwhile, sensor-002 re-registers on south (e.g. it moved)
    gw_south.insert("sensor-002");
    println!("  sensor-002 re-registers on south (concurrent)");

    gw_north.merge(&gw_south);
    gw_south.merge(&gw_north);
    println!("\n  After sync:");
    println!("  sensor-002 active: {} (add wins!)", gw_north.contains(&"sensor-002"));
    println!("  Total fleet: {} devices", gw_north.len());
    assert!(gw_north.contains(&"sensor-002"));

    // Device metadata using AW-Map
    println!("\n  Device metadata (AWMap):");
    let mut meta_a = AWMap::new(1);
    meta_a.insert("sensor-001", "firmware=2.1,zone=A");
    meta_a.insert("sensor-002", "firmware=2.0,zone=B");

    let mut meta_b = AWMap::new(2);
    meta_b.insert("sensor-001", "firmware=2.2,zone=A"); // concurrent update

    meta_a.merge(&meta_b);
    println!("  sensor-001 meta: {:?}", meta_a.get(&"sensor-001"));
    println!("  sensor-002 meta: {:?}", meta_a.get(&"sensor-002"));
    println!();
}

/// Scenario 3: Append-only alert log using GSet.
///
/// Alerts can never be un-acknowledged — once seen, always tracked.
fn alert_log() {
    println!("━━━ 3. ALERT LOG (GSet) ━━━\n");

    let mut edge_alerts = GSet::new();
    let mut cloud_alerts = GSet::new();

    edge_alerts.insert("2024-01-15T08:30:00 TEMP_HIGH sensor-003");
    edge_alerts.insert("2024-01-15T09:15:00 HUMIDITY_LOW sensor-001");
    edge_alerts.insert("2024-01-15T10:00:00 MOTION_AFTER_HOURS gate-B");

    cloud_alerts.insert("2024-01-15T08:45:00 BATTERY_LOW sensor-005");
    cloud_alerts.insert("2024-01-15T09:15:00 HUMIDITY_LOW sensor-001"); // duplicate

    println!("  Edge alerts:  {}", edge_alerts.len());
    println!("  Cloud alerts: {}", cloud_alerts.len());

    edge_alerts.merge(&cloud_alerts);
    println!("  Merged total: {} unique alerts", edge_alerts.len());
    assert_eq!(edge_alerts.len(), 4); // duplicate deduplicated

    for alert in edge_alerts.iter() {
        println!("    ⚠ {alert}");
    }
    println!();
}

/// Scenario 4: Distributed config management with LWW-Map.
///
/// Multiple admins can update sensor configuration. The latest write wins.
fn config_management() {
    println!("━━━ 4. SENSOR CONFIG (LWWMap) ━━━\n");

    let ts = |ms: u64, node: u16| HybridTimestamp {
        physical: ms,
        logical: 0,
        node_id: node,
    };

    let mut config_admin = LWWMap::new();
    config_admin.insert("sample_rate_ms", 1000u32, ts(100, 1));
    config_admin.insert("threshold_temp_c", 85, ts(100, 1));
    config_admin.insert("threshold_humidity", 30, ts(100, 1));
    config_admin.insert("report_interval_s", 60, ts(100, 1));

    println!("  Admin sets initial config:");
    for (k, v) in config_admin.iter() {
        println!("    {k} = {v}");
    }

    // Field engineer updates sample rate (more recent)
    let mut config_field = LWWMap::new();
    config_field.insert("sample_rate_ms", 500u32, ts(200, 2));
    config_field.insert("threshold_temp_c", 90, ts(200, 2));
    println!("\n  Field engineer updates: sample_rate=500, temp_threshold=90");

    // Admin tries to revert (but with older timestamp — ignored)
    let mut config_stale = LWWMap::new();
    config_stale.insert("sample_rate_ms", 2000u32, ts(50, 3));
    println!("  Stale admin sets sample_rate=2000 (ts=50, will be ignored)");

    config_admin.merge(&config_field);
    config_admin.merge(&config_stale);

    println!("\n  Final config after merge:");
    for (k, v) in config_admin.iter() {
        println!("    {k} = {v}");
    }
    assert_eq!(config_admin.get(&"sample_rate_ms"), Some(&500));
    assert_eq!(config_admin.get(&"threshold_temp_c"), Some(&90));

    // Remove a config key
    config_field.remove(&"report_interval_s", ts(300, 2));
    config_admin.merge(&config_field);
    println!("\n  After removing report_interval_s:");
    println!("    report_interval_s present: {}", config_admin.contains_key(&"report_interval_s"));
    assert!(!config_admin.contains_key(&"report_interval_s"));
    println!();
}

/// Scenario 5: Firmware version tracking with conflict detection.
///
/// MV-Register shows when two gateways have pushed different firmware
/// to the same device — a conflict that operations must resolve.
fn firmware_tracking() {
    println!("━━━ 5. FIRMWARE TRACKING (MVRegister + LWWRegister) ━━━\n");

    let mut gw1_view = MVRegister::new(1);
    let mut gw2_view = MVRegister::new(2);

    gw1_view.set("v2.1.0".to_string());
    println!("  GW1 reports firmware: {:?}", gw1_view.values());

    gw2_view.merge(&gw1_view);
    println!("  GW2 syncs, sees:      {:?}", gw2_view.values());

    // Both gateways push different updates concurrently
    gw1_view.set("v2.2.0-hotfix".to_string());
    gw2_view.set("v2.2.0-stable".to_string());

    println!("\n  [Network partition — concurrent OTA updates]");
    println!("  GW1 pushes: {:?}", gw1_view.values());
    println!("  GW2 pushes: {:?}", gw2_view.values());

    gw1_view.merge(&gw2_view);
    println!("\n  After sync:");
    println!("  Versions:  {:?}", gw1_view.values());
    println!("  Conflict?  {}", gw1_view.is_conflicted());
    assert!(gw1_view.is_conflicted());

    // Ops team resolves
    gw1_view.set("v2.2.1-unified".to_string());
    println!("\n  Ops resolves to: {:?}", gw1_view.values());
    println!("  Conflict?  {}", gw1_view.is_conflicted());
    assert!(!gw1_view.is_conflicted());

    // LWW for auto-resolved last-seen timestamp
    println!("\n  Last heartbeat (LWWRegister):");
    let mut clock1 = HybridClock::new(1);
    let mut clock2 = HybridClock::new(2);

    let mut heartbeat1 = LWWRegister::new("2024-01-15T10:30:00Z", &mut clock1);
    let heartbeat2 = LWWRegister::new("2024-01-15T10:31:00Z", &mut clock2);

    heartbeat1.merge(&heartbeat2);
    println!("  Latest heartbeat: {}", heartbeat1.value());
    println!();
}

/// Scenario 6: Full dashboard sync bringing all CRDTs together.
///
/// Simulates two edge nodes collecting data offline, then syncing
/// to a central dashboard using delta-state transfer.
fn full_dashboard_sync() {
    println!("━━━ 6. FULL DASHBOARD SYNC (Delta State) ━━━\n");

    // --- Edge Node 1 collects data ---
    let mut edge1_events = GCounter::new(1);
    edge1_events.increment_by(1200);

    let mut edge1_notes = TextCrdt::new(1);
    edge1_notes.insert_str(0, "Zone A: all nominal").unwrap();

    // --- Edge Node 2 collects data ---
    let mut edge2_events = GCounter::new(2);
    edge2_events.increment_by(890);

    let mut edge2_notes = TextCrdt::new(2);
    edge2_notes.insert_str(0, "Zone B: sensor-003 replaced").unwrap();

    // --- Central dashboard (stale state) ---
    let mut central_events = GCounter::new(100);
    let mut central_notes = TextCrdt::new(100);

    println!("  Edge 1: {} events, notes=\"{}\"", edge1_events.value(), edge1_notes);
    println!("  Edge 2: {} events, notes=\"{}\"", edge2_events.value(), edge2_notes);
    println!("  Central: {} events (stale)", central_events.value());

    // Delta sync: send only what central doesn't have
    let delta1 = edge1_events.delta(&central_events);
    let delta2 = edge2_events.delta(&central_events);
    central_events.apply_delta(&delta1);
    central_events.apply_delta(&delta2);

    // Full merge for text (first sync, no shared state)
    central_notes.merge(&edge1_notes);
    central_notes.merge(&edge2_notes);

    println!("\n  ─── After delta sync ───");
    println!("  Central events: {}", central_events.value());
    println!("  Central notes:  \"{}\"", central_notes);
    assert_eq!(central_events.value(), 2090);

    // Subsequent sync: edge1 collects 50 more events
    edge1_events.increment_by(50);
    let delta_incremental = edge1_events.delta(&central_events);
    central_events.apply_delta(&delta_incremental);
    println!("\n  Edge 1 adds 50 more events...");
    println!("  Central after incremental delta: {}", central_events.value());
    assert_eq!(central_events.value(), 2140);

    // RGA: ordered event timeline
    println!("\n  Event timeline (Rga):");
    let mut timeline = Rga::new(1);
    timeline.insert_at(0, "08:30 TEMP_HIGH").unwrap();
    timeline.insert_at(1, "09:15 HUMIDITY_LOW").unwrap();
    timeline.insert_at(2, "10:00 MOTION_DETECT").unwrap();

    let mut remote_timeline = Rga::new(2);
    remote_timeline.insert_at(0, "08:45 BATTERY_LOW").unwrap();
    remote_timeline.insert_at(1, "09:30 RECONNECTED").unwrap();

    timeline.merge(&remote_timeline);
    println!("  Merged timeline ({} events):", timeline.len());
    for (i, event) in timeline.iter().enumerate() {
        println!("    [{i}] {event}");
    }

    println!("\n  ✓ All CRDTs synced. Dashboard is consistent across all nodes.");
    println!();
}
