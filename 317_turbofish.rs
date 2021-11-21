// p.317 Turbofish!

fn main() {
    let size = std::mem::size_of::<[i32; 5]>();
    println!("Size: {}", size);
}
