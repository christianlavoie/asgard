use rustyline::error::ReadlineError;
use rustyline::Editor;

mod lexer;
use crate::lexer::lexer::Lexer;

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
                let r = Lexer {
                    input: &mut line.chars().peekable()
                };

                println!("Line: {}", line);
                println!("Tokens: {:?}", r.collect::<Vec<_>>());
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
