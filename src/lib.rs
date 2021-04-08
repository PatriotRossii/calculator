use entities::{
    expression::{Evaluate, Expression},
    literal::Literal,
};
use parser::{GrammarParser, Parse, Rule};
use pest::Parser;

extern crate pest;
#[macro_use]
extern crate pest_derive;

type Number = f64;

pub mod entities;
pub mod parser;

pub struct Calculator {}
impl Calculator {
    pub fn evaluate_string(&self, str: String) -> Literal {
        let expression = Expression::parse(
            GrammarParser::parse(Rule::expr, &str)
                .unwrap()
                .next()
                .unwrap(),
        );
        expression.evaluate()
    }
}
