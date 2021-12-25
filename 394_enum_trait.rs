// p. 394, enum, trait

enum Subject {
    Art,
    Math,
}

enum Job {
    Teacher(Subject),
    Scientist,
}

trait IsTeacher {
    fn is_teacher(&self) -> bool;
}

struct Person {
    name: String,
    age: u32,
    job: Job,
}

impl IsTeacher for Job {
    fn is_teacher(&self) -> bool {
        match self {
            Job::Teacher(_) => true,
            _ => false,
        }
    }
}

impl IsTeacher for Person {
    fn is_teacher(&self) -> bool {
        self.job.is_teacher()
    }
}

fn main() {
    assert!(Job::Teacher(Subject::Math).is_teacher());
    assert!(!Job::Scientist.is_teacher());
    let alice = Person {
        name: "Alice".to_owned(),
        age: 30,
        job: Job::Teacher(Subject::Math),
    };
    assert!(alice.is_teacher());
    println!("Success!");
}
