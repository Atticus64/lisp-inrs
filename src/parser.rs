use crate::env::KEYWORDS;
use crate::lexer::*;
use crate::object::*;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ParseError {
    err: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error: {}", self.err)
    }
}

impl Error for ParseError {}

pub fn parse(program: &str) -> Result<Object, ParseError> {
    let token_result = tokenize(program);

    if let Err(error) = token_result {
        return Err(ParseError {
            err: format!("{}", error),
        });
    }

    let mut tokens = token_result.unwrap().into_iter().rev().collect::<Vec<_>>();
    let parsed_list = parse_list(&mut tokens)?;

    Ok(parsed_list)
}

fn parse_list(tokens: &mut Vec<Token>) -> Result<Object, ParseError> {
    let token = tokens.pop();

    if token != Some(Token::LParen) {
        return Err(ParseError {
            err: format!("Expected LParen, found {:?}", token),
        });
    }

    let mut list: Vec<Object> = Vec::new();

    while !tokens.is_empty() {
        let token = tokens.pop();

        if token.is_none() {
            return Err(ParseError {
                err: "Did not find enough tokens".to_string(),
            });
        }

        let t = token.unwrap();

        match t {
            Token::Integer(n) => list.push(Object::Integer(n)),
            Token::Float(n) => list.push(Object::Float(n)),
            Token::Str(s) => list.push(Object::Str(s.to_string())),
            Token::Symbol(s) => {

                if KEYWORDS.contains(&s.as_str()) {
                    list.push(Object::Keyword(s.to_string()));
                    continue;
                }


                list.push(Object::Symbol(s));
            }
            Token::LParen => {
                tokens.push(Token::LParen);
                let sub_list = parse_list(tokens)?;
                list.push(sub_list);
            }
            Token::RParen => {
                return Ok(Object::List(list));
            }
        }
    }

    Ok(Object::List(list))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_parse() {
        let list = parse("(+ 1 2)").unwrap();

        assert_eq!(
            list,
            Object::List(vec![
                Object::Symbol("+".to_string()),
                Object::Integer(1),
                Object::Integer(2),
            ])
        )
    }

    #[test]
    fn test_keyword_equal() {

        let list = parse("(equal 1 2)").unwrap();

        assert_eq!(
            list,
            Object::List(vec![
                Object::Keyword("equal".to_string()),
                Object::Integer(1),
                Object::Integer(2),
            ])
        )
    }
}
