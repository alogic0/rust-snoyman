// p. 273, example String use

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
    michael.say_info();
    alice.say_info();
}

