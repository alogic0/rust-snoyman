// p. 149

fn main() {
    let mut counter: u32 = 0;
    while counter < 4 {
        let times = if counter == 1 {
            "time"
            } else {
            "times"
            };
        println!("Inside the while loop, already looped {} {}.",
            counter,
            times);
        counter += 1;
    }
}
