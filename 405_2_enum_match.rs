// p. 405, ex. 2, enum, match

enum Science {
    Biology,
    Physics,
}

enum School {
    Elementary,
    Middle,
    High,
}

enum Job {
    Teacher(School),
    Scientist(Science),
    Electrician,
}

impl Job {
    fn salary(&self) -> u32 {
        use Job::*;
        use School::*;
        use Science::*;
        match self {
            Teacher(Elementary) => 100,
            Teacher(Middle) => 150,
            Teacher(High) => 200,
            Scientist(Physics) => 180,
            Scientist(Biology) => 220,
            Electrician => 250,
        }
    }
}

fn main() {
    use Job::*;
    use School::*;
    use Science::*;
    assert_eq!(Teacher(Elementary).salary(), 100);
    assert_eq!(Teacher(Middle).salary(), 150);
    assert_eq!(Teacher(High).salary(), 200);
    assert_eq!(Scientist(Physics).salary(), 180);
    assert_eq!(Scientist(Biology).salary(), 220);
    assert_eq!(Electrician.salary(), 250);
    println!("Success!");
}
