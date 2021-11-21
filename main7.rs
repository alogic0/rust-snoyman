fn main() {
    let hello = "Hello, ";
    let world = "world!";
    let helloworld: String = hello.to_owned() + world;
    println!("{}", helloworld);
}
