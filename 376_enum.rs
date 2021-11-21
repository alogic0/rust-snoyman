// p. 376

#[derive(PartialEq, Eq)]
enum Job {
    Teacher,
    Scientist,
}
struct Person {
    name: String,
    age: u32,
    job: Job,
}
impl Person {
    fn same_job(&self, other: &Person) -> bool {
        self.job == other.job
    }
}
fn main() {
    let alice = Person {
        name: "Alice".to_owned(),
        age: 30,
        job: Job::Scientist,
    };
    let bob = Person {
        name: "Bob".to_owned(),
        age: 35,
        job: Job::Scientist,
    };
    let charlie = Person {
        name: "Charlie".to_owned(),
        age: 40,
        job: Job::Teacher,
    };
    assert!(alice.same_job(&bob));
    assert!(!alice.same_job(&charlie));
    println!("Success!");
}

