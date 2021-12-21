// p. 370, ex. 4, type parameters, bounds

use std::fmt::Display;

fn stringify<T>(x: T) -> String
where
    T: Display,
{
    format!("{}", x)
}

fn main() {
    assert_eq!(stringify(5), "5".to_owned());
    assert_eq!(stringify(true), "true".to_owned());
    assert_eq!(stringify("Hello"), "Hello".to_owned());
    println!("Success!");
}
