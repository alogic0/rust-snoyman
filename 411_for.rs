// p.411, for loop

fn main() {
    let mut total = 0;
    for i in 1..=10 {
        total += i;
    }
    assert_eq!(total, 55);
    println!("Success!");
}
