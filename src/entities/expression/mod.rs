use std::{ops::Deref, rc::Rc};

use pest::iterators::Pair;

use crate::parser::{Parse, Rule};

use super::{
    literal::Literal,
    operation::{binary_operation::BinaryOperation, unary_operation::UnaryOperation},
};

pub trait Evaluate {
    fn evaluate(&self) -> Literal;
}

#[derive(Clone, Debug)]
pub struct UnaryExpression {
    lhs: Rc<Expression>,
    operation: UnaryOperation,
}

impl Parse for UnaryExpression {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut inner = pair.into_inner();

        let operation = UnaryOperation::parse(inner.next().unwrap());
        let lhs = Expression::parse(inner.next().unwrap());

        Self {
            lhs: Rc::new(lhs),
            operation,
        }
    }
}

impl Evaluate for UnaryExpression {
    fn evaluate(&self) -> Literal {
        self.operation.apply(self.lhs.deref())
    }
}

#[derive(Clone, Debug)]
pub struct BinaryExpression {
    lhs: Rc<Expression>,
    rhs: Rc<Expression>,
    operation: BinaryOperation,
}

impl Evaluate for BinaryExpression {
    fn evaluate(&self) -> Literal {
        self.operation.apply(self.lhs.deref(), self.rhs.deref())
    }
}

impl Parse for BinaryExpression {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut inner = pair.into_inner();

        let operation = BinaryOperation::parse(inner.next().unwrap());
        let lhs = Expression::parse(inner.next().unwrap());
        let rhs = Expression::parse(inner.next().unwrap());

        Self {
            operation,
            lhs: Rc::new(lhs),
            rhs: Rc::new(rhs),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LiteralExpression {
    value: Literal,
}

impl Evaluate for LiteralExpression {
    fn evaluate(&self) -> Literal {
        self.value
    }
}

impl Parse for LiteralExpression {
    fn parse(pair: Pair<Rule>) -> Self {
        Self {
            value: Literal::parse(pair),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Expression {
    Literal(LiteralExpression),
    Unary(UnaryExpression),
    Binary(BinaryExpression),
}

impl Evaluate for Expression {
    fn evaluate(&self) -> Literal {
        match self {
            Expression::Literal(expr) => expr.evaluate(),
            Expression::Unary(expr) => expr.evaluate(),
            Expression::Binary(expr) => expr.evaluate(),
        }
    }
}

impl Parse for Expression {
    fn parse(pair: Pair<Rule>) -> Self {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::literal_expr => Self::Literal(LiteralExpression::parse(inner)),
            Rule::unary_expr => Self::Unary(UnaryExpression::parse(inner)),
            Rule::binary_expr => Self::Binary(BinaryExpression::parse(inner)),
            _ => unreachable!(),
        }
    }
}
