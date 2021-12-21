// p. 371_5, Double trait

trait Double {
    fn double(&self) -> Self;
}

impl Double for i32 {
    fn double(&self) -> Self {
        self * 2
    }
}

impl Double for i64 {
    fn double(&self) -> Self {
        self * 2
    }
}

fn main() {
    assert_eq!(10, 5_i32.double());
    assert_eq!(10, 5_i64.double());
    println!("Success!");
}
