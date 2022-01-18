// p. 437, ex. 1, iterator, factorial

fn fact(x: u32) -> u32 {
    let mut result: u32 = 1;
    for i in 1..=x {
        result *= i;
    }
    result
}
fn main() {
    assert_eq!(fact(0), 1);
    assert_eq!(fact(1), 1);
    assert_eq!(fact(2), 2);
    assert_eq!(fact(3), 6);
    assert_eq!(fact(4), 24);
    assert_eq!(fact(5), 120);
    println!("Success!");
}
