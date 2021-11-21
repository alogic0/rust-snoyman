struct RunningTotal {
    total: i32,
    count: i32,
}

fn new_running_total() -> RunningTotal {
    RunningTotal { total: 0, count: 0 }
}

fn print_average(rt: &RunningTotal) {
    let avrg = if rt.count == 0 {
        0
    } else {
        rt.total / rt.count
    };
    println!("Average: {}", avrg);
}

fn add_value(rt: &mut RunningTotal, value: i32) {
    rt.total += value;
    rt.count += 1;
}
fn main() {
    let mut rt = new_running_total();
    print_average(&rt);
    add_value(&mut rt, 5);
    add_value(&mut rt, 3);
    print_average(&rt);
    add_value(&mut rt, 10);
    print_average(&rt);
}
