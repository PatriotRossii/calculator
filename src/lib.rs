#![feature(once_cell)]

use entities::{
    expression::{Evaluate, Expression},
    literal::Literal,
};
use parser::{GrammarParser, Parse, Rule};

use std::collections::HashMap;

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

#[derive(Debug, Clone)]
pub enum AngleRepresentation {
    Degree,
    Radian,
}

#[derive(Debug, Clone)]
pub struct CalculatorState {
    angle_repr: AngleRepresentation,
    variables: HashMap<String, CalculationResult>,
}

impl CalculatorState {
    pub fn new(angle_representation: AngleRepresentation) -> Self {
        Self {
            angle_repr: angle_representation,
            variables: HashMap::new(),
        }
    }

    pub fn push_variable<T>(&mut self, key: T, value: CalculationResult)
    where
        T: ToString,
    {
        self.variables.insert(key.to_string(), value);
    }

    pub fn remove_variable(&mut self, key: &str) {
        self.variables.remove(key);
    }
}

impl Default for CalculatorState {
    fn default() -> Self {
        Self::new(AngleRepresentation::Radian)
    }
}

#[derive(Debug, Clone)]
pub struct Calculator {
    state: CalculatorState,
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new(CalculatorState::default())
    }
}

impl Calculator {
    pub fn new(state: CalculatorState) -> Self {
        Self { state }
    }

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

    pub fn evaluate_expression<T>(&self, expr: Expression<T>) -> T
    where
        T: Literal,
    {
        expr.evaluate(&self.state)
    }

    pub fn evaluate_string(&self, str: &str, mode: CalculationMode) -> CalculationResult {
        match mode {
            CalculationMode::HighPrecision => CalculationResult::HighPrecision(
                self.evaluate_expression(Calculator::parse_expression(str)),
            ),
            CalculationMode::StandardPrecision => CalculationResult::StandardPrecision(
                self.evaluate_expression(Calculator::parse_expression(str)),
            ),
        }
    }

    pub fn evaluate_standard(&self, str: &str) -> f64 {
        match self.evaluate_string(str, CalculationMode::StandardPrecision) {
            CalculationResult::StandardPrecision(e) => e,
            _ => unreachable!(),
        }
    }

    pub fn evaluate_high(&self, str: &str) -> Decimal {
        match self.evaluate_string(str, CalculationMode::HighPrecision) {
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
        let calculator = Calculator::default();

        assert_eq!(calculator.evaluate_standard("add(2, 2)"), 2_f64 * 2_f64);
        assert_eq!(calculator.evaluate_standard("sub(4.5, 2)"), 4.5_f64 - 2_f64);
        assert_eq!(
            calculator.evaluate_standard("mul(100, 12)"),
            100_f64 * 12_f64
        )
    }

    #[test]
    pub fn evaluate_simple_unary() {
        let calculator = Calculator::default();

        assert_eq!(calculator.evaluate_standard("sin(3.14)"), 3.14_f64.sin());
        assert_eq!(calculator.evaluate_standard("abs(-5)"), (-5_f64).abs());
        assert_eq!(calculator.evaluate_standard("sqrt(1000)"), 1000_f64.sqrt());
    }

    #[test]
    pub fn test_angle_transform() {
        let calculator = Calculator::new(CalculatorState::new(AngleRepresentation::Degree));

        assert_eq!(
           calculator.evaluate_standard("sin(180)").round(),
            0_f64,
        );

        assert_eq!(
            calculator.evaluate_standard("cos(180)").round(),
            -1_f64,
        );

        assert_eq!(
            calculator.evaluate_standard("tg(45)").round(),
            1_f64,
        );
    }
}
