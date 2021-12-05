// p. 341, Display trait

use std::fmt::Display;

fn stringy<T: Display>(x: T) -> String {
    format!("{}", x)
}

fn main() {
    assert_eq!(stringy(42), "42".to_owned());
    assert_eq!(stringy(true), "true".to_owned());
    assert_eq!(stringy("hello"), "hello".to_owned());
    println!("Success!");
}
