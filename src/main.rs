use rustyline::error::ReadlineError;
use rustyline::Editor;

use asgard::*;
use asgard::Value::*;

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
        let mut env = Environment::new();
        env.values.insert(String::from("a"), Int(123));
        env.values.insert(String::from("b"), Int(456));

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                parse_toplevel(&mut env, line.as_str());
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
