// p. 303, ex. 3, vector slices

fn fibs(count: usize) -> Vec<u32> {
    let mut fib_vec: Vec<u32> = vec![1, 1];
    let mut i = 2;
    while i < count {
        let fib_next = fib_vec[i - 1] + fib_vec[i - 2];
        fib_vec.push(fib_next);
        i += 1;
    }
    fib_vec[..count].to_vec()
}
fn main() {
    assert_eq!(fibs(5), &[1, 1, 2, 3, 5]);
    assert_eq!(fibs(1), &[1]);
    assert_eq!(fibs(8), &[1, 1, 2, 3, 5, 8, 13, 21]);
    println!("Success!");
}

