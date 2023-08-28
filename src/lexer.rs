use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug)]
pub enum Token {
    Integer(i64),
    Symbol(String),
    LParen,
    RParen,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Integer(n) => write!(f, "{}", n),
            Token::Symbol(s) => write!(f, "{}", s),
            Token::RParen => write!(f, "("),
            Token::LParen => write!(f, ")"),
        }
    }
}

#[derive(Debug)]
pub struct TokenError {
    ch: char
}

impl Error for TokenError {}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unexpected character: {}", self.ch)
    }
}


pub fn tokenize(program: &str) -> Result<Vec<Token>, TokenError> {
    let program_fmt = program.replace('(', " ( ").replace(')', " ) ");

    let words = program_fmt.split_whitespace();

    let mut tokens: Vec<Token> = Vec::new();

    for word in words {
        match word {
            "(" => tokens.push(Token::LParen),
            ")" => tokens.push(Token::RParen),
            _ => {
                let i = word.parse::<i64>();

                if let Ok(integer) = i {
                    tokens.push(Token::Integer(integer));
                } else {
                    tokens.push(Token::Symbol(word.to_string()));
                }
            }
        }
    }

    Ok(tokens)

}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn add() {
        let tokens = tokenize("(+ 1 2)").unwrap_or(vec![]);
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::Symbol("+".to_string()),
                Token::Integer(1),
                Token::Integer(2),
                Token::RParen,
            ]
        );
    }

    #[test]
    fn test_area_of_a_circle() {
        let program = "
            (
                (define r 10)
                (define pi 314)
                (* pi (* r r))
            )
        ";
        let tokens = tokenize(program).unwrap_or(vec![]);
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::LParen,
                Token::Symbol("define".to_string()),
                Token::Symbol("r".to_string()),
                Token::Integer(10),
                Token::RParen,
                Token::LParen,
                Token::Symbol("define".to_string()),
                Token::Symbol("pi".to_string()),
                Token::Integer(314),
                Token::RParen,
                Token::LParen,
                Token::Symbol("*".to_string()),
                Token::Symbol("pi".to_string()),
                Token::LParen,
                Token::Symbol("*".to_string()),
                Token::Symbol("r".to_string()),
                Token::Symbol("r".to_string()),
                Token::RParen,
                Token::RParen,
                Token::RParen
            ]
        );
    }
}

