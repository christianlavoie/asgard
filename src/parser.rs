use std::fmt;
use std::iter::Peekable;

use crate::lexer::*;
use crate::lexer::Lexeme::*;

pub struct Parser<'e> {
    pub input: &'e mut Peekable<Lexer<'e>>,
}

#[derive(Clone)]
pub struct NativeFn {
    pub native_fn: fn(&[Value]) -> Value
}

impl fmt::Debug for NativeFn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NativeFn").finish()
    }
}

impl Eq for NativeFn { }

impl PartialEq for NativeFn {
    fn eq(&self, _other: &Self) -> bool {
        panic!("Comparing native functions!");
    }
}

#[derive(Clone,Debug,Eq,PartialEq)]
pub enum Value {
    List(Vec<Value>),
    Int(i64),
    Str(String),
    Ident(String),
    Builtin(NativeFn)
}

use Value::*;

impl<'e> Iterator for Parser<'e> {
    type Item = Value;

    fn next(&mut self) -> Option<Value> {
        let (current, next) = (self.input.by_ref().next(),
                               self.input.by_ref().peek());
        match (current, next) {
            (Some(IntLit(n)), _) => { Some(Int(n)) }
            (Some(StrLit(s)), _) => { Some(Str(s)) }
            (Some(Lexeme::Ident(c)), _) => { Some(Value::Ident(c.clone())) }

            (Some(OpenParen), Some(Fn)) => {
                panic!("Unimplemented function definition");
            }

            (Some(OpenParen), _) => {
                let mut list = Vec::<Value>::new();

                loop {
                    if self.input.by_ref().peek() == Some(&CloseParen) {
                        return Some(List(list));
                    } else if self.input.by_ref().peek() == None {
                        panic!("Missing closing parenthesis");
                    } else {
                        list.push(self.by_ref().next().unwrap());
                    }
                }
            }

            c => {
                println!("Seen: {:?}", c);
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::*;
    use crate::parser::*;

    #[test]
    fn basic_form() {
        let s = "(+ 1 2)";

        let expected = List(vec![
            Value::Ident(String::from("+")),
            Int(1),
            Int(2) ]);

        let lexer = Lexer {
            input: &mut s.chars().peekable()
        };

        let actual = Parser {
            input: &mut lexer.peekable()
        };

        assert_eq!(&expected, actual.collect::<Vec<Value>>().get(0).unwrap());
    }
}
