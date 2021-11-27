// p. 282, Strings, ex. 1 

fn main() {
    let mut hello = "Hello, ".to_owned();
    hello += "world!";
    println!("{}", hello);
}

