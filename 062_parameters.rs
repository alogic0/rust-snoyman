// p. 62

fn say_apples(apples: i32) {
    println!("I have {} apples.", apples);
}

fn say_ate() {
    println!("I ate an apple.");
}

fn main() {
    let apples = 10;
    say_apples(apples);
    say_ate();
    say_apples(apples - 1);
    say_ate();
    say_apples(apples - 2);
    say_ate();
    say_apples(apples - 3);
}
