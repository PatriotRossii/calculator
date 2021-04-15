use pest::iterators::Pair;

use crate::{
    entities::{
        expression::{Evaluate, Expression},
        literal::Literal,
    },
    parser::{Parse, Rule},
    AngleRepresentation, CalculatorState,
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
    pub fn apply<T>(&self, lhs: &Expression<T>, state: &CalculatorState) -> T
    where
        T: Literal,
    {
        let mut operand = lhs.evaluate(state);

        if let AngleRepresentation::Degree = state.angle_repr {
            match self {
                UnaryOperation::Sin
                | UnaryOperation::Cos
                | UnaryOperation::Tg
                | UnaryOperation::Ctg => operand = operand.to_radians(),
                _ => {}
            }
        }

        match self {
            UnaryOperation::Abs => operand.abs(),
            UnaryOperation::Sqrt => operand.sqrt(),
            UnaryOperation::Sin => operand.sin(),
            UnaryOperation::Cos => operand.cos(),
            UnaryOperation::Tg => operand.tg(),
            UnaryOperation::Ctg => operand.ctg(),
        }
    }
}

impl Parse for UnaryOperation {
    fn parse(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::unary_op => {
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
