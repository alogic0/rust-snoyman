// p. 282, Strings, ex. 1

fn main() {
    let hello = "Hello, ";
    let world = "world!";
    let helloworld = hello.to_owned() + world;
    println!("{}", helloworld);
}
