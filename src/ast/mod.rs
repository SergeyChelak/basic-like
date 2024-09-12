use value::Value;

mod operator_expr;
pub mod value;

pub trait Statement {
    fn execute(&mut self);
}

pub trait Expression {
    fn evaluate(&self) -> Value;
}
