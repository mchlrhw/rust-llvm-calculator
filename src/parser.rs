use crate::{ast::Node, error::Error};

pub fn parse(x: &str) -> Result<Node, Error> {
    peg::parser! {
        grammar syntax() for str {
            rule number() -> i32
                = n:$(['0'..='9']+) { n.parse().unwrap() }

            pub(crate) rule calculate() -> Node = precedence!{
                x:(@) "+" y:@ { Node::Add(Box::new(x), Box::new(y)) }
                x:(@) "-" y:@ { Node::Sub(Box::new(x), Box::new(y)) }
                --
                x:(@) "*" y:@ { Node::Mul(Box::new(x), Box::new(y)) }
                x:(@) "/" y:@ { Node::Div(Box::new(x), Box::new(y)) }
                --
                "(" v:calculate() ")" { v }
                n:number() { Node::Number(n) }
            }
        }
    }

    syntax::calculate(x).map_err(|e| Error::Parse {
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
