// p. 435, Iterator, Item parametrized type

fn sum<I: Iterator<Item = u32>>(vals: I) -> u32 {
    let mut total = 0;
    for val in vals {
        total += val;
    }
    total
}

fn main() {
    assert_eq!(sum(vec![1, 2, 3, 4].into_iter()), 10);
    assert_eq!(sum(1..=4), 10);
    println!("Success!");
}
