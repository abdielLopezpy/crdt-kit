//! Example: Distributed e-commerce system with real business entities.
//!
//! Demonstrates how CRDTs solve real-world problems in a multi-store
//! e-commerce platform where devices go offline and sync later.
//!
//! Run: `cargo run --example ecommerce`

use crdt_kit::prelude::*;

fn main() {
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║   crdt-kit — Distributed E-Commerce Demo            ║");
    println!("╚══════════════════════════════════════════════════════╝\n");

    inventory_sync();
    shopping_cart();
    product_pricing();
    collaborative_description();
    analytics_delta_sync();
    order_tracking();
}

/// Scenario 1: Multi-store inventory that stays consistent even offline.
fn inventory_sync() {
    println!("━━━ 1. DISTRIBUTED INVENTORY ━━━\n");

    let mut warehouse = PNCounter::new(1);
    let mut store_nyc = PNCounter::new(2);
    let mut store_la = PNCounter::new(3);

    for _ in 0..100 {
        warehouse.increment();
    }
    println!("  Warehouse shipped:   +100 units");

    for _ in 0..35 {
        store_nyc.decrement();
    }
    println!("  NYC store sold:       -35 units (offline)");

    for _ in 0..22 {
        store_la.decrement();
    }
    println!("  LA store sold:        -22 units (offline)");

    warehouse.merge(&store_nyc);
    warehouse.merge(&store_la);

    println!("  ─────────────────────────────");
    println!(
        "  After sync, total stock: {} units  (100 - 35 - 22 = 43)",
        warehouse.value()
    );
    assert_eq!(warehouse.value(), 43);

    let mut reverse = store_la.clone();
    reverse.merge(&store_nyc);
    reverse.merge(&warehouse);
    println!(
        "  Reverse merge:           {} units  (commutativity verified)",
        reverse.value()
    );
    assert_eq!(warehouse.value(), reverse.value());
    println!();
}

/// Scenario 2: Shopping cart that handles concurrent add/remove across devices.
fn shopping_cart() {
    println!("━━━ 2. SHOPPING CART (OR-Set) ━━━\n");

    let mut phone = ORSet::new(1);
    let mut laptop = ORSet::new(2);

    phone.insert("Wireless Earbuds - $49.99");
    phone.insert("Phone Case - $19.99");
    phone.insert("USB-C Cable - $9.99");
    println!("  Phone adds:  Wireless Earbuds, Phone Case, USB-C Cable");

    laptop.insert("Mechanical Keyboard - $89.99");
    laptop.insert("Mouse Pad - $14.99");
    println!("  Laptop adds: Mechanical Keyboard, Mouse Pad");

    phone.remove(&"USB-C Cable - $9.99");
    println!("  Phone removes: USB-C Cable");

    laptop.insert("USB-C Cable - $9.99");
    println!("  Laptop re-adds: USB-C Cable (concurrent with remove)");

    phone.merge(&laptop);
    laptop.merge(&phone);

    println!("\n  Cart after sync ({} items):", phone.len());
    for item in phone.iter() {
        println!("    - {item}");
    }
    println!("  USB-C Cable survived! (add wins over concurrent remove)");
    assert!(phone.contains(&"USB-C Cable - $9.99"));
    assert_eq!(phone.len(), 5);
    println!();
}

/// Scenario 3: Product pricing with automatic conflict resolution.
fn product_pricing() {
    use crdt_kit::clock::HybridTimestamp;

    println!("━━━ 3. PRODUCT PRICING (LWW-Register) ━━━\n");

    let ts = |ms: u64, node: u16| HybridTimestamp {
        physical: ms,
        logical: 0,
        node_id: node,
    };

    let mut price = LWWRegister::with_timestamp("USD 79.99", ts(900, 1));
    println!("  09:00 Admin sets price:     {}", price.value());

    let flash_sale = LWWRegister::with_timestamp("USD 59.99 (SALE!)", ts(1000, 2));
    price.merge(&flash_sale);
    println!("  10:00 Marketing flash sale: {}", price.value());

    let algo = LWWRegister::with_timestamp("USD 64.99", ts(1100, 3));
    price.merge(&algo);
    println!("  11:00 Algorithm adjusts:    {}", price.value());

    let stale = LWWRegister::with_timestamp("USD 99.99", ts(800, 4));
    price.merge(&stale);
    println!("  08:00 Stale admin override: ignored (older timestamp)");
    println!("  Final price:                {}", price.value());
    assert_eq!(*price.value(), "USD 64.99");
    println!();
}

/// Scenario 4: Collaborative product description editing.
fn collaborative_description() {
    println!("━━━ 4. COLLABORATIVE DESCRIPTION (TextCrdt) ━━━\n");

    let mut pm = TextCrdt::new(1);
    pm.insert_str(0, "Ergonomic office chair").unwrap();
    println!("  PM writes:        \"{}\"", pm);

    let mut designer = pm.fork(2);
    let mut copywriter = pm.fork(3);

    let len = designer.len();
    designer.insert_str(len, " with lumbar support").unwrap();
    println!("  Designer adds:    \"{}\"", designer);

    copywriter.insert_str(0, "[BESTSELLER] ").unwrap();
    println!("  Copywriter adds:  \"{}\"", copywriter);

    pm.merge(&designer);
    pm.merge(&copywriter);
    println!("\n  Merged result:    \"{}\"", pm);
    println!("  (All concurrent edits converge deterministically)");
    println!();
}

/// Scenario 5: Analytics with efficient delta sync.
fn analytics_delta_sync() {
    println!("━━━ 5. ANALYTICS WITH DELTA SYNC (DeltaCrdt) ━━━\n");

    let mut edge_us = GCounter::new(1);
    let mut edge_eu = GCounter::new(2);
    let mut central = GCounter::new(3);

    edge_us.increment_by(5000);
    edge_eu.increment_by(3200);

    println!("  US edge node:  {} views", edge_us.value());
    println!("  EU edge node:  {} views", edge_eu.value());
    println!("  Central (old): {} views", central.value());

    let delta_us = edge_us.delta(&central);
    let delta_eu = edge_eu.delta(&central);

    central.apply_delta(&delta_us);
    central.apply_delta(&delta_eu);

    println!("\n  After delta sync:");
    println!("  Central total: {} views", central.value());
    assert_eq!(central.value(), 8200);
    println!();
}

/// Scenario 6: Order status tracking with MV-Register (conflict detection).
fn order_tracking() {
    println!("━━━ 6. ORDER STATUS TRACKING (MV-Register) ━━━\n");

    let mut warehouse_view = MVRegister::new(1);
    let mut shipping_view = MVRegister::new(2);

    warehouse_view.set("packed".to_string());
    println!("  Warehouse sets: {:?}", warehouse_view.values());

    shipping_view.merge(&warehouse_view);
    println!("  Shipping sees:  {:?}", shipping_view.values());
    println!("  Conflict?       {}", shipping_view.is_conflicted());

    warehouse_view.set("ready-for-pickup".to_string());
    shipping_view.set("in-transit".to_string());

    println!("\n  [Network partition — concurrent updates]");
    println!("  Warehouse sets: {:?}", warehouse_view.values());
    println!("  Shipping sets:  {:?}", shipping_view.values());

    warehouse_view.merge(&shipping_view);
    println!("\n  After sync:");
    println!("  Values:   {:?}", warehouse_view.values());
    println!(
        "  Conflict? {}  (app can show alert to ops team)",
        warehouse_view.is_conflicted()
    );

    warehouse_view.set("in-transit".to_string());
    println!("\n  Ops resolves to: {:?}", warehouse_view.values());
    println!("  Conflict? {}", warehouse_view.is_conflicted());
    println!();
}
