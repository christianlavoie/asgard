use std::iter::Peekable;
use std::str::Chars;

#[derive(Clone,Debug,Eq,PartialEq)]
pub enum Lexeme {
    OpenParen,
    CloseParen,

    Fn,
    Def,

    Ident(String),
    IntLit(i64),
    StrLit(String)
}

use Lexeme::*;

pub struct Lexer<'e> {
    pub input: &'e mut Peekable<Chars<'e>>,
}

fn ends_lexeme(c: &char) -> bool {
    c.is_ascii_whitespace() || *c == '(' || *c == ')'
}

impl<'e> Iterator for Lexer<'e> {
    type Item = Lexeme;

    fn next(&mut self) -> Option<Lexeme> {
        let it = &mut self.input;
        if let Some(c) = it.skip_while(|c| c.is_ascii_whitespace()).next() {
            match c {
                '(' => { Some(OpenParen) }
                ')' => { Some(CloseParen) }
                '"' => {
                    let s = it.take_while(|c2| *c2 != '"').collect::<String>();
                    Some(StrLit(s))
                }

                _ if c.is_ascii_digit() => {
                    let mut num = String::from(c);
                    while let Some(c) = it.peek() {
                        if ends_lexeme(c) { break; }
                        num.push(*c);
                        it.next();
                    }
                    Some(IntLit(num.parse::<i64>().expect(format!("Could not parse {} to i64", num).as_str())))
                }

                _ => {
                    let mut ident = String::from(c);
                    while let Some(c) = it.peek() {
                        if ends_lexeme(c) { break; }
                        ident.push(*c);
                        it.next();
                    }

                    if ident == String::from("fn") {
                        Some(Fn)
                    } else if ident == String::from("def") {
                        Some(Def)
                    } else {
                        Some(Ident(ident))
                    }
                }
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::*;

    #[test]
    fn basic_lexing() {
        let s = String::from("(+ - \"123\" 123)");
        let l = Lexer {
            input: &mut s.chars().peekable()
        };

        let expected = vec![
            OpenParen,
            Ident("+".to_string()),
            Ident("-".to_string()),
            StrLit("123".to_string()),
            IntLit(123),
            CloseParen];

        let actual = l.collect::<Vec<_>>();
        assert_eq!(expected, actual);
    }
}
