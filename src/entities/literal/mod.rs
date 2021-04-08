use pest::iterators::Pair;

use crate::{
    parser::{Parse, Rule},
    Number,
};

pub type Literal = Number;

impl Parse for Literal {
    fn parse(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::num => pair.as_str().parse::<f64>().unwrap(),
            _ => unreachable!(),
        }
    }
}
