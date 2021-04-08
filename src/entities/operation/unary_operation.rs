use pest::iterators::Pair;

use crate::{
    entities::{
        expression::{Evaluate, Expression},
        literal::Literal,
    },
    parser::{Parse, Rule},
};

#[derive(Clone, Copy, Debug)]
pub enum UnaryOperation {
    Abs,
    Sqrt,
    Sin,
    Cos,
    Tg,
    Ctg,
}

impl UnaryOperation {
    pub fn apply(&self, lhs: &Expression) -> Literal {
        match *self {
            UnaryOperation::Abs => lhs.evaluate().abs(),
            UnaryOperation::Sqrt => lhs.evaluate().sqrt(),
            UnaryOperation::Sin => lhs.evaluate().sin(),
            UnaryOperation::Cos => lhs.evaluate().cos(),
            UnaryOperation::Tg => lhs.evaluate().tan(),
            UnaryOperation::Ctg => lhs.evaluate().tan().powi(-1),
        }
    }
}

impl Parse for UnaryOperation {
    fn parse(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::binary_op => {
                let inner = pair.into_inner().next().unwrap();
                match inner.as_rule() {
                    Rule::abs => Self::Abs,
                    Rule::sqrt => Self::Sqrt,
                    Rule::sin => Self::Sin,
                    Rule::cos => Self::Cos,
                    Rule::tg => Self::Tg,
                    Rule::ctg => Self::Ctg,
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
}
