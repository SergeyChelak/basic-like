use super::Expression;

pub enum Statement {
    Assign {
        name: String,
        value: Box<dyn Expression>,
    },
    IfThen {
        condition: Box<dyn Expression>,
        label: String,
    },
    Goto {
        label: String,
    },
    Print {
        expression: Box<dyn Expression>,
    },
    Input {
        name: String,
    },
}

impl Statement {
    pub fn assign(name: String, value: Box<dyn Expression>) -> Self {
        Self::Assign { name, value }
    }

    pub fn goto(label: String) -> Self {
        Self::Goto { label }
    }

    pub fn if_then(condition: Box<dyn Expression>, label: String) -> Self {
        Self::IfThen { condition, label }
    }

    pub fn input(name: String) -> Self {
        Self::Input { name }
    }

    pub fn print(expression: Box<dyn Expression>) -> Self {
        Self::Print { expression }
    }
}
