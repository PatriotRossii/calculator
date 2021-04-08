use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct GrammarParser;

pub trait Parse {
    fn parse(pair: Pair<Rule>) -> Self;
}

#[cfg(test)]
mod tests {
    use pest::{Parser};
    use crate::{GrammarParser, Rule};

    #[test]
    pub fn binary_operation() {
        let operations = vec![
            "add", "sub", "mul", "div", "pow"
        ];
        for operation in operations {
            GrammarParser::parse(Rule::binary_op, operation).unwrap();
        }
    }

    #[test]
    pub fn unary_operation() {
        let operations = vec![
            "abs", "sqrt", "sin", "cos", "tg", "ctg"
        ];
        for operation in operations {
            GrammarParser::parse(Rule::unary_op, operation).unwrap();
        }
    }

    #[test]
    pub fn num() {
        let literals = vec![
            "65279", "3.1415927", "1e10", "1e-10",
            "3.1415927e10"
        ];
        for literal in literals {
            GrammarParser::parse(Rule::literal_expr, literal).unwrap();
        }
    }

    #[test]
    pub fn binary_expression() {
        // 1 + 2
        GrammarParser::parse(Rule::binary_expr, "add(1, 2)").unwrap();

        // 5^(8^3)
        GrammarParser::parse(Rule::binary_expr, "pow(5, (pow(8, 3)))").unwrap();
    }
}
