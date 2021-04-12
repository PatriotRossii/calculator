use entities::{
    expression::{Evaluate, Expression},
    literal::Literal,
};
use parser::{GrammarParser, Parse, Rule};
use pest::Parser;
use rust_decimal::Decimal;

extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod entities;
pub mod parser;

#[derive(Debug, Clone, Copy)]
pub enum CalculationMode {
    HighPrecision,
    StandardPrecision,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CalculationResult {
    HighPrecision(Decimal),
    StandardPrecision(f64),
}

pub struct Calculator {}
impl Calculator {
    pub fn parse_expression<T>(str: &str) -> Expression<T>
    where
        T: Literal,
    {
        Expression::parse(
            GrammarParser::parse(Rule::expr, str)
                .unwrap()
                .next()
                .unwrap(),
        )
    }

    pub fn evaluate_expression<T>(expr: Expression<T>) -> T
    where
        T: Literal,
    {
        expr.evaluate()
    }

    pub fn evaluate_string(str: &str, mode: CalculationMode) -> CalculationResult {
        match mode {
            CalculationMode::HighPrecision => CalculationResult::HighPrecision(
                Calculator::evaluate_expression(Calculator::parse_expression(str)),
            ),
            CalculationMode::StandardPrecision => CalculationResult::StandardPrecision(
                Calculator::evaluate_expression(Calculator::parse_expression(str)),
            ),
        }
    }

    pub fn evaluate_standard(str: &str) -> f64 {
        match Calculator::evaluate_string(str, CalculationMode::StandardPrecision) {
            CalculationResult::StandardPrecision(e) => e,
            _ => unreachable!(),
        }
    }

    pub fn evaluate_high(str: &str) -> Decimal {
        match Calculator::evaluate_string(str, CalculationMode::HighPrecision) {
            CalculationResult::HighPrecision(e) => e,
            _ => unreachable!(),
        }
    }

    pub fn get_binary_operations<'a>() -> Vec<&'a str> {
        vec!["add", "sub", "mul", "div", "pow"]
    }
    pub fn get_unary_operations<'a>() -> Vec<&'a str> {
        vec!["abs", "sin", "cos", "tg", "ctg"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn evaluate_simple_binary() {
        assert_eq!(Calculator::evaluate_standard("add(2, 2)"), 2_f64 * 2_f64);
        assert_eq!(
            Calculator::evaluate_standard("sub(4.5, 2)"),
            4.5_f64 - 2_f64
        );
        assert_eq!(
            Calculator::evaluate_standard("mul(100, 12)"),
            100_f64 * 12_f64
        )
    }

    #[test]
    pub fn evaluate_simple_unary() {
        assert_eq!(Calculator::evaluate_standard("sin(3.14)"), 3.14_f64.sin());
        assert_eq!(Calculator::evaluate_standard("abs(-5)"), (-5_f64).abs());
        assert_eq!(Calculator::evaluate_standard("sqrt(1000)"), 1000_f64.sqrt());
    }
}
