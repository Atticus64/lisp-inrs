use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Void,
    Integer(i64),
    Bool(bool),
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
            Object::Str(s) => write!(f, "{}", s),
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
