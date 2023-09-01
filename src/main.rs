mod lexer;
mod object;
mod parser;
mod env;
mod eval;

use linefeed::{Interface, ReadResult};
use object::Object;

const PROMPT: &str = "lisp-rs> ";

fn repl() -> Result<(), Box<dyn std::error::Error>> {
    let reader = Interface::new(PROMPT).unwrap();
    let mut env = env::Env::new();

    reader.set_prompt(PROMPT).as_ref().unwrap();

    while let ReadResult::Input(input) = reader.read_line().unwrap() {
        if input.eq("exit") {
            break;
        }
        let val = match eval::eval(input.as_ref(), &mut env) {
            Ok(data) => data,
            Err(err) => {
                println!("Error: {}", err);
                continue;
            }
        };

        match val {
            Object::Void => {}
            Object::Integer(n) => println!("{}", n),
            Object::Bool(b) => println!("{}", b),
            Object::Symbol(s) => println!("{}", s),
            Object::Lambda(params, body) => {
                println!("Lambda(");
                for param in params {
                    println!("{} ", param);
                }
                println!(")");
                for expr in body {
                    println!(" {}", expr);
                }
            }
            _ => println!("{}", val),
        }
    }

    println!("Good bye");
 
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        repl()?;
    } else if args.len() >= 2 {
        let file = &args[1];
        let parts = file.split('.').collect::<Vec<&str>>();
        let ext = parts.get(1).expect("Filename not correctly").to_string();

        if ext != "lisp" && ext != "cl" {
            panic!("extension not correct, file not valid");
        }

        let contents = std::fs::read_to_string(file)
            .expect("Should have been able to read the file");

        let mut env = env::Env::new();
        for line in contents.lines() {
            if line.is_empty() {
                continue;
            }
            let val = eval::eval(line, &mut env)?;
            match val {
                Object::Void => {}
                Object::Integer(n) => println!("{}", n),
                Object::Bool(b) => println!("{}", b),
                Object::Symbol(s) => println!("{}", s),
                Object::Lambda(params, body) => {
                    println!("Lambda(");
                    for param in params {
                        println!("{} ", param);
                    }
                    println!(")");
                    for expr in body {
                        println!(" {}", expr);
                    }
                }
                _ => println!("{}", val),
            }
        }
    }

    Ok(())

}
