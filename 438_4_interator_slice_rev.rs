// p. 438, ex.4, iterator, slice, reverse order

fn vertical_backwards_name(in_str: &str) -> () {
    for item in in_str.chars().rev() {
        println!("{}", item);
    }
}

fn main() {
    vertical_backwards_name("Alice");
}
