use crate::types::Expression;

pub(crate) fn parse(input: String) -> Expression {
    treeify(&mut atomize(tokenize(input)))
}

fn tokenize(input: String) -> Vec<String> {
    let mut comment = false;
    input
        .replace("(", " ( ")
        .replace(")", " ) ")
        .chars()
        .map(|x| {
            match x {
                ';' => {
                    comment = true;
                    ' '
                }
                '\n' => {
                    comment = false;
                    ' '
                }
                _ => {
                    match comment {
                        true => ' ',
                        false => x
                    }
                }
            }
        })
        .collect::<String>()
        .split_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

fn atomize(input: Vec<String>) -> Vec<Expression> {
    let mut result: Vec<Expression> = Vec::new();
    for value in input {
        if let Ok(i) = value.parse::<f64>() {
            result.push(Expression::Float(i))
        } else if let Ok(bool) = value.parse::<bool>() {
            result.push(Expression::Bool(bool))
        } else {
            result.push(Expression::String(value))
        }
    }
    result
}


fn treeify(tokens: &mut Vec<Expression>) -> Expression {
    fn peek(tokens: &Vec<Expression>, ttype: Expression) -> bool {
        if let Expression::String(value) = &tokens[0] {
            if let Expression::String(value2) = &ttype {
                return value == value2;
            }
        }
        false
    }
    if peek(tokens, Expression::String(String::from("("))) {
        tokens.remove(0);
        let mut lbody: Vec<Expression> = Vec::new();
        while !peek(tokens, Expression::String(String::from(")"))) {
            lbody.push(treeify(tokens));
        }
        tokens.remove(0); // remove ')'
        Expression::List(lbody)
    } else {
        tokens.remove(0) // TIL this method returns the value being removed
    }
}