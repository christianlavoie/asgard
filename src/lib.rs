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
    pub native_fn: fn(env: &mut Environment, &Vec<Value>) -> Value
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
    Builtin(NativeFn)
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

pub fn func_builtin_add(_env: &mut Environment, values: &Vec<Value>) -> Value {
    let mut sum = 0;
    for v in values.iter() {
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

pub fn parse_toplevel(env: &mut Environment, line: &str) {
    for pair in AsgardLispParser::parse(Rule::toplevel, line).unwrap_or_else(|e| panic!("{}", e)) {
        let v = eval(env, &pair);
        println!("{:?}", v);
    }
}

pub fn add_default_funcs(env: &mut Environment) {
    env.values.insert(String::from("+"), BUILTIN_ADD);
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
