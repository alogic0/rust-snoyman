// p. 345, clone()

struct Fruit {
    apples: i32,
    bananas: i32,
}

// We can implement a clone method on Fruit:
impl Fruit {
    fn clone(&self) -> Fruit {
        Fruit {
            apples: self.apples.clone(),
            bananas: self.bananas.clone(),
        }
    }
}
// Should use a reference, but I'm proving a point
fn print_fruit(fruit: Fruit) {
    println!("Apples: {}, bananas: {}", fruit.apples, fruit.bananas);
}
fn main() {
    let mut fruit = Fruit {
        apples: 5,
        bananas: 10,
    };
    print_fruit(fruit.clone()); // cloned here
    fruit.apples *= 2; // this works now
    fruit.bananas *= 3;
    print_fruit(fruit);
}

