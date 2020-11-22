use std::collections::HashMap;
use std::fmt;

use pest::iterators::Pair;

use crate::Value::*;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct AsgardLispParser;

#[derive(Clone)]
pub struct NativeFn {
    pub native_fn: fn(env: &mut Environment, &[Value]) -> Value
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
    Bool(bool),
    If(Vec<Value>),
    Ident(String),
    Builtin(NativeFn),
    Nil
}

#[derive(Clone,Debug,Eq,PartialEq)]
pub struct Environment {
    pub values: HashMap<String, Value>
}

impl Environment {
    pub fn new() -> Environment {
        let mut env = Environment { values: HashMap::new() };
        add_default_funcs(&mut env);
        env
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

pub fn func_builtin_add(_env: &mut Environment, values: &[Value]) -> Value {
    let mut retval = 0;
    for v in values.iter() {
        match v {
            Value::Int(i) => { retval += i; }
            _ => { panic!("Tried to add non-numerical {:?}", v); }
        }
    }

    Value::Int(retval)
}

const BUILTIN_ADD: Value = Value::Builtin(NativeFn {
    native_fn: func_builtin_add
});

pub fn func_builtin_sub(_env: &mut Environment, values: &[Value]) -> Value {
    let mut retval;
    let mut it = values.iter();
    match it.next() {
        Some(Value::Int(i)) => { retval = *i; }
        v => { panic!("builtin func sub passed invalid arg {:?}", v); }
    }
    for v in values.iter() {
        match v {
            Value::Int(i) => { retval -= i; }
            v => { panic!("builtin func sub passed invalid arg {:?}", v); }
        }
    }

    Value::Int(retval)
}

const BUILTIN_SUB: Value = Value::Builtin(NativeFn {
    native_fn: func_builtin_sub
});

pub fn func_builtin_mul(_env: &mut Environment, values: &[Value]) -> Value {
    let mut retval = 1;
    for v in values.iter() {
        match v {
            Value::Int(i) => { retval *= i; }
            _ => { panic!("Tried to mul non-numerical {:?}", v); }
        }
    }

    Value::Int(retval)
}

const BUILTIN_MUL: Value = Value::Builtin(NativeFn {
    native_fn: func_builtin_mul
});

pub fn func_builtin_div(_env: &mut Environment, values: &[Value]) -> Value {
    let mut retval;
    let mut it = values.iter();
    match it.next() {
        Some(Value::Int(i)) => { retval = *i; }
        v => { panic!("builtin func div passed invalid arg {:?}", v); }
    }
    for v in values.iter() {
        match v {
            Value::Int(i) => { retval /= i; }
            v => { panic!("builtin func div passed invalid arg {:?}", v); }
        }
    }

    Value::Int(retval)
}

const BUILTIN_DIV: Value = Value::Builtin(NativeFn {
    native_fn: func_builtin_div
});

pub fn func_builtin_eq(_env: &mut Environment, values: &[Value]) -> Value {
    let val: &Value;
    let mut it = values.iter();
    match it.next() {
        Some(v) => { val = v; }
        None => { return Bool(true); }
    }
    for v in values.iter() {
        if val != v {
            return Bool(false);
        }
    }

    Value::Bool(true)
}

const BUILTIN_EQ: Value = Value::Builtin(NativeFn {
    native_fn: func_builtin_eq
});

pub fn parse_toplevel(env: &mut Environment, line: &str) {
    for pair in AsgardLispParser::parse(Rule::toplevel, line).unwrap_or_else(|e| panic!("{}", e)) {
        let v = eval(env, &pair);
        println!("{:?}", v);
    }
}

pub fn add_default_funcs(env: &mut Environment) {
    env.values.insert(String::from("+"), BUILTIN_ADD);
    env.values.insert(String::from("-"), BUILTIN_SUB);
    env.values.insert(String::from("*"), BUILTIN_MUL);
    env.values.insert(String::from("/"), BUILTIN_DIV);
    env.values.insert(String::from("eq?"), BUILTIN_EQ);
}

pub fn eval(env: &mut Environment, pair: &Pair<Rule>) -> Value {
    match pair.as_rule() {
        Rule::true_  => { Bool(true) }
        Rule::false_ => { Bool(false) }
        Rule::num    => { Int(pair.as_str().parse::<i64>().unwrap()) }

        Rule::ident => {
            let ident = pair.as_str();
            match env.values.get(ident) {
                Some(v) => (*v).clone(),
                None => panic!("Unknown ident: {:?}", ident)
            }
        }

        Rule::list => {
            let mut lst = Vec::new();
            for inner in pair.clone().into_inner() {
                lst.push(eval(env, &inner));
            }

            let head = lst.remove(0);
            match head {
                Builtin(f) => { (f.native_fn)(env, &lst) }
                _          => { panic!("{:?} not a function", head) }
            }
        }

        Rule::assert => {
            let mut pairs = pair.clone().into_inner();
            let cond = eval(env, &pairs.next().unwrap());
            match cond {
                Bool(true) => { return Bool(true); }
                _ => { panic!("Assert failure: {:?}", cond) }
            }
        }

        Rule::def => {
            let mut pairs = pair.clone().into_inner();
            let ident = String::from(pairs.next().unwrap().as_str());
            let value = eval(env, &pairs.next().unwrap());
            env.values.insert(ident, value.clone());
            return value
        }

        Rule::do_ => {
            let mut v = Nil;
            for inner in pair.clone().into_inner() {
                v = eval(env, &inner);
            }
            return v
        }

        Rule::if_ => {
            let mut pairs = pair.clone().into_inner();
            let cond = eval(env, &pairs.next().unwrap());
            let t = pairs.next();
            let f = pairs.next();
            match (cond.clone(), t, f) {
                (Bool(true), Some(v), _)  => eval(env, &v),
                (Bool(false), _, Some(v)) => eval(env, &v),
                (Bool(true), None, _)     => {
                    panic!("if branch missing a needed true alternative");
                }
                (Bool(false), _, None)     => {
                    panic!("if branch missing a needed false alternative");
                }
                _ => {
                    panic!("Not a boolean: {:?}", cond);
                }
            }
        }

        _ => panic!("Unreachable: {:?}", pair)
    }
}
