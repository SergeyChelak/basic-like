use super::{Double, Expression, Value};

/// A numeric value. All numbers are represented as doubles
#[derive(Debug, Clone)]
pub struct NumberValue {
    value: Double,
}

impl NumberValue {
    pub fn new(value: Double) -> Self {
        Self { value }
    }

    pub fn with_bool(value: bool) -> Self {
        Self {
            value: if value { 1.0 } else { 0.0 },
        }
    }
}

impl Value for NumberValue {
    fn to_string(&self) -> String {
        self.value.to_string()
    }

    fn to_number(&self) -> Double {
        self.value
    }
}

impl Expression for NumberValue {
    fn evaluate(&self) -> Box<dyn Value + 'static> {
        Box::new(self.clone())
    }
}

/// String value
#[derive(Debug, Clone)]
pub struct StringValue {
    value: String,
}

impl StringValue {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl Value for StringValue {
    fn to_string(&self) -> String {
        self.value.clone()
    }

    fn to_number(&self) -> Double {
        self.value.parse::<Double>().unwrap_or_default()
    }
}
impl Expression for StringValue {
    fn evaluate(&self) -> Box<dyn Value + 'static> {
        Box::new(self.clone())
    }
}
