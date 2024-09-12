use std::mem;

use super::Expression;

pub type Double = f32;

#[derive(Debug, Clone)]
pub enum Value {
    /// A numeric value. All numbers are represented as doubles
    Number(Double),
    Str(String),
}

impl Expression for Value {
    fn evaluate(&self) -> Value {
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

    pub fn to_string(&self) -> String {
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
        match self {
            Value::Number(_) => true,
            _ => false,
        }
    }
}
