use std::fmt;

/// Object in Lisp
#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Void,
    /// Int type of lisp
    /// ```rs
    /// let n1 = Object::Integer(10);
    /// ```
    Integer(i64),
    /// Float type of lisp
    /// ```rs
    /// let pi = Object::Float(3.1416);
    /// ```
    Float(f64),
    Keyword(String),
    /// Boolean type of lisp
    /// ```rs
    /// let is_empty = Object::Bool(true);
    /// ```
    Bool(bool),
    /// String type of lisp
    /// ```rs
    /// let n = String::from("John Connor");
    /// let name = Object::Str(n);
    /// ```
    Str(String),
    Symbol(String),
    Lambda(Vec<String>, Vec<Object>),
    List(Vec<Object>),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Void => write!(f, "Void"),
            Object::Integer(n) => write!(f, "{}", n),
            Object::Float(n) => write!(f, "{}", n),
            Object::Str(s) => write!(f, "{}", s),
            Object::Keyword(s) => write!(f, "Keyword: {}", s),
            Object::Bool(b) => write!(f, "{}", b),
            Object::Symbol(s) => write!(f, "{}", s),
            Object::Lambda(params, body) => {
                write!(f, "Lambda(")?;
                for param in params {
                    write!(f, "{} ", param)?;
                }
                write!(f, ")")?;
                for expr in body {
                    write!(f, "{}", expr)?;
                }

                Ok(())
            }
            Object::List(items) => {
                write!(f, "(")?;

                for (i, obj) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }

                    write!(f, "{}", obj)?;
                }

                write!(f, ")")?;

                Ok(())
            }
        }
    }
}
