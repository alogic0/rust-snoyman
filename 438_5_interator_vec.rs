// p. 438, ex.5, iterator, vec, borrow

fn average(scores: &Vec<usize>) -> usize {
    let mut result: usize = 0;
    for i in scores {
        result += i;
    }
    result / scores.len()
}
fn main() {
    let physics = vec![53, 64, 73, 36, 96, 100, 45, 81, 88, 64];
    let biology = vec![68, 29, 87, 34, 89, 95];
    let chemistry = vec![100, 81, 68, 60, 86, 79, 88, 94];
    assert_eq!(average(&physics), 70);
    println!(
        "The average score on the physics exam was {}.",
        average(&physics)
    );
    assert_eq!(average(&biology), 67);
    println!(
        "The average score on the biology exam was {}.",
        average(&biology)
    );
    assert_eq!(average(&chemistry), 82);
    println!(
        "The average score on the chemistry exam was
{}.",
        average(&chemistry)
    );
}
