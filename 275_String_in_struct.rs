// p. 275, String use in a struct

struct Person {
    name: String,
    age: i32,
}
impl Person {
    fn say_info(&self) {
        println!("{} is {} years old", self.name, self.age);
    }
}
fn main() {
    let snoyman: &str = " Snoyman";
    let michael = Person {
        name: "Michael".to_owned() + snoyman,
        age: 35,
    };
    let alice = Person {
        name: "Alice Smith".to_owned(),
        age: 30,
    };
    let bob = Person {
        name: "Bob Jhonson".to_owned(),
        age: 25,
    }
    michael.say_info();
    alice.say_info();
    bob.say_info();
}

