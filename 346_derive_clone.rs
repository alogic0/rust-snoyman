// p. 346, derive Clone

use std::fmt::Display;

#[derive(Clone)]
struct Fruit<T> {
    apples: T,
    bananas: T,
}
// Should use a reference, but I'm proving a point
fn print_fruit<T: Display>(fruit: Fruit<T>) {
    println!("Apples: {}, bananas: {}", fruit.apples, fruit.bananas);
}
fn main() {
    let mut fruit = Fruit {
        apples: 5,
        bananas: 10,
    };
    print_fruit(fruit.clone());
    fruit.apples *= 2;
    fruit.bananas *= 3;
    print_fruit(fruit);
}

