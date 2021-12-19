// p. 368, ex. 3, mutable parameters and trait

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

trait AddOne {
    fn add_one(&mut self) -> Self;
}

impl AddOne for u32 {
    fn add_one(&mut self) -> Self {
        *self + 1
    }
}

impl AddOne for u64 {
    fn add_one(&mut self) -> Self {
        *self + 1
    }
}

impl<Name, Age: AddOne> Person<Name, Age> {
    fn older(&mut self) {
        self.age = self.age.add_one();
    }
}

fn main() {
    let mut alice: Person<String, u32> = Person {
        name: "Alice".to_owned(),
        age: 30_u32,
    };
    alice.older();
    greet(&alice);
    let mut bob: Person<String, u64> = Person {
        name: "Bob".to_owned(),
        age: 35_u64,
    };
    bob.older();
    greet(&bob);
}
