//! Example: Collaborative offline todo list using OR-Set.

use crdt_kit::prelude::*;

fn main() {
    println!("=== Collaborative Todo List (OR-Set) ===\n");

    let mut alice = ORSet::new(1);
    let mut bob = ORSet::new(2);

    alice.insert("Buy groceries");
    alice.insert("Walk the dog");
    alice.insert("Write report");
    println!("Alice's list:");
    for item in alice.iter() {
        println!("  - {item}");
    }

    bob.insert("Fix bike");
    bob.insert("Buy groceries");
    println!("\nBob's list:");
    for item in bob.iter() {
        println!("  - {item}");
    }

    alice.merge(&bob);
    bob.merge(&alice);

    println!("\n--- After sync ---");
    println!("Shared list ({} items):", alice.len());
    for item in alice.iter() {
        println!("  - {item}");
    }

    alice.remove(&"Buy groceries");
    println!("\nAlice completed 'Buy groceries'");

    bob.insert("Buy groceries");

    alice.merge(&bob);
    println!("\nAfter sync (add wins):");
    println!(
        "'Buy groceries' present: {}",
        alice.contains(&"Buy groceries")
    );
    println!("Total items: {}", alice.len());
}
