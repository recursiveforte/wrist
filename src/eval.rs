use std::collections::HashMap;
use std::rc::Rc;
use crate::types::{Environment, Expression, Lambda};

pub(crate) fn eval(e: &Expression, env: &mut Environment) -> Expression {
    match e {
        Expression::Float(_v) => e.clone(),
        Expression::Bool(_v) => e.clone(),
        Expression::String(value) => env.get(&value.to_lowercase()).expect("value not found in environment").clone(),
        Expression::List(list) => {
            let first_entry = list.first().unwrap();
            let args = &list[1..];
            if let Expression::String(value) = first_entry {
                match value.as_str() {
                    "if" => eval_if(args, env),
                    "define" => eval_define(args, env),
                    "lambda" => eval_lambda(args),
                    _ => {
                        let first_eval = eval(first_entry, env);
                        match first_eval {
                            Expression::Func(function) => {
                                let args_eval = args
                                    .iter()
                                    .map(|x| eval(x, env))
                                    .collect::<Vec<Expression>>();
                                function(args_eval)
                            }
                            Expression::Lambda(lambda) =>
                                eval(&lambda.body,
                                     &mut lambda_env(lambda.params, args, env)),
                            _ => panic!()
                        }
                    }
                }
            } else {
                eval(&first_entry, env)
            }
        }
        _ => panic!()
    }
}

fn eval_if(args: &[Expression], env: &mut Environment) -> Expression {
    if args.len() != 3 { panic!("if expected three arguments") };
    let test = args.first().unwrap();
    let test_eval = eval(test, env);
    if let Expression::Bool(bool) = test_eval {
        let form = if bool { 1 } else { 2 };
        eval(args.get(form).unwrap(), env)
    } else {
        panic!("if statement expects a boolean")
    }
}

fn eval_define(args: &[Expression], env: &mut Environment) -> Expression {
    if args.len() != 2 { panic!("def expected two arguments") };
    let key: String;
    if let Expression::String(value) = args.first().unwrap() {
        key = value.clone();
    } else {
        panic!()
    }
    let value = eval(args.get(1).unwrap(), env);
    let key_clone = key.chars().map(|x| {
        if x.is_ascii_alphanumeric() {
            x
        } else if ['!', '$', '%', '&', '*', '+', '-', '.', '/', ':', '<', '=', '>', '?', '@', '^', '_', '~'].contains(&x) {
            x
        } else {
            panic!("identifiers must only contain valid extended alphanumeric characters")
        }
    }).collect::<String>().clone();
    if key_clone.chars().nth(0).unwrap().is_numeric() {
        panic!("identifier must not start with a number")
    }
    env.data.insert(key.to_lowercase(), value);
    Expression::String(key_clone)
}

fn eval_lambda(args: &[Expression]) -> Expression {
    if args.len() != 2 { panic!("lambda definition expects two arguments") };
    Expression::Lambda(Lambda {
        body: Rc::new(args.get(1).unwrap().clone()),
        params: Rc::new(args.first().unwrap().clone()),
    })
}

fn lambda_env<'a>(params: Rc<Expression>, args: &[Expression], parent: &'a mut Environment)
                  -> Environment<'a> {
    let mut keys: Vec<String> = Vec::new();
    if let Expression::List(vector) = params.as_ref() {
        for x in vector {
            if let Expression::String(s) = x {
                keys.push(s.clone())
            }
        };
        if vector.len() != args.len() {
            panic!("number of params and arguments must match")
        }
    }
    let mut env: HashMap<String, Expression> = HashMap::new();
    for (key, value) in keys.iter().zip(args.iter().map(|x| eval(x, parent))) {
        env.insert(key.clone(), value.clone());
    }
    Environment { data: env, parent: Some(parent) }
}
