// p. 303, ex. 2, slices, vectors

fn reverse(numbers: &[i32]) -> Vec<i32> {
    let mut rev_numbers: Vec<i32> = vec![];
    let last_idx = numbers.len() - 1;
    let mut i = 0;
    while i <= last_idx {
        rev_numbers.push(numbers[last_idx - i]);
        i += 1;
    }
    rev_numbers
}
fn main() {
    let numbers = &[4, 5, 2, 8];
    assert_eq!(reverse(numbers), &[8, 2, 5, 4]);
    println!("Success!");
}

