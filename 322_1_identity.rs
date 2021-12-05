// p.322, ex 1, identity

fn identity<T>(inp: T) -> T {
    inp
}

fn main() {
    assert_eq!(identity("foo"), "foo");
    assert_eq!(identity(5_i32), 5);
    assert_eq!(identity(6_i64), 6);
    assert_eq!(identity(true), true);
    println!("Success!");
}

