use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug)]
pub enum Token {
    Integer(i64),
    Float(f64),
    Str(String),
    Symbol(String),
    LParen,
    RParen,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Integer(n) => write!(f, "{}", n),
            Token::Float(n) => write!(f, "{}", n),
            Token::Str(s) => write!(f, "{}", s),
            Token::Symbol(s) => write!(f, "{}", s),
            Token::RParen => write!(f, "("),
            Token::LParen => write!(f, ")"),
        }
    }
}

#[derive(Debug)]
pub struct TokenError {
    ch: char,
}

impl Error for TokenError {}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unexpected character: {}", self.ch)
    }
}

fn paren_validation(input: &str) -> bool {
    let chars: Vec<char> = input.chars().filter(|c| *c == '(' || *c == ')').collect();
    let left = chars.iter().filter(|c| **c == '(').count();
    let right = chars.iter().filter(|c| **c == ')').count();

    left != right
}

pub fn tokenize(program: &str) -> Result<Vec<Token>, TokenError> {
    if !paren_validation(program) {
        return Err(TokenError { ch: ')' });
    }

    let program_fmt = program.replace('(', " ( ").replace(')', " ) ");

    let words = program_fmt.split_whitespace();

    let mut tokens: Vec<Token> = Vec::new();

    let strs: Vec<&str> = program_fmt.split('\"').collect();
    let mut pos_string = String::new();
    let mut has_quot = false;
    let mut has_spaces = false;
    if strs.len() >= 2 {
        let string = strs[1];
        has_spaces = string.contains(' ');
    }
    for word in words {
        match word {
            "(" => tokens.push(Token::LParen),
            ")" => tokens.push(Token::RParen),
            _ => {
                let i = word.parse::<i64>();
                let f = word.parse::<f64>();

                if let Ok(integer) = i {
                    tokens.push(Token::Integer(integer));
                } else if let Ok(float) = f {
                    tokens.push(Token::Float(float));
                } else if word.contains('\"') {
                    if !has_spaces {
                        pos_string.push_str(word);
                        let new_word: String = pos_string.chars().filter(|c| c != &'\"').collect();
                        tokens.push(Token::Str(new_word));
                    }
                    if !has_quot {
                        has_quot = true;
                        let w = word.to_string() + " ";
                        pos_string.push_str(w.as_str());
                    } else {
                        pos_string.push_str(word);
                        let new_word: String = pos_string.chars().filter(|c| c != &'\"').collect();
                        tokens.push(Token::Str(new_word));
                        has_quot = false;
                        pos_string = String::new();
                    }
                } else {
                    if has_quot {
                        let w = word.to_string() + " ";
                        pos_string.push_str(w.as_str());
                        continue;
                    }
                    tokens.push(Token::Symbol(word.to_string()));
                    pos_string = "".to_string();
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

    #[test]
    fn paren_error() {
        let program = "(
            (define r 10)
            (print r
        )";
        let tokens = tokenize(program).unwrap_or(vec![]);
        let list = tokenize(program);

        assert_eq!(tokens, vec![]);
        assert!(list.is_err());
    }
}
