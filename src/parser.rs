use crate::{ast::Node, error::Error};

pub fn parse(x: &str) -> Result<Node, Error> {
    peg::parser! {
        grammar syntax() for str {
            rule _ = quiet!{[' ' | '\r' | '\n' | '\t']*};

            rule number() -> i32
                = n:$(['0'..='9']+) { n.parse().unwrap() }

            rule calculate() -> Node = precedence!{
                x:(@) _ "+" _ y:@ { Node::Add(Box::new(x), Box::new(y)) }
                x:(@) _ "-" _ y:@ { Node::Sub(Box::new(x), Box::new(y)) }
                --
                x:(@) _ "*" _ y:@ { Node::Mul(Box::new(x), Box::new(y)) }
                x:(@) _ "/" _ y:@ { Node::Div(Box::new(x), Box::new(y)) }
                --
                "(" _ v:calculate() _ ")" { v }
                n:number() { Node::Number(n) }
            }

            pub rule expr() -> Node = _ v:calculate() _ { v };
        }
    }

    syntax::expr(x).map_err(|e| Error::Parse {
        message: e.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn skip_space() -> anyhow::Result<()> {
        assert_eq!(parse("1 +\n1 ")?, parse("1+1")?);

        Ok(())
    }
}
