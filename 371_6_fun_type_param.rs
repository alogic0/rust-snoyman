// p. 371, ex. 6, function type parameter

fn reverse<T>(numbers: &[T]) -> Vec<T>
where
    T: Copy,
{
    let mut res = vec![];
    let mut i = numbers.len();
    while i > 0 {
        i -= 1;
        res.push(numbers[i]);
    }
    res
}

fn main() {
    assert_eq!(reverse(&[4, 5, 2, 8]), &[8, 2, 5, 4]);
    assert_eq!(
        reverse(&[true, false, true, false]),
        &[false, true, false, true]
    );
    assert_eq!(reverse(&[(), ()]), &[(), ()]);
    println!("Success!");
}
