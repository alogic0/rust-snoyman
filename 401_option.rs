// p. 401, Optiion

enum Monster {
    LochNess,
    Dracula,
    Bigfoot,
    Alien,
}

#[derive(PartialEq, Eq, Debug)]
enum Food {
    Blood,
    Cows,
}

impl Monster {
    fn eats(&self) -> Option<Food> {
        use Food::*;
        use Monster::*;

        match self {
            LochNess => None,
            Dracula => Some(Blood),
            Bigfoot => None,
            Alien => Some(Cows),
        }
    }
}

fn main() {
    use Food::*;
    use Monster::*;
    assert_eq!(Dracula.eats(), Some(Blood));
    assert_eq!(Alien.eats(), Some(Cows));
    assert_eq!(Bigfoot.eats(), None);
    assert_eq!(LochNess.eats(), None);
}
