use std::ops::{Add, Mul, Sub};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
enum Peano {
    O,              // Zero is natural number.
    S(Rc<Peano>),   // Successor of a natural number is a natural number.
}
impl From<usize> for Peano {
    fn from(value: usize) -> Self {
        match value {
            0 => Peano::O,
            _ => Peano::S(Rc::new(Peano::from(value - 1))),
        }
    }
}
impl Into<usize> for Peano {
    fn into(self) -> usize {
        match self {
            Peano::O => 0,
            Peano::S(n) => {
                let num : usize = (*n).clone().into();
                1 + num
            }
        }
    }
}

impl Peano {
    fn pred(self) -> Self {
        match self {
            Peano::O => Peano::O,
            Peano::S(n) => (*n).clone(),
        }
    }
    fn succ(self) -> Self {
        Peano::S(Rc::new(self))
    }
}
impl Add for Peano {
    type Output = Peano;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Peano::O, Peano::O) => Peano::O,
            (Peano::O, rhs) => rhs,
            (lhs, Peano::O) => lhs,
            (lhs, rhs) => lhs.succ() + rhs.pred(),
        }
    }
}

impl Sub for Peano {
    type Output = Peano;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Peano::O, _) => Peano::O,
            (lhs, Peano::O) => lhs,
            (lhs, rhs) => lhs.pred() - rhs.pred(),
        }
    }
}

impl Mul for Peano {
    type Output = Peano;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Peano::O, _) | (_, Peano::O) => Peano::O,
            (lhs, rhs) => lhs.clone() + (lhs.clone() * rhs.pred()),
        }
    }
}

mod test_peano {
    use super::*;

    pub fn check_one_add_one() {
        let peano_1: Peano = 1.into();
        let peano_2: Peano = 2.into();
        assert_eq!(peano_1.clone() + peano_1, peano_2);
    }

    pub fn check_three_minus_one() {
        let peano_1: Peano = 1.into();
        let peano_2: Peano = 2.into();
        let peano_3: Peano = 3.into();
        assert_eq!(peano_3 - peano_1, peano_2);
    }

    pub fn check_three_mult_four() {
        let peano_3: Peano = 3.into();
        let peano_4: Peano = 4.into();
        let peano_12: Peano = 12.into();
        assert_eq!(peano_3 * peano_4, peano_12);
    }
}

fn main() {
    test_peano::check_one_add_one();
    test_peano::check_three_minus_one();
    test_peano::check_three_mult_four();
}