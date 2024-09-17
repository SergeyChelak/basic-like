use crate::interpreter::InterpreterContext;

use super::{Expression, Value};

pub struct OperatorExpression {
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
    fn evaluate(&self, context: &InterpreterContext) -> Value {
        let left_value = self.left.evaluate(context);
        let right_value = self.right.evaluate(context);
        let is_left_numeric = left_value.is_numeric();
        match self.operator {
            '=' => {
                // Coerce to the left argument's type, then compare
                let is_equal = if is_left_numeric {
                    left_value.to_number() == right_value.to_number()
                } else {
                    left_value.to_string() == right_value.to_string()
                };
                return Value::with_bool(is_equal);
            }
            '+' => {
                // Addition if the left argument is a number, otherwise do
                // string concatenation
                if is_left_numeric {
                    let sum = left_value.to_number() + right_value.to_number();
                    return Value::number(sum);
                } else {
                    let concat = left_value.to_string() + &right_value.to_string();
                    return Value::string(concat);
                }
            }
            '-' => {
                let sub = left_value.to_number() - right_value.to_number();
                return Value::number(sub);
            }
            '*' => {
                let mul = left_value.to_number() * right_value.to_number();
                return Value::number(mul);
            }
            '/' => {
                let div = left_value.to_number() / right_value.to_number();
                return Value::number(div);
            }
            '<' => {
                // Coerce to the left argument's type, then compare
                let val = if is_left_numeric {
                    left_value.to_number() < right_value.to_number()
                } else {
                    left_value.to_string() < right_value.to_string()
                };
                return Value::with_bool(val);
            }
            '>' => {
                // Coerce to the left argument's type, then compare.
                let val = if is_left_numeric {
                    left_value.to_number() > right_value.to_number()
                } else {
                    left_value.to_string() > right_value.to_string()
                };
                return Value::with_bool(val);
            }
            _ => {
                panic!("Unknown operator {}", self.operator)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn operator_expr_equals_numbers() {
        let left = Box::new(Value::number(10.0));
        let right = Box::new(Value::number(10.0));
        let operator = '=';
        let op_expr = OperatorExpression::new(left, operator, right);
        let context = InterpreterContext::default();
        let val = op_expr.evaluate(&context);
        assert!(val.is_numeric());
        assert_eq!(val.to_number(), 1.0);
    }

    #[test]
    fn operator_expr_not_equals_numbers() {
        let left = Box::new(Value::number(1.0));
        let right = Box::new(Value::number(2.0));
        let operator = '=';
        let op_expr = OperatorExpression::new(left, operator, right);
        let context = InterpreterContext::default();
        let val = op_expr.evaluate(&context);
        assert_eq!(val.to_number(), 0.0);
    }

    #[test]
    fn operator_expr_equals_number_string() {
        let left = Box::new(Value::number(10.0));
        let right = Box::new(Value::string("10".to_string()));
        let operator = '=';
        let op_expr = OperatorExpression::new(left, operator, right);
        let context = InterpreterContext::default();
        let val = op_expr.evaluate(&context);
        assert_eq!(val.to_number(), 1.0);
    }

    #[test]
    fn operator_expr_not_equals_number_string() {
        let left = Box::new(Value::number(10.0));
        let right = Box::new(Value::string("11".to_string()));
        let operator = '=';
        let op_expr = OperatorExpression::new(left, operator, right);
        let context = InterpreterContext::default();
        let val = op_expr.evaluate(&context);
        assert_eq!(val.to_number(), 0.0);
    }

    #[test]
    fn operator_expr_plus_num_num() {
        let left = Box::new(Value::number(10.0));
        let right = Box::new(Value::number(21.0));
        let operator = '+';
        let op_expr = OperatorExpression::new(left, operator, right);
        let context = InterpreterContext::default();
        let val = op_expr.evaluate(&context);
        assert!(val.is_numeric());
        assert_eq!(val.to_number(), 31.0);
    }

    #[test]
    fn operator_expr_plus_str_str() {
        let left = Box::new(Value::string("abc".to_string()));
        let right = Box::new(Value::string("def".to_string()));
        let operator = '+';
        let op_expr = OperatorExpression::new(left, operator, right);
        let context = InterpreterContext::default();
        let val = op_expr.evaluate(&context);
        assert!(!val.is_numeric());
        assert_eq!(val.to_string(), "abcdef");
    }

    #[test]
    fn operator_expr_plus_num_str() {
        let left = Box::new(Value::number(10.0));
        let right = Box::new(Value::string("20".to_string()));
        let operator = '+';
        let op_expr = OperatorExpression::new(left, operator, right);
        let context = InterpreterContext::default();
        let val = op_expr.evaluate(&context);
        assert!(val.is_numeric());
        assert_eq!(val.to_number(), 30.0);
    }

    #[test]
    #[should_panic]
    fn operator_expr_plus_num_str_panic() {
        let left = Box::new(Value::number(10.0));
        let right = Box::new(Value::string("abc".to_string()));
        let operator = '+';
        let op_expr = OperatorExpression::new(left, operator, right);
        let context = InterpreterContext::default();
        let _ = op_expr.evaluate(&context);
    }

    #[test]
    fn operator_expr_plus_str_num() {
        let left = Box::new(Value::string("20".to_string()));
        let right = Box::new(Value::number(10.0));
        let operator = '+';
        let op_expr = OperatorExpression::new(left, operator, right);
        let context = InterpreterContext::default();
        let val = op_expr.evaluate(&context);
        assert!(!val.is_numeric());
        assert_eq!(val.to_string(), "2010");
    }

    #[test]
    fn operator_expr_minus_num_num() {
        let left = Box::new(Value::number(21.0));
        let right = Box::new(Value::number(10.0));
        let operator = '-';
        let op_expr = OperatorExpression::new(left, operator, right);
        let context = InterpreterContext::default();
        let val = op_expr.evaluate(&context);
        assert!(val.is_numeric());
        assert_eq!(val.to_number(), 11.0);
    }

    #[test]
    fn operator_expr_minus_str_num() {
        let left = Box::new(Value::string("20".to_string()));
        let right = Box::new(Value::number(10.0));
        let operator = '-';
        let op_expr = OperatorExpression::new(left, operator, right);
        let context = InterpreterContext::default();
        let val = op_expr.evaluate(&context);
        assert_eq!(val.to_number(), 10.0);
    }

    #[test]
    #[should_panic]
    fn operator_expr_minus_str_num_fail() {
        let left = Box::new(Value::string("a20".to_string()));
        let right = Box::new(Value::number(10.0));
        let operator = '-';
        let op_expr = OperatorExpression::new(left, operator, right);
        let context = InterpreterContext::default();
        let _ = op_expr.evaluate(&context);
    }

    #[test]
    fn operator_expr_mul_num_num() {
        let left = Box::new(Value::number(21.0));
        let right = Box::new(Value::number(10.0));
        let operator = '*';
        let op_expr = OperatorExpression::new(left, operator, right);
        let context = InterpreterContext::default();
        let val = op_expr.evaluate(&context);
        assert!(val.is_numeric());
        assert_eq!(val.to_number(), 210.0);
    }

    #[test]
    fn operator_expr_mul_str_num() {
        let left = Box::new(Value::string("20".to_string()));
        let right = Box::new(Value::number(10.0));
        let operator = '*';
        let op_expr = OperatorExpression::new(left, operator, right);
        let context = InterpreterContext::default();
        let val = op_expr.evaluate(&context);
        assert_eq!(val.to_number(), 200.0);
    }

    #[test]
    #[should_panic]
    fn operator_expr_mul_str_num_fail() {
        let left = Box::new(Value::string("a20".to_string()));
        let right = Box::new(Value::number(10.0));
        let operator = '*';
        let op_expr = OperatorExpression::new(left, operator, right);
        let context = InterpreterContext::default();
        let _ = op_expr.evaluate(&context);
    }

    #[test]
    fn operator_expr_div_num_num() {
        let left = Box::new(Value::number(20.0));
        let right = Box::new(Value::number(10.0));
        let operator = '/';
        let op_expr = OperatorExpression::new(left, operator, right);
        let context = InterpreterContext::default();
        let val = op_expr.evaluate(&context);
        assert!(val.is_numeric());
        assert_eq!(val.to_number(), 2.0);
    }

    #[test]
    fn operator_expr_div_str_num() {
        let left = Box::new(Value::string("20".to_string()));
        let right = Box::new(Value::number(10.0));
        let operator = '/';
        let op_expr = OperatorExpression::new(left, operator, right);
        let context = InterpreterContext::default();
        let val = op_expr.evaluate(&context);
        assert_eq!(val.to_number(), 2.0);
    }

    #[test]
    #[should_panic]
    fn operator_expr_div_str_num_fail() {
        let left = Box::new(Value::string("a20".to_string()));
        let right = Box::new(Value::number(10.0));
        let operator = '/';
        let op_expr = OperatorExpression::new(left, operator, right);
        let context = InterpreterContext::default();
        let _ = op_expr.evaluate(&context);
    }

    #[test]
    fn operator_expr_less_num_num() {
        let left = Box::new(Value::number(21.0));
        let right = Box::new(Value::number(10.0));
        let operator = '<';
        let op_expr = OperatorExpression::new(left, operator, right);
        let context = InterpreterContext::default();
        let val = op_expr.evaluate(&context);
        assert!(val.is_numeric());
        assert_eq!(val.to_number(), 0.0);
    }

    #[test]
    fn operator_expr_less_num_str() {
        let left = Box::new(Value::number(21.0));
        let right = Box::new(Value::string("10".to_string()));
        let operator = '<';
        let op_expr = OperatorExpression::new(left, operator, right);
        let context = InterpreterContext::default();
        let val = op_expr.evaluate(&context);
        assert!(val.is_numeric());
        assert_eq!(val.to_number(), 0.0);
    }

    #[test]
    fn operator_expr_less_str_str() {
        let left = Box::new(Value::string("abc".to_string()));
        let right = Box::new(Value::string("bbc".to_string()));
        let operator = '<';
        let op_expr = OperatorExpression::new(left, operator, right);
        let context = InterpreterContext::default();
        let val = op_expr.evaluate(&context);
        assert!(val.is_numeric());
        assert_eq!(val.to_number(), 1.0);
    }

    #[test]
    fn operator_expr_greater_num_str() {
        let left = Box::new(Value::number(21.0));
        let right = Box::new(Value::string("10".to_string()));
        let operator = '>';
        let op_expr = OperatorExpression::new(left, operator, right);
        let context = InterpreterContext::default();
        let val = op_expr.evaluate(&context);
        assert!(val.is_numeric());
        assert_eq!(val.to_number(), 1.0);
    }

    #[test]
    fn operator_expr_greater_str_str() {
        let left = Box::new(Value::string("abc".to_string()));
        let right = Box::new(Value::string("bbc".to_string()));
        let operator = '>';
        let op_expr = OperatorExpression::new(left, operator, right);
        let context = InterpreterContext::default();
        let val = op_expr.evaluate(&context);
        assert!(val.is_numeric());
        assert_eq!(val.to_number(), 0.0);
    }
}
