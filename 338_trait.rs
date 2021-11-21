// p. 338

trait Double {
    fn double(&self) -> Self;
}

impl Double for i32 {
    fn double(&self) -> Self {
        self * 2
    }
}
impl Double for i64 {
    fn double(&self) -> Self {
        self * 2
    }
}

struct Fruit<T> {
    apples: T,
    bananas: T,
}

impl<T: Double> Double for Fruit<T> {
    fn double(&self) -> Self {
        Fruit {
            apples: self.apples.double(),
            bananas: self.bananas.double(),
        }
    }
}

fn main() {
    let fruit = Fruit {
        apples: 5_i32,
        bananas: 10_i32,
    };
    
    let double_fruit = fruit.double();
    
    println!("apples: {}, bananas: {}", fruit.apples, fruit.bananas);
    println!("apples now: {}, bananas: {}", double_fruit.apples, double_fruit.bananas);

}
