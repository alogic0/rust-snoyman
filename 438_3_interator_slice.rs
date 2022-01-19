// p. 438, ex.3, iterator, slice

fn print_vertical(in_str: &str) -> () {
    //let chars = in_str.as_chars();
    let in_len = in_str.len();
    for (i, item) in in_str.chars().enumerate() {
        println!("{} = {} out of {}", item, i + 1, in_len);
    }
}

fn main() {
    print_vertical("Alice");
}
