//! Example: Distributed counter across three devices.

use crdt_kit::prelude::*;

fn main() {
    println!("=== G-Counter Example ===\n");

    let mut device_a = GCounter::new(1);
    let mut device_b = GCounter::new(2);
    let mut device_c = GCounter::new(3);

    device_a.increment_by(5);
    device_b.increment_by(3);
    device_c.increment_by(8);

    println!("Phone views:  {}", device_a.value());
    println!("Tablet views: {}", device_b.value());
    println!("Laptop views: {}", device_c.value());

    device_a.merge(&device_b);
    device_a.merge(&device_c);

    println!("\nAfter sync, total views: {}", device_a.value());

    println!("\n=== PN-Counter Example ===\n");

    let mut warehouse = PNCounter::new(1);
    let mut store = PNCounter::new(2);

    warehouse.increment();
    warehouse.increment();
    warehouse.increment();
    println!("Warehouse added 3 items: {}", warehouse.value());

    store.decrement();
    store.decrement();
    println!("Store sold 2 items: {}", store.value());

    warehouse.merge(&store);
    println!("After sync, net stock change: {}", warehouse.value());
}
