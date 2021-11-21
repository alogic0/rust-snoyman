// p. 277

fn make_full_name(first: &str, last: &str) -> String {
    format!("{} {}", first, last)
}
fn main() {
    assert_eq!(&make_full_name("Michael", "Snoyman"), "Michael Snoyman");
    assert_eq!(
        &make_full_name("Alice", "Smith"),
        "Alice Smith",
    );
    println!("Success!");
}

