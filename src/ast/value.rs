use crate::interpreter::InterpreterContext;

use super::Expression;

pub type Double = f32;

#[derive(Debug, Clone)]
pub enum Value {
    /// A numeric value. All numbers are represented as doubles
    Number(Double),
    Str(String),
}

impl Expression for Value {
    fn evaluate(&self, _context: &InterpreterContext) -> Value {
        self.clone()
    }
}

impl Value {
    pub fn number(value: Double) -> Self {
        Self::Number(value)
    }

    pub fn string(value: String) -> Self {
        Self::Str(value)
    }

    pub fn with_bool(value: bool) -> Self {
        Self::Number(if value { 1.0 } else { 0.0 })
    }

    pub fn to_text(&self) -> String {
        match self {
            Value::Number(val) => val.to_string(),
            Value::Str(val) => val.clone(),
        }
    }

    pub fn to_number(&self) -> Double {
        match self {
            Value::Number(val) => *val,
            Value::Str(val) => val.parse::<Double>().unwrap(),
        }
    }

    pub fn is_numeric(&self) -> bool {
        matches!(self, Value::Number(_))
    }
}
