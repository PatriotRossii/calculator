use rust_decimal::{prelude::ToPrimitive, Decimal};
use std::{convert::TryInto, str::FromStr};

/*pub type Literal = Number;

impl Parse for Literal {
    fn parse(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::num => pair.as_str().parse::<f64>().unwrap(),
            _ => unreachable!(),
        }
    }
}*/

pub trait Literal: Copy {
    fn parse(repr: &str) -> Self;

    fn abs(&self) -> Self;
    fn sqrt(&self) -> Self;
    fn sin(&self) -> Self;
    fn cos(&self) -> Self;
    fn tg(&self) -> Self;
    fn ctg(&self) -> Self;

    fn add(&self, rhs: &Self) -> Self;
    fn sub(&self, rhs: &Self) -> Self;
    fn mul(&self, rhs: &Self) -> Self;
    fn div(&self, rhs: &Self) -> Self;
    fn pow(&self, rhs: &Self) -> Self;
}

impl Literal for f64 {
    fn parse(repr: &str) -> Self {
        repr.parse::<f64>().unwrap()
    }

    fn abs(&self) -> Self {
        f64::abs(*self)
    }

    fn sqrt(&self) -> Self {
        f64::sqrt(*self)
    }

    fn sin(&self) -> Self {
        f64::sin(*self)
    }

    fn cos(&self) -> Self {
        f64::cos(*self)
    }

    fn tg(&self) -> Self {
        self.tan()
    }

    fn ctg(&self) -> Self {
        self.tan().powi(-1)
    }

    fn add(&self, rhs: &Self) -> Self {
        self + rhs
    }

    fn sub(&self, rhs: &Self) -> Self {
        self - rhs
    }

    fn mul(&self, rhs: &Self) -> Self {
        self * rhs
    }

    fn div(&self, rhs: &Self) -> Self {
        self / rhs
    }

    fn pow(&self, rhs: &Self) -> Self {
        self.powf(*rhs)
    }
}

impl Literal for Decimal {
    fn parse(repr: &str) -> Self {
        if repr.find('e').is_none() {
            Self::from_str(repr).unwrap()
        } else {
            Self::from_scientific(repr).unwrap()
        }
    }

    fn abs(&self) -> Self {
        self.abs()
    }

    fn sqrt(&self) -> Self {
        self.sqrt().unwrap()
    }

    fn sin(&self) -> Self {
        self.to_f64().unwrap().sin().try_into().unwrap()
    }

    fn cos(&self) -> Self {
        self.to_f64().unwrap().cos().try_into().unwrap()
    }

    fn tg(&self) -> Self {
        self.to_f64().unwrap().tg().try_into().unwrap()
    }

    fn ctg(&self) -> Self {
        self.to_f64().unwrap().ctg().try_into().unwrap()
    }

    fn add(&self, rhs: &Self) -> Self {
        self + rhs
    }

    fn sub(&self, rhs: &Self) -> Self {
        self - rhs
    }

    fn mul(&self, rhs: &Self) -> Self {
        self * rhs
    }

    fn div(&self, rhs: &Self) -> Self {
        self / rhs
    }

    fn pow(&self, rhs: &Self) -> Self {
        if self.is_sign_positive() {
            (rhs * self.ln()).exp()
        } else {
            panic!("Calculate real power of negative number by yourself")
        }
    }
}
