use std::collections::HashMap;
use crate::types::{Environment, Expression};

pub(crate) fn gen_default_env<'a>() -> Environment<'a> {
    let mut environment: HashMap<String, Expression> = HashMap::new();

    environment.insert(String::from("+"), Expression::Func(|input| {
        if input.len() != 2 { panic!("+ expected two arguments") }
        let numbers = parse_float_list(input);
        Expression::Float(numbers[0] + numbers[1])
    }));

    environment.insert(String::from("-"), Expression::Func(|input| {
        if input.len() != 2 { panic!("- expected two arguments") }
        let numbers = parse_float_list(input);
        Expression::Float(numbers[0] - numbers[1])
    }));

    environment.insert(String::from("*"), Expression::Func(|input| {
        if input.len() != 2 { panic!("* expected two arguments") }
        let numbers = parse_float_list(input);
        Expression::Float(numbers[0] * numbers[1])
    }));

    environment.insert(String::from("/"), Expression::Func(|input| {
        if input.len() != 2 { panic!("/ expected two arguments") }
        let numbers = parse_float_list(input);
        Expression::Float(numbers[0] / numbers[1])
    }));

    environment.insert(String::from("="), Expression::Func(|input| {
        if input.len() != 2 { panic!("= expected two arguments") }
        let numbers = parse_float_list(input);
        Expression::Bool(numbers[0] == numbers[1])
    }));

    environment.insert(String::from(">"), Expression::Func(|input| {
        if input.len() != 2 { panic!("> expected two arguments") }
        let numbers = parse_float_list(input);
        Expression::Bool(numbers[0] > numbers[1])
    }));

    environment.insert(String::from("<"), Expression::Func(|input| {
        if input.len() != 2 { panic!("< expected two arguments") }
        let numbers = parse_float_list(input);
        Expression::Bool(numbers[0] < numbers[1])
    }));

    environment.insert(String::from("<="), Expression::Func(|input| {
        if input.len() != 2 { panic!("<= expected two arguments") }
        let numbers = parse_float_list(input);
        Expression::Bool(numbers[0] <= numbers[1])
    }));

    environment.insert(String::from(">="), Expression::Func(|input| {
        if input.len() != 2 { panic!(">= expected two arguments") }
        let numbers = parse_float_list(input);
        Expression::Bool(numbers[0] >= numbers[1])
    }));

    Environment { data: environment, parent: None }
}

fn parse_float_list(input: Vec<Expression>) -> Vec<f64> {
    let mut numbers: Vec<f64> = Vec::new();
    for expression in input {
        if let Expression::Float(value) = expression {
            numbers.push(value);
        } else {
            panic!("expected numbers")
        }
    }
    numbers
}
