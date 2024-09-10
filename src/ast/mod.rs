mod operator_expr;
mod values;

pub trait Statement {
    fn execute(&mut self);
}

pub trait Expression {
    fn evaluate(&self) -> Box<dyn Value + 'static>;
}

type Double = f32;
pub trait Value: Expression {
    fn to_string(&self) -> String;

    fn to_number(&self) -> Double;
}
