use std::{ops::Deref, rc::Rc};

use pest::iterators::Pair;

use crate::parser::{Parse, Rule};

use super::{
    literal::Literal,
    operation::{binary_operation::BinaryOperation, unary_operation::UnaryOperation},
};

pub trait Evaluate<T>
where
    T: Literal,
{
    fn evaluate(&self) -> T;
}

#[derive(Clone, Debug)]
pub struct UnaryExpression<T>
where
    T: Literal,
{
    lhs: Rc<Expression<T>>,
    operation: UnaryOperation,
}

impl<T> Parse for UnaryExpression<T>
where
    T: Literal,
{
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

impl<T> Evaluate<T> for UnaryExpression<T>
where
    T: Literal,
{
    fn evaluate(&self) -> T {
        self.operation.apply(self.lhs.deref())
    }
}

#[derive(Clone, Debug)]
pub struct BinaryExpression<T>
where
    T: Literal,
{
    lhs: Rc<Expression<T>>,
    rhs: Rc<Expression<T>>,
    operation: BinaryOperation,
}

impl<T> Evaluate<T> for BinaryExpression<T>
where
    T: Literal,
{
    fn evaluate(&self) -> T {
        self.operation.apply(self.lhs.deref(), self.rhs.deref())
    }
}

impl<T> Parse for BinaryExpression<T>
where
    T: Literal,
{
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
pub struct LiteralExpression<T>
where
    T: Literal,
{
    value: T,
}

impl<T> Evaluate<T> for LiteralExpression<T>
where
    T: Literal,
{
    fn evaluate(&self) -> T {
        self.value
    }
}

impl<T> Parse for LiteralExpression<T>
where
    T: Literal,
{
    fn parse(pair: Pair<Rule>) -> Self {
        Self {
            value: Literal::parse(pair.into_inner().next().unwrap().as_str()),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Expression<T>
where
    T: Literal,
{
    Literal(LiteralExpression<T>),
    Unary(UnaryExpression<T>),
    Binary(BinaryExpression<T>),
}

impl<T> Evaluate<T> for Expression<T>
where
    T: Literal,
{
    fn evaluate(&self) -> T {
        match self {
            Expression::Literal(expr) => expr.evaluate(),
            Expression::Unary(expr) => expr.evaluate(),
            Expression::Binary(expr) => expr.evaluate(),
        }
    }
}

impl<T> Parse for Expression<T>
where
    T: Literal,
{
    fn parse(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::literal_expr => Self::Literal(LiteralExpression::parse(pair)),
            Rule::unary_expr => Self::Unary(UnaryExpression::parse(pair)),
            Rule::binary_expr => Self::Binary(BinaryExpression::parse(pair)),
            _ => unreachable!(),
        }
    }
}
