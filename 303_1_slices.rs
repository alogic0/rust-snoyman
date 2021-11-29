// p. 303, ex. 1, slices

fn maximum(numbers: &[u32]) -> u32 {
    let mut max: u32 = 0;
    let mut i = 0;
    while i < numbers.len() {
        if numbers[i] > max {
            max = numbers[i];
        }
        i += 1;
    }
    max
}
fn main() {
    assert_eq!(maximum(&[1, 2, 3]), 3);
    assert_eq!(maximum(&[1, 8, 3]), 8);
    assert_eq!(maximum(&[1, 1, 1]), 1);
    assert_eq!(maximum(&[]), 0);
    println!("Success!");
}

