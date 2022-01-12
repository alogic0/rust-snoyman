// p.430, chars() of String

fn main() {
    let name = "Ёлы-палы, йо-хо-хо.";
    for x in name.chars() {
        print!("{}", x);
    }
    println!();
    for x in name.bytes() {
        print!("{} ", x);
    }
    println!();
}
