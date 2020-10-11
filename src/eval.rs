use std::collections::HashMap;
use std::iter::Peekable;

use crate::parser::*;
use crate::parser::Value::*;

#[derive(Clone,Debug,Eq,PartialEq)]
pub struct Environment<'a> {
    pub values: HashMap<String, &'a Value>
}

pub fn func_builtin_add(values: &[Value]) -> Value {
    let mut it = values.iter();
    let mut sum = 0;
    while let Some(v) = it.next() {
        match v {
            Value::Int(i) => { sum = sum + i; }
            _ => { panic!("Tried to add non-numerical {:?}", v); }
        }
    }

    Value::Int(sum)
}

const BUILTIN_ADD: Value = Value::Builtin(NativeFn {
    native_fn: func_builtin_add
});

pub fn add_default_funcs(env: &mut Environment) {
    env.values.insert(String::from("+"), &BUILTIN_ADD);
}

pub fn eval<'e>(env: &mut Environment, forms: &mut Peekable<Parser<'e>>) -> Result<Value, String> {
    while let Some(form) = forms.next() {
        match form {
            List(v) => {
                match v.get(0) {
                    Some(Builtin(func)) => {
                        return Ok((func.native_fn)(&v[1..]));
                    }

                    Some(Ident(ident)) => {
                        match env.values.get(ident) {
                            Some(Value::Builtin(func)) => {
                                return Ok((func.native_fn)(&v[1..]));
                            }

                            notfunc => {
                                return Err(format!("Got non-func out of environment for {}: {:?}", ident, notfunc));
                            }
                        }
                    }

                    notfunc => {
                        return Err(format!("Unimplemented: {:?}", notfunc))
                    }
                }
            }

            notlist => {
                return Err(format!("Unimplemented: {:?}", notlist))
            }
        }
    }

    Ok(Int(42))
}

#[cfg(test)]
mod tests {
    use crate::lexer::*;
    use crate::parser::*;
    use crate::eval::*;

    #[test]
    fn basic_form() {
        let mut env = Environment {
            values: HashMap::<String, &Value>::new()
        };
        add_default_funcs(&mut env);

        let s = "(+ 1 2)";

        let lexer = Lexer {
            input: &mut s.chars().peekable()
        };

        let parser = Parser {
            input: &mut lexer.peekable()
        };

        let expected = Ok(Value::Int(3));
        let actual = eval(&mut env, &mut parser.peekable());

        assert_eq!(expected, actual);
    }
}
