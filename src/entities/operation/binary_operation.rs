use pest::iterators::Pair;

use crate::{
    entities::{
        expression::{Evaluate, Expression},
        literal::Literal,
    },
    parser::{Parse, Rule},
    CalculatorState,
};

#[derive(Clone, Copy, Debug)]
pub enum BinaryOperation {
    Add,
    Sub,
    Div,
    Mul,
}

impl BinaryOperation {
    pub fn apply<T>(&self, lhs: &Expression<T>, rhs: &Expression<T>, state: &CalculatorState) -> T
    where
        T: Literal,
    {
        let lhs = lhs.evaluate(state);
        let rhs = rhs.evaluate(state);

        match *self {
            BinaryOperation::Add => lhs.add(&rhs),
            BinaryOperation::Sub => lhs.sub(&rhs),
            BinaryOperation::Div => lhs.div(&rhs),
            BinaryOperation::Mul => lhs.mul(&rhs),
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
