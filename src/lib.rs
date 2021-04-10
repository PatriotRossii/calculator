use entities::{
    expression::{Evaluate, Expression},
    literal::Literal,
};
use parser::{GrammarParser, Parse, Rule};
use pest::Parser;

extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod entities;
pub mod parser;

pub struct Calculator {}
impl Calculator {
    pub fn evaluate_string<T>(str: String) -> T
    where
        T: Literal,
    {
        let expression = Expression::parse(
            GrammarParser::parse(Rule::expr, &str)
                .unwrap()
                .next()
                .unwrap(),
        );
        expression.evaluate()
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
        assert_eq!(
            Calculator::evaluate_string::<f64>(String::from("add(2, 2)")),
            2_f64 * 2_f64
        );
        assert_eq!(
            Calculator::evaluate_string::<f64>(String::from("sub(4.5, 2)")),
            4.5_f64 - 2_f64
        );
        assert_eq!(
            Calculator::evaluate_string::<f64>(String::from("mul(100, 12)")),
            100_f64 * 12_f64
        )
    }

    #[test]
    pub fn evaluate_simple_unary() {
        assert_eq!(
            Calculator::evaluate_string::<f64>(String::from("sin(3.14)")),
            3.14_f64.sin()
        );
        assert_eq!(
            Calculator::evaluate_string::<f64>(String::from("abs(-5)")),
            (-5_f64).abs()
        );
        assert_eq!(
            Calculator::evaluate_string::<f64>(String::from("sqrt(1000)")),
            1000_f64.sqrt()
        );
    }
}
