use std::fmt;

use rustyline::error::ReadlineError;
use rustyline::Editor;

#[derive(Debug)]
enum BuiltIn {
    Add,
    Sub,
    Mul,
    Div,
}

impl fmt::Display for BuiltIn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuiltIn::Add => { write!(f, "+") }
            BuiltIn::Sub => { write!(f, "-") }
            BuiltIn::Mul => { write!(f, "*") }
            BuiltIn::Div => { write!(f, "/") }
        }
    }
}

#[derive(Debug)]
enum Value {
    Num(i64),
    Str(String),
    BuiltIn(BuiltIn),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Num(i)      => { write!(f, "num({})", i) }
            Value::Str(s)      => { write!(f, "str({})", s) }
            Value::BuiltIn(bi) => { write!(f, "builtin({})", bi) }
        }
    }
}

#[derive(Debug)]
enum Expr {
    Fn,
    Value(Value),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Fn       => { write!(f, "Fn") }
            Expr::Value(v) => { write!(f, "Value({})", v) }
        }
    }
}

#[derive(Clone,Copy,Debug)]
enum ParserState {
    Normal,
    String
}

fn lexer(input: &str) -> Expr {
    let mut stack: Vec<Expr> = Vec::new();
    let mut ps = ParserState::Normal;
    let it = &mut input.chars().peekable();
    while let Some(&c) = it.peek() {
        match (c, ps) {
            (_,   ParserState::Normal) if c.is_ascii_whitespace() => { }
            ('+', ParserState::Normal)   => { stack.push(Expr::Value(Value::BuiltIn(BuiltIn::Add))) }
            ('-', ParserState::Normal)   => { stack.push(Expr::Value(Value::BuiltIn(BuiltIn::Sub))) }
            ('*', ParserState::Normal)   => { stack.push(Expr::Value(Value::BuiltIn(BuiltIn::Mul))) }
            ('/', ParserState::Normal)   => { stack.push(Expr::Value(Value::BuiltIn(BuiltIn::Div))) }
            ('"', ParserState::Normal)   => {
                ps = ParserState::String;
                it.next();
                let s = it.take_while(|c2| *c2 != '"').collect::<String>();
                it.next();
                stack.push(Expr::Value(Value::Str(s)));
            }
            (_, _) => {
                panic!("Unrecognized char {} in state {:?}", c, ps);
            }
        }
    }

    println!("stack: {:?}", stack);

    Expr::Fn
}

fn eval(_input: &Expr) -> Value {
    Value::Num(1)
}

fn main() {
    let mut rl = Editor::<()>::new();
    match dirs::home_dir() {
        Some(mut pathbuf) => {
            pathbuf.push(".lispedit_history");
            if rl.load_history(&pathbuf).is_err() {
                println!("No previous history.")
            }
        }

        None => {
            println!("Could not find home dir");
        }
    }

    loop {
        let readline = rl.readline("lispedit> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let r = read(&line);
                let _e = eval(&r);

                println!("Line: {}", line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
}
