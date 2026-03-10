//! Example: Simple P2P chat with conflict detection using MV-Register.

use crdt_kit::clock::HybridClock;
use crdt_kit::prelude::*;

fn main() {
    println!("=== Chat Status with Conflict Detection (MV-Register) ===\n");

    let mut alice_view = MVRegister::new(1);
    let mut bob_view = MVRegister::new(2);

    alice_view.set("Project kickoff meeting".to_string());
    println!("Alice sets topic: {:?}", alice_view.values());

    bob_view.merge(&alice_view);
    println!("Bob sees topic:   {:?}", bob_view.values());

    alice_view.set("Sprint planning".to_string());
    bob_view.set("Design review".to_string());

    println!("\n--- Concurrent updates ---");
    println!("Alice's topic: {:?}", alice_view.values());
    println!("Bob's topic:   {:?}", bob_view.values());

    alice_view.merge(&bob_view);
    println!("\n--- After sync ---");
    println!("Values: {:?}", alice_view.values());
    println!("Conflict detected: {}", alice_view.is_conflicted());

    alice_view.set("Sprint planning + Design review".to_string());
    println!("\n--- Alice resolves conflict ---");
    println!("Topic: {:?}", alice_view.values());
    println!("Conflict: {}", alice_view.is_conflicted());

    println!("\n=== LWW-Register (auto-resolve by HLC timestamp) ===\n");

    let mut clock1 = HybridClock::new(1);
    let mut clock2 = HybridClock::new(2);

    let mut r1 = LWWRegister::new("value-a", &mut clock1);
    let r2 = LWWRegister::new("value-b", &mut clock2);

    println!("Node 1: {:?}", r1.value());
    println!("Node 2: {:?}", r2.value());

    r1.merge(&r2);
    println!("After merge: {:?} (latest HLC timestamp wins)", r1.value());
}
