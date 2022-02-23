use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub(crate) enum Expression {
    List(Vec<Expression>),
    Float(f64),
    String(String),
    Bool(bool),
    Func(fn(Vec<Expression>) -> Expression),
    Lambda(Lambda),
}

#[derive(Debug, Clone)]
pub(crate) struct Lambda {
    pub(crate) params: Rc<Expression>,
    pub(crate) body: Rc<Expression>,
}

pub(crate) struct Environment<'a> {
    pub(crate) data: HashMap<String, Expression>,
    pub(crate) parent: Option<&'a Environment<'a>>,
}

impl Environment<'_> {
    pub(crate) fn get(&self, k: &str) -> Option<Expression> {
        match self.data.get(k) {
            Some(val) => Some(val.clone()),
            None => {
                match self.parent {
                    Some(val) => val.get(k),
                    None => None
                }
            }
        }
    }
}