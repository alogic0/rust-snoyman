// p. 293

fn sum(numbers: &[i32]) -> i32 {
    let mut total = 0;
    let mut i = 0;
    while i < numbers.len() {
        total += numbers[i];
        i += 1
    }
    total
}
fn main() {
    let numbers: &[i32] = &[2, 3, 8, 1, 9, 4];
    assert_eq!(27, sum(&numbers));
    println!("Good job!");
}
