// p.322, ex.2, just_second

fn just_second<S, T>(_inp1: S, inp2: T) -> T {
    inp2
}

fn main() {
    assert_eq!(just_second((), "foo"), "foo");
    assert_eq!(just_second(true, 5_i32), 5);
    assert_eq!(just_second("ignored", 6_i64), 6);
    assert_eq!(just_second(5_u8, true), true);
    println!("Success!");
}
