use std::str::Chars;

use rustyline::error::ReadlineError;
use rustyline::Editor;

#[derive(Clone,Copy,Debug)]
enum ParserState {
    Normal,
    String
}

struct Lexer<'e> {
    input: &'e mut Chars<'e>,
    parser_state: ParserState
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
        match (it.next(), self.parser_state) {
            (None,      _)                     => { None }
            (Some(c),   ParserState::Normal) if c.is_ascii_whitespace() => { self.next() }
            (Some('+'), ParserState::Normal)   => { Some(LexItem::Ident(String::from("+"))) }
            (Some('-'), ParserState::Normal)   => { Some(LexItem::Ident(String::from("-"))) }
            (Some('*'), ParserState::Normal)   => { Some(LexItem::Ident(String::from("*"))) }
            (Some('/'), ParserState::Normal)   => { Some(LexItem::Ident(String::from("/"))) }
            (Some('('), ParserState::Normal)   => { Some(LexItem::OpenParen) }
            (Some(')'), ParserState::Normal)   => { Some(LexItem::CloseParen) }
            (Some('"'), ParserState::Normal)   => {
                self.parser_state = ParserState::String;
                it.next();
                let s = it.take_while(|c2| *c2 != '"').collect::<String>();
                it.next();
                Some(LexItem::StringLit(s))
            }
            (c, ps) => {
                panic!("Unrecognized char {:?} in state {:?}", c, ps);
            }
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
                    input: &mut line.chars(),
                    parser_state: ParserState::Normal
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
