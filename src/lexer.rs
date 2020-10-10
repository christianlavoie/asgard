pub mod lexer {
    use std::iter::Peekable;
    use std::str::Chars;

    pub struct Lexer<'e> {
        pub input: &'e mut Peekable<Chars<'e>>,
    }

    #[derive(Clone,Debug,Eq,PartialEq)]
    pub enum LexItem {
        OpenParen,
        CloseParen,
        Ident(String),
        StringLit(String)
    }

    impl<'e> Iterator for Lexer<'e> {
        type Item = LexItem;

        fn next(&mut self) -> Option<LexItem> {
            let it = &mut self.input;
            if let Some(c) = it.skip_while(|c| c.is_ascii_whitespace()).next() {
                match c {
                    '(' => { Some(LexItem::OpenParen) }
                    ')' => { Some(LexItem::CloseParen) }
                    '"' => {
                        let s = it.take_while(|c2| *c2 != '"').collect::<String>();
                        Some(LexItem::StringLit(s))
                    }

                    _ => {
                        let mut ident = String::from(c);
                        while let Some(c) = it.peek() {
                            if c.is_ascii_whitespace() || *c == '(' || *c == ')' {
                                break;
                            }

                            ident.push(*c);
                            it.next();
                        }
                        Some(LexItem::Ident(ident))
                    }
                }
            } else {
                None
            }
        }
    }
}

mod tests {
    use crate::lexer::lexer::*;
    use crate::lexer::lexer::LexItem::*;

    #[test]
    fn basic_lexing() {
        let s = String::from("(+ - \"123\" 123)");
        let l = Lexer {
            input: &mut s.chars().peekable()
        };

        let expected = vec![OpenParen, Ident("+".to_string()), Ident("-".to_string()), StringLit("123".to_string()), Ident("123".to_string()), CloseParen];
        let actual = l.collect::<Vec<_>>();
        assert_eq!(expected, actual);
    }
}
