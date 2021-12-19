// p. 368, ex. 2, type bound

use std::fmt::Display;

struct Person<Name, Age> {
    name: Name,
    age: Age,
}

fn greet<Name, Age>(person: &Person<Name, Age>)
where
    Name: Display,
    Age: Display,
{
    println!("Hello, {}, you are {} years old.", person.name, person.age);
}

fn main() {
    let alice = Person {
        name: "Alice".to_owned(),
        age: 30_u32,
    };
    greet(&alice);
    let bob = Person {
        name: "Bob",
        age: 35_u64,
    };
    greet(&bob);
}

