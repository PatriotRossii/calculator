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
    pub fn apply<T>(&self, lhs: &Expression<T>, rhs: &Expression<T>) -> T
    where
        T: Literal,
    {
        match *self {
            BinaryOperation::Add => lhs.evaluate().add(&rhs.evaluate()),
            BinaryOperation::Sub => lhs.evaluate().sub(&rhs.evaluate()),
            BinaryOperation::Div => lhs.evaluate().div(&rhs.evaluate()),
            BinaryOperation::Mul => lhs.evaluate().mul(&rhs.evaluate()),
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
