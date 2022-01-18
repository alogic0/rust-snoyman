// p. 437, ex. 2, iterator, print border

fn print_border(nraw: u32, ncol: u32) -> () {
    for _ in 1..=ncol {
        print!("*");
    }
    println!("");
    for _ in 2..nraw {
        print!("*");
        for _ in 2..ncol {
            print!(" ");
        }
        println!("*");
    }
    for _ in 1..=ncol {
        print!("*");
    }
    println!("");
}

fn main() {
    print_border(6, 5);
}
