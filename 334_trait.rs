// p. 334

trait DoubleString {
    fn double_string(&self) -> String;
}
impl DoubleString for i32 {
    fn double_string(&self) -> String {
        format!("{}", self * 2)
    }
}
impl DoubleString for i64 {
    fn double_string(&self) -> String {
        format!("{}", self * 2)
    }
}
fn print_double<T: DoubleString>(x: T) {
    println!("The doubled value is {}", x.double_string());
}
fn main() {
    print_double(5_i32);
    print_double(20_i64);
}

