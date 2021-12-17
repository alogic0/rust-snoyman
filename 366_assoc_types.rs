// p. 366, Associated types

#[derive(Debug, Clone, Copy)]
struct Kmh {
    value: u32,
}

#[derive(Debug, Clone, Copy)]
struct Km {
    value: u32,
}

impl Km {
    fn in_miles(&self) -> Miles {
        Miles {
            value : self.value * 5/9,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Mph {
    value: u32,
}

#[derive(Debug, Clone, Copy)]
struct Miles {
    value: u32,
}

impl Miles {
    fn in_km(&self) -> Km {
        Km {
            value : self.value * 9/5,
        }
    }
}

trait InThreeHours {
    type Distance;
    fn in_three_hours(&self) -> Self::Distance;
}

impl InThreeHours for Kmh {
    type Distance = Km;
    fn in_three_hours(&self) -> Self::Distance {
        Self::Distance {
            value: self.value * 3,
        }
    }
}

impl InThreeHours for Mph {
    type Distance = Miles;
    fn in_three_hours(&self) -> Self::Distance {
        Self::Distance {
            value: self.value * 3,
        }
    }
}

fn main() {
    let speed = Kmh { value: 90 };
    let distance = speed.in_three_hours();
    println!("At {:?}, you will travel {:?} in 3 hours, or {:?}.", speed, distance, distance.in_miles());
    let speed_m = Mph { value: 90 };
    let distance_m = speed_m.in_three_hours();
    println!("At {:?}, you will travel {:?} in 3 hours, or {:?}.", speed_m, distance_m, distance_m.in_km());
}

