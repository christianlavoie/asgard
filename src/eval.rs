use std::collections::HashMap;
use std::iter::Peekable;

use crate::parser::*;
use crate::parser::Value::*;

#[derive(Clone,Debug,Eq,PartialEq)]
pub struct Environment<'a> {
    pub values: HashMap<String, &'a Value>
}

pub fn func_builtin_add(values: &[Value]) -> Value {
    let it = values.iter();
    let mut sum = 0;
    for v in it {
        match v {
            Value::Int(i) => { sum += i; }
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

fn eval_value(env: &mut Environment, value: &Value) -> Result<Value, String> {
    println!("Evaluating {:?}", value);

    match value {
        List(v) => {
            match v.get(0) {
                Some(Builtin(func)) => {
                    let mut args = Vec::<Value>::new();
                    for arg in &v[1..] { args.push(eval_value(env, arg)?); }
                    Ok((func.native_fn)(&args))
                }

                Some(Ident(ident)) => {
                    match env.values.get(ident) {
                        Some(Value::Builtin(func)) => {
                            let mut args = Vec::<Value>::new();
                            for arg in &v[1..] { args.push(eval_value(env, arg)?); }
                            println!("Calling {} with {:?}", ident, args);

                            Ok((func.native_fn)(&args))
                        }

                        notfunc => {
                            Err(format!("Got non-func out of environment for {}: {:?}", ident, notfunc))
                        }
                    }
                }

                notfunc => {
                    Err(format!("Unimplemented: {:?}", notfunc))
                }
            }
        }

        Int(i) => {
            Ok(Int(*i))
        }

        notlist => {
            Err(format!("Unimplemented: {:?}", notlist))
        }
    }
}

pub fn eval<'e>(env: &mut Environment, forms: &mut Peekable<Parser<'e>>) -> Result<Vec::<Value>, String> {
    let mut retval = Vec::<Value>::new();

    for form in forms {
        match eval_value(env, &form) {
            Ok(v) => { retval.push(v); }
            Err(v) => { return Err(v); }
        }
    }

    Ok(retval)
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

        let s = "(+ (+ 1 1) (+ 2 3) 13 22)";

        let lexer = Lexer {
            input: &mut s.chars().peekable()
        };

        let parser = Parser {
            input: &mut lexer.peekable()
        };

        let expected = Ok(vec![Value::Int(42)]);
        let actual = eval(&mut env, &mut parser.peekable());

        assert_eq!(expected, actual);
    }
}
