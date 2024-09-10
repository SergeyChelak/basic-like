use std::any::{type_name, type_name_of_val};

use crate::ast::values::NumberValue;

use super::{Expression, Value};

struct OperatorExpression {
    left: Box<dyn Expression>,
    operator: char,
    right: Box<dyn Expression>,
}

impl OperatorExpression {
    pub fn new(left: Box<dyn Expression>, operator: char, right: Box<dyn Expression>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

impl Expression for OperatorExpression {
    fn evaluate(&self) -> Box<dyn Value> {
        let left_value = self.left.evaluate();
        let right_value = self.right.evaluate();

        let is_left_numeric = type_name::<NumberValue>() == type_name_of_val(&left_value);

        match self.operator {
            '=' => {
                if is_left_numeric {
                    let val =
                        NumberValue::with_bool(left_value.to_number() == right_value.to_number());
                    return Box::new(val);
                }
            }
            _ => {}
        }
        todo!()
    }
}
