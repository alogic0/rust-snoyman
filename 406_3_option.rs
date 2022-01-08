// p. 406, ex. 3, Option

fn minus(x: u32, y: u32) -> Option<u32> {
    if x < y {
        None
    } else {
        Some(x - y)
    }
}

fn main() {
    assert_eq!(minus(5, 2), Some(3));
    assert_eq!(minus(3, 2), Some(1));
    assert_eq!(minus(2, 2), Some(0));
    assert_eq!(minus(1, 2), None);
    println!("Success!");
}

