// p. 405, ex. 1, enum matching

#[derive(Debug, PartialEq, Eq)]
enum Size {
    Small,
    Medium,
    Large,
}

fn vehicle_size(seats: u32) -> Size {
    use Size::*;
    match seats {
        0..=3 => Small,
        4..=7 => Medium,
        _ => Large,
    }
}

fn main() {
    use Size::*;
    assert_eq!(Small, vehicle_size(1));
    assert_eq!(Small, vehicle_size(2));
    assert_eq!(Small, vehicle_size(3));
    assert_eq!(Medium, vehicle_size(4));
    assert_eq!(Medium, vehicle_size(5));
    assert_eq!(Medium, vehicle_size(6));
    assert_eq!(Medium, vehicle_size(7));
    assert_eq!(Large, vehicle_size(8));
    assert_eq!(Large, vehicle_size(9));
    assert_eq!(Large, vehicle_size(10));
    println!("Success!");
}
