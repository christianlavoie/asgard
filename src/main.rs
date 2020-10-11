use std::collections::HashMap;

use rustyline::error::ReadlineError;
use rustyline::Editor;

mod lexer;
mod parser;
mod eval;

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
                let r = lexer::Lexer {
                    input: &mut line.chars().peekable()
                };

                let p = parser::Parser {
                    input: &mut r.peekable()
                };

                let mut env = eval::Environment {
                    values: HashMap::<String, &parser::Value>::new()
                };

                eval::add_default_funcs(&mut env);

                match eval::eval(&mut env, &mut p.peekable()) {
                    Ok(v) => { println!("Got {:?}", v); }
                    Err(s) => { panic!(s); }
                }
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
