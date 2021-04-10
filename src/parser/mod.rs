use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct GrammarParser;

pub trait Parse {
    fn parse(pair: Pair<Rule>) -> Self;
}

#[cfg(test)]
mod tests {
    use crate::{GrammarParser, Rule};
    use pest::Parser;

    #[test]
    pub fn binary_operation() {
        let operations = vec!["add", "sub", "mul", "div", "pow"];
        for operation in operations {
            GrammarParser::parse(Rule::binary_op, operation).unwrap();
        }
    }

    #[test]
    pub fn unary_operation() {
        let operations = vec!["abs", "sqrt", "sin", "cos", "tg", "ctg"];
        for operation in operations {
            GrammarParser::parse(Rule::unary_op, operation).unwrap();
        }
    }

    #[test]
    pub fn num() {
        let literals = vec!["65279", "3.1415927", "1e10", "1e-10", "3.1415927e10"];
        for literal in literals {
            GrammarParser::parse(Rule::literal_expr, literal).unwrap();
        }
    }

    #[test]
    pub fn binary_expression() {
        // 1 + 2
        GrammarParser::parse(Rule::binary_expr, "add(1, 2)").unwrap();

        // 5^(8^3)
        GrammarParser::parse(Rule::binary_expr, "pow(5, pow(8, 3))").unwrap();

        // Test surrounding with parentheses. 5 - (2^2)
        GrammarParser::parse(Rule::binary_expr, "sub((5), (pow(2, 2)))").unwrap();
    }

    #[test]
    pub fn unary_expression() {
        // sin 3.14
        GrammarParser::parse(Rule::unary_expr, "sin(3.14)").unwrap();

        // |-10|
        GrammarParser::parse(Rule::unary_expr, "abs(-10)").unwrap();

        // Square root from 100
        GrammarParser::parse(Rule::unary_expr, "sqrt(100)").unwrap();
    }

    #[test]
    pub fn term() {
        GrammarParser::parse(Rule::term, "5").unwrap();
        GrammarParser::parse(Rule::term, "abs(5)").unwrap();

        GrammarParser::parse(Rule::term, "(5)").unwrap();
        GrammarParser::parse(Rule::term, "(((add(1,2))))").unwrap();
    }
}
