use crate::env::*;
use crate::object::*;
use crate::parser::*;

#[derive(Debug)]
enum Number {
    Float(f64),
    Integer(i64),
}

fn eval_string_op(list: &Vec<Object>, env: &mut Env) -> Result<Object, String> {

    if list.len() != 3 {
        return Err("Invalid number of arguments for infix operator".to_string());
    }

    let operator = list[0].clone();
    let left = eval_obj(&list[1].clone(), env)?;
    let right = eval_obj(&list[2].clone(), env)?;
    let left_val = match left {
        Object::Str(s) => s,
        Object::Integer(i) => i.to_string(),
        Object::Float(f) => f.to_string(),
        Object::Bool(b) => b.to_string(),
        _ => return Err(format!("Left operand must be an string {:?}", left)),
    };

    let right_val = match right {
        Object::Str(s) => s,
        Object::Integer(i) => i.to_string(),
        Object::Bool(b) => b.to_string(),
        Object::Float(f) => f.to_string(),
        _ => return Err(format!("Right operand must be an string {:?}", right)),
    };

    match operator {
        Object::Symbol(s) => match s.as_str() {
            "concat" => Ok(Object::Str(left_val + &right_val)),
            _ => Err(format!("Invalid infix operator: {}", s)),
        },
        _ => Err("Operator must be a symbol".to_string()),
    }
}

fn get_float_op(op: &str, numbers: (f64, f64)) -> Result<Object, String> {
    let (l, r) = numbers;

    match op {
        "+" => Ok(Object::Float(l + r)),
        "-" => Ok(Object::Float(l - r)),
        "*" => Ok(Object::Float(l * r)),
        "/" => Ok(Object::Float(l / r)),
        ">" => Ok(Object::Bool(l > r)),
        "<" => Ok(Object::Bool(l < r)),
        "==" => Ok(Object::Bool(l == r)),
        ">=" => Ok(Object::Bool(l >= r)),
        "<=" => Ok(Object::Bool(l <= r)),
        "^" => Ok(Object::Float(l.powf(r))),
        _ => Err(format!("Invalid infix operator: {}", op)),
    }
}

fn get_int_op(op: &str, numbers: (i64, i64)) -> Result<Object, String> {
    let (l, r) = numbers;

    match op {
        "+" => Ok(Object::Integer(l + r)),
        "-" => Ok(Object::Integer(l - r)),
        "*" => Ok(Object::Integer(l * r)),
        "/" => Ok(Object::Integer(l / r)),
        ">" => Ok(Object::Bool(l > r)),
        "<" => Ok(Object::Bool(l < r)),
        "==" => Ok(Object::Bool(l == r)),
        ">=" => Ok(Object::Bool(l >= r)),
        "<=" => Ok(Object::Bool(l <= r)),
        "^" => Ok(Object::Integer(l.pow(r as u32))),
        _ => Err(format!("Invalid infix operator: {}", op)),
    }
}

fn num_operations(operator: Object, numbers: (Number, Number)) -> Result<Object, String> {
    let (left, right) = numbers;

    match operator {
        Object::Symbol(s) => {
            if let Number::Float(l) = left {
                let r = match right {
                    Number::Float(g) => g,
                    Number::Integer(i) => i as f64,
                };

                return get_float_op(s.as_str(), (l, r));
            } else if let Number::Integer(l) = left {
                let r = match right {
                    Number::Float(r) => {
                        let left = l as f64;
                        return get_float_op(s.as_str(), (left, r));
                    }
                    Number::Integer(i) => i,
                };

                return get_int_op(s.as_str(), (l, r));
            } else {
                Err(format!("Left operand must be a number {:?}", left))
            }
        }
        _ => Err("Operator must be a symbol".to_string()),
    }
}

fn eval_binary_op(list: &Vec<Object>, env: &mut Env) -> Result<Object, String> {
    if list.len() != 3 {
        return Err("Invalid number of arguments for infix operator".to_string());
    }

    let operator = list[0].clone();
    let left = eval_obj(&list[1].clone(), env)?;
    let right = eval_obj(&list[2].clone(), env)?;

    if let Object::Float(f) = left {
        let r = match right {
            Object::Float(g) => Number::Float(g),
            Object::Integer(i) => Number::Integer(i),
            _ => return Err(format!("Right operand must be a number {:?}", right)),
        };

        return num_operations(operator, (Number::Float(f), r));
    }

    if let Object::Integer(f) = left {
        let r = match right {
            Object::Float(i) => Number::Float(i),
            Object::Integer(i) => Number::Integer(i),
            _ => return Err(format!("Right operand must be a number {:?}", right)),
        };

        num_operations(operator, (Number::Integer(f), r))
    } else {
        Err("Operands must be a numbers".to_string())
    }
}

fn eval_define(list: &Vec<Object>, env: &mut Env) -> Result<Object, String> {
    if list.len() != 3 {
        return Err("Invalid number of arguments for define".to_string());
    }

    let sym = match &list[1] {
        Object::Symbol(s) => s.clone(),
        Object::Keyword(k) => return Err(format!("Cannot define the keyword `{}`", k)),
        _ => return Err("Invalid define".to_string()),
    };
    let val = eval_obj(&list[2], env)?;
    env.set(&sym, val);
    Ok(Object::Void)
}

fn eval_if(list: &Vec<Object>, env: &mut Env) -> Result<Object, String> {
    if list.len() != 4 {
        return Err("Invalid number of arguments for if statement".to_string());
    }

    let cond_obj = eval_obj(&list[1], env)?;
    let cond = match cond_obj {
        Object::Bool(b) => b,
        _ => return Err("Condition must be a boolean".to_string()),
    };

    if cond {
        eval_obj(&list[2], env)
    } else {
        eval_obj(&list[3], env)
    }
}

fn eval_function_definition(list: &[Object]) -> Result<Object, String> {
    let params = match &list[1] {
        Object::List(list) => {
            let mut params = Vec::new();
            for param in list {
                match param {
                    Object::Symbol(s) => params.push(s.clone()),
                    _ => return Err("Invalid lambda parameter".to_string()),
                }
            }
            params
        }
        _ => return Err("Invalid lambda".to_string()),
    };

    let body = match &list[2] {
        Object::List(list) => list.clone(),
        _ => return Err("Invalid lambda".to_string()),
    };

    Ok(Object::Lambda(params, body))
}

fn eval_function_call(s: &str, list: &[Object], env: &mut Env) -> Result<Object, String> {
    let lamdba = env.get(s);
    if lamdba.is_none() {
        return Err(format!("Unbound symbol: {}", s));
    }

    let func = lamdba.unwrap();
    match func {
        Object::Lambda(params, body) => {
            let mut new_env = env.clone();
            for (i, param) in params.iter().enumerate() {
                let arg = match list.get(i + 1) {
                    Some(a) => a,
                    None => return Err(format!("Invalid number of arguments for lambda: {}", s)),
                };
                let val = eval_obj(arg, env)?;
                new_env.set(param, val);
            }
            eval_obj(&Object::List(body), &mut new_env)
        }
        Object::Str(str) => {
            println!("{}", str);
            Ok(Object::Void)
        }
        Object::Bool(b) => {
            println!("{}", b);
            Ok(Object::Void)
        }
        Object::Integer(i) => {
            println!("{}", i);
            Ok(Object::Void)
        }
        Object::Float(f) => {
            println!("{}", f);
            Ok(Object::Void)
        }
        _ => Err(format!("Not a lambda: {}", s)),
    }
}

fn eval_symbol(s: &str, env: &mut Env) -> Result<Object, String> {
    let val = env.get(s);
    if val.is_none() {
        return Err(format!("Unbound symbol: {}", s));
    }
    Ok(val.unwrap())
}

fn get_type(obj: &Object) -> String {
    match obj {
        Object::Keyword(_) => "Keyword".to_string(),
        Object::List(_) => "List".to_string(),
        Object::Symbol(_) => "Symbol".to_string(),
        Object::Lambda(_, _) => "Lambda".to_string(),
        Object::Str(_) => "Str".to_string(),
        Object::Bool(_) => "Bool".to_string(),
        Object::Integer(_) => "Integer".to_string(),
        Object::Float(_) => "Float".to_string(),
        Object::Void => "Void".to_string(),
    }
}


fn get_doc(k: String) -> String {

    match k.as_str() {
        "if" => "Conditional if".to_string(),
        "define" => "Define a symbol".to_string(),
        "lambda" => "define a Lambda function".to_string(),
        "equal" => "Check if two values are equal".to_string(),
        "print" => "Print a value".to_string(),
        "load" => "Load a file".to_string(),
        _ => "".to_string(),
    }
}

fn eval_print(list: &Vec<Object>, env: &mut Env) -> Result<Object, String> {
    if list.len() == 1 {
        return Err("Invalid number of arguments for print".to_string());
    }

    let obj = list[1].clone();
    match obj {
        Object::Keyword(k) => {
            let explanation = get_doc(k);
            println!("{}", explanation);
            Ok(Object::Void)
        },
        Object::Symbol(s) => {
            let val = env.get(&s);
            if val.is_none() {
                return Err(format!("Unbound symbol: {}", s));
            }
            let val = val.unwrap();
            let t = get_type(&val);

            println!("Type: {t}, Var {s}: {}", val);
            Ok(Object::Void)
        }
        Object::Lambda(_, _) => {
            println!("{}", obj);
            Ok(Object::Void)
        }
        Object::Str(str) => {
            println!("Str: {}", str);
            Ok(Object::Void)
        }
        Object::Bool(b) => {
            println!("Bool: {}", b);
            Ok(Object::Void)
        }
        Object::Integer(i) => {
            println!("Int: {}", i);
            Ok(Object::Void)
        }
        Object::Float(f) => {
            println!("Float: {}", f);
            Ok(Object::Void)
        }
        Object::List(l) => {
            let obj = eval_list(&l, env)?;
            println!("{}", obj);
            Ok(Object::Void)
        }
        _ => Err("Invalid print argument".to_string()),
    }
}

fn eval_load(list: &Vec<Object>, env: &mut Env) -> Result<Object, String> {
    if list.len() == 1 {
        return Err("Invalid number of arguments for load".to_string());
    }

    let obj = list[1].clone();

    let mut file = match obj {
        Object::Str(s) => s,
        Object::Symbol(sym) => {
            let val = env.get(&sym);
            if val.is_none() {
                return Err(format!("Unbound symbol: {}", sym));
            }
            match val.unwrap() {
                Object::Str(s) => s,
                _ => return Err("Load arg must be a String".to_string()),
            }
        }
        _ => return Err("Load argument must be a String or Symbol to String".to_string()),
    };

    let ext = match file.split('.').last() {
        Some(s) => s,
        _ => "",
    };

    if ext != "lisp" && ext != "cl" && file.contains('.') {
        return Err(format!("Invalid file extension: {}", ext));
    }

    if ext != "cl" && ext != "lisp" {
        file = file.to_string() + ".lisp";
    }

    let data = match std::fs::read_to_string(&file) {
        Ok(s) => s,
        Err(_) => {
            return Err(format!("Module {} not found", file));
        }
    };

    let parsed_list = parse(&data);
    let ls = match parsed_list {
        Ok(list) => list,
        Err(e) => return Err(e.to_string()),
    };

    Ok(eval_obj(&ls, env).unwrap())
}

fn eval_equal(list: &Vec<Object>, env: &mut Env) -> Result<Object, String> {
    if list.len() != 3 {
        return Err("Invalid number of arguments for equal".to_string());
    }

    let left = eval_obj(&list[1], env)?;
    let right = eval_obj(&list[2], env)?;

    Ok(Object::Bool(left == right))
}

fn eval_keyword(kw: &str, list: &Vec<Object>, env: &mut Env) -> Result<Object, String> {
    match kw {
        "define" => eval_define(list, env),
        "load" => eval_load(list, env),
        "print" => eval_print(list, env),
        "if" => eval_if(list, env),
        "lambda" => eval_function_definition(list),
        "equal" => eval_equal(list, env),
        _ => Err(format!("Invalid keyword: {}", kw)),
    }
}

fn eval_list(list: &Vec<Object>, env: &mut Env) -> Result<Object, String> {
    if list.is_empty() {
        return Ok(Object::Void);
    }

    let head = &list[0];
    let operators = ["+", "-", "*", "/", "<", ">", "=", "!=", "^", ">=", "<="];
    let str_op = ["concat"];
    match head {
        Object::Keyword(k) => eval_keyword(k, list, env),
        Object::Symbol(s) => match s.as_str() {
            ref oper if operators.contains(oper) => eval_binary_op(list, env),
            ref op if str_op.contains(op) => eval_string_op(list, env),
            _ => eval_function_call(s, list, env),
        },
        _ => {
            let mut new_list = Vec::new();
            for obj in list {
                let result = eval_obj(obj, env)?;
                match result {
                    Object::Void => {}
                    _ => new_list.push(result),
                }
            }
            Ok(Object::List(new_list))
        }
    }
}

fn eval_obj(obj: &Object, env: &mut Env) -> Result<Object, String> {
    match obj {
        Object::List(list) => eval_list(list, env),
        Object::Void => Ok(Object::Void),
        Object::Keyword(k) => Ok(Object::Keyword(k.clone())),
        Object::Lambda(_params, _body) => Ok(Object::Void),
        Object::Bool(_) => Ok(obj.clone()),
        Object::Integer(n) => Ok(Object::Integer(*n)),
        Object::Float(f) => Ok(Object::Float(*f)),
        Object::Str(s) => Ok(Object::Str(s.clone())),
        Object::Symbol(s) => eval_symbol(s, env),
    }
}

pub fn eval(program: &str, env: &mut Env) -> Result<Object, String> {
    let parsed_list = parse(program);
    if parsed_list.is_err() {
        return Err(format!("{}", parsed_list.err().unwrap()));
    }
    eval_obj(&parsed_list.unwrap(), env)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_add() {
        let mut env = Env::new();
        let result = eval("(+ 1 2)", &mut env).unwrap();
        assert_eq!(result, Object::Integer(3));
    }

    #[test]
    fn test_area_of_a_circle() {
        let mut env = Env::new();
        let program = "(
                        (define r 10)
                        (define pi 314)
                        (* pi (* r r))
                      )";
        let result = eval(program, &mut env).unwrap();
        assert_eq!(
            result,
            Object::List(vec![Object::Integer((314 * 10 * 10) as i64)])
        );
    }

    #[test]
    fn test_sqr_function() {
        let mut env = Env::new();
        let program = "(
                        (define sqr (lambda (r) (* r r))) 
                        (sqr 10)
                       )";
        let result = eval(program, &mut env).unwrap();
        assert_eq!(
            result,
            Object::List(vec![Object::Integer((10 * 10) as i64)])
        );
    }

    #[test]
    fn test_fibonacci() {
        let mut env = Env::new();
        let program = "
            (
                (define fib (lambda (n) (if (< n 2) 1 (+ (fib (- n 1)) (fib (- n 2))))))
                (fib 10)
            )
        ";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(result, Object::List(vec![Object::Integer(89)]));
    }

    #[test]
    fn test_factorial() {
        let mut env = Env::new();
        let program = "
            (
                (define fact (lambda (n) (if (< n 1) 1 (* n (fact (- n 1))))))
                (fact 5)
            )
        ";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(result, Object::List(vec![Object::Integer(120)]));
    }

    #[test]
    fn test_circle_area_function() {
        let mut env = Env::new();
        let program = "
            (
                (define pi 314)
                (define r 10)
                (define sqr (lambda (r) (* r r)))
                (define area (lambda (r) (* pi (sqr r))))
                (area r)
            )
        ";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(
            result,
            Object::List(vec![Object::Integer((314 * 10 * 10) as i64)])
        );
    }

    #[test]
    fn test_print_correct_str() {
        let mut env = Env::new();
        let program = r#"
            (
                (define age 50)
                (define old "Youre Old")
                (define young "Youre Young")
                (define res (lambda (age) (if (>= age 40) old young)))
                (res 40)
            )
        "#;

        let result = eval(program, &mut env).unwrap();
        assert_eq!(
            result,
            Object::List(vec![Object::Str("Youre Old".to_string())])
        );
    }

    #[test]
    fn test_concat_str() {
        let mut env = Env::new();
        let program = r#"
            (
                (define name "Midnight ")
                (define phrase "esta fumado 🚬")
                (concat name phrase)
            )
            "#;

        let result = eval(program, &mut env).unwrap();
        assert_eq!(
            result,
            Object::List(vec![Object::Str("Midnight esta fumado 🚬".to_string())])
        );
    }

    #[test]
    fn float_operations() {
        let mut env = Env::new();

        let program = r#"
            (
                (define PI 3.1416)
                (define r 3)
                (define area (lambda (r) (* PI (* r r))))
                (area r)
            )"#;

        let result = eval(program, &mut env).unwrap();

        assert_eq!(result, Object::List(vec![Object::Float(28.2744)]));
    }

    #[test]
    fn negative_operations() {
        let mut env = Env::new();

        let program = r#"
            (
                (define debt -4000)
                (define money 6000)
                (+ money debt)
            )"#;

        let result = eval(program, &mut env).unwrap();

        assert_eq!(result, Object::List(vec![Object::Integer(2000)]));
    }

    #[test]
    fn equal_keyword() {
        let mut env = Env::new();

        let program = r#"
            (
                (define age 20)
                (equal age 29)
            )
        "#;

        let result = eval(program, &mut env).unwrap();

        assert_eq!(result, Object::List(vec![Object::Bool(false)]));
    }
}
