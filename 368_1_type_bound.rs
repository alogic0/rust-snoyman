// p. 368, ex. 1, type bound

use std::fmt::Display;

struct Person<Name, Age> {
    name: Name,
    age: Age,
}

fn greet<Age>(person: &Person<String, Age>)
where
    Age: Display,
{
    println!("Hello, {}, you are {} years old.", person.name, person.age);
}

fn main() {
    let alice: Person<String, u32> = Person {
        name: "Alice".to_owned(),
        age: 30_u32,
    };
    greet(&alice);
    let bob = Person {
        name: "Bob".to_owned(),
        age: 35_u64,
    };
    greet(&bob);
}

