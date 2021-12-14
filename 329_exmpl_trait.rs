// p. 329, trait example

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
    println!("double 5_i32 == {}", 5_i32.double());
    println!("double 5_i64 == {}", 5_i64.double());
}

