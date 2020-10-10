use std::str::Chars;

use rustyline::error::ReadlineError;
use rustyline::Editor;

struct Lexer<'e> {
    input: &'e mut Chars<'e>,
}

#[derive(Clone,Debug)]
enum LexItem {
    OpenParen,
    CloseParen,
    Ident(String),
    StringLit(String)
}

impl<'e> Iterator for Lexer<'e> {
    type Item = LexItem;

    fn next(&mut self) -> Option<LexItem> {
        let it = &mut self.input;
        if let Some(c) = it.next() {
            match c {
                _ if c.is_ascii_whitespace() => {
                    self.next()
                }

                '(' => {
                    Some(LexItem::OpenParen)
                }

                ')' => {
                    Some(LexItem::CloseParen)
                }

                '"' => {
                    let s = it.take_while(|c2| *c2 != '"').collect::<String>();
                    it.next(); // skip quote
                    Some(LexItem::StringLit(s))
                }

                _ => {
                    let s = &mut it.take_while(|c2| !c2.is_ascii_whitespace() && *c2 != '(' && *c2 != ')').collect::<String>();
                    s.insert(0, c);
                    Some(LexItem::Ident(s.to_string()))
                }
            }
        } else {
            None
        }
    }
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
                let r = Lexer {
                    input: &mut line.chars()
                };

                println!("Line: {}", line);
                println!("Tokens: {:?}", r.collect::<Vec<LexItem>>());
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
