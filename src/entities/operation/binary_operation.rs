use pest::iterators::Pair;

use crate::{
    entities::{
        expression::{Evaluate, Expression},
        literal::Literal,
    },
    parser::{Parse, Rule},
};

#[derive(Clone, Copy, Debug)]
pub enum BinaryOperation {
    Add,
    Sub,
    Div,
    Mul,
}

impl BinaryOperation {
    pub fn apply(&self, lhs: &Expression, rhs: &Expression) -> Literal {
        match *self {
            BinaryOperation::Add => lhs.evaluate() + rhs.evaluate(),
            BinaryOperation::Sub => lhs.evaluate() - rhs.evaluate(),
            BinaryOperation::Div => lhs.evaluate() / rhs.evaluate(),
            BinaryOperation::Mul => lhs.evaluate() * rhs.evaluate(),
        }
    }
}

impl Parse for BinaryOperation {
    fn parse(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::binary_op => {
                let inner = pair.into_inner().next().unwrap();
                match inner.as_rule() {
                    Rule::add => Self::Add,
                    Rule::sub => Self::Sub,
                    Rule::div => Self::Div,
                    Rule::mul => Self::Mul,
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
}
