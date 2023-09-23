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
    ch: char
}

impl Error for TokenError {}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "expected character: {}", self.ch)
    }
}

/// Validate the correct position of the parens `()`
///
/// ```lisp
/// (concat "hola" ("mundo")
/// // should throw error
/// ```
fn paren_validation(input: &str) -> bool {
    let chars: Vec<char> = input.chars().filter(|c| *c == '(' || *c == ')').collect();
    let left  = chars.iter().filter(|c| **c == '(').count();
    let right = chars.iter().filter(|c| **c == ')').count();

    left == right
}

pub fn tokenize(program: &str) -> Result<Vec<Token>, TokenError> {
    if !paren_validation(program) {
        return Err(TokenError { ch: ')' });
    }

    let program_fmt = program.replace('(', " ( ").replace(')', " ) ");

    let mut tokens: Vec<Token> = Vec::new();
    let chars: Vec<char> = program_fmt.chars().collect();
    let mut build_str = false;
    let mut build_num = false;
    let mut pos_num = String::new();
    let mut pos_str = String::new();
    let mut pos_sym = String::new();

    for (i, c) in program_fmt.chars().enumerate() {
        match c {
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            _ => {
                if c.is_whitespace() && !build_str {
                    let i = pos_num.parse::<i64>();
                    let f = pos_num.parse::<f64>();

                    if let Ok(integer) = i {
                        tokens.push(Token::Integer(integer));
                        pos_num = "".to_string();
                        build_num = false;
                        continue;
                    } else if let Ok(float) = f {
                        tokens.push(Token::Float(float));
                        pos_num = "".to_string();
                        build_num = false;
                        continue;
                    }

                    if !pos_sym.is_empty() {
                        tokens.push(Token::Symbol(pos_sym.to_string()));
                        pos_sym = "".to_string();
                    }

                    continue;
                }


                if c.is_numeric() && !pos_sym.is_empty() {
                    pos_sym.push(c);
                    continue;
                }

                if c.is_numeric() && !build_str && pos_sym.is_empty() {
                    pos_num.push(c);
                    build_num = true;
                    build_str = false;
                    continue;
                }

                if build_str {
                    if c == '"' {
                        tokens.push(Token::Str(pos_str.to_string()));
                        pos_str = "".to_string();
                        build_str = false;
                        continue;
                    }

                    pos_str.push(c);
                    continue;
                }

                if c == '"' {
                    build_str = true;
                    continue;
                }


                if c.is_ascii() && !build_num || c == '+' && c == '-' {

                    if c != '+' && c != '-' {
                        pos_sym.push(c);
                        continue;
                    }

                    if let Some(next) = chars.get(i + 1) {
                        if next == &' ' {
                            pos_sym.push(c);
                            continue;
                        } else if next.is_numeric() {
                            pos_num.push(c);
                            continue;
                        }
                    }
                    pos_sym.push(c);
                } else if c == '.' {
                    pos_num.push(c);
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

    #[test]
    fn string_tokens() {
        let program = r#"
                (concat "hola" "mundo")
        "#;
        let tokens = tokenize(program).unwrap_or(vec![]);
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::Symbol("concat".to_string()),
                Token::Str("hola".to_string()),
                Token::Str("mundo".to_string()),
                Token::RParen
            ]
        );
    }

    #[test]
    fn string_with_spaces() {
        let program = r#"
                (concat "hola " " a todos!")
        "#;

        let tokens = tokenize(program).unwrap_or(vec![]);

        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::Symbol("concat".to_string()),
                Token::Str("hola ".to_string()),
                Token::Str(" a todos!".to_string()),
                Token::RParen
            ]
        );
    }

    #[test]
    fn string_with_spaces_revenge() {
        let program = r#"
                (concat "hola" " a todos! ")
        "#;

        let tokens = tokenize(program).unwrap_or(vec![]);

        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::Symbol("concat".to_string()),
                Token::Str("hola".to_string()),
                Token::Str(" a todos! ".to_string()),
                Token::RParen
            ]
        );
    }
}
