use std::ops;

#[derive(Debug, Clone)]
/**
 * Enum for different data types
 */
// TODO: Seperate out into its own file
pub enum Type {
    String(String),
    Numeric(i32),
    Bool(bool),
}

impl Type {
    fn determine(value: &str) -> Option<Self> {
        let d_type: Option<Self>;
        if let Ok(num) = value.parse::<i32>() {
            d_type = Some(Type::Numeric(num))
        } else if let Ok(b) = value.parse::<bool>() {
            d_type = Some(Type::Bool(b))
        } else {
            d_type = Some(Type::String(value.to_owned()))
        }
        if !d_type.is_some() {
            return None;
        }
        return d_type;
    }

    pub fn new(value: &str) -> Self {
        let d_type = Type::determine(value);
        return d_type.expect("Unable to determine datatype.");
    }
}

impl ops::Add for Type {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Type::Numeric(first), Type::Numeric(second)) => Type::Numeric(first + second),
            (Type::String(first), Type::String(second)) => {
                Type::String(format!("{}{}", first, second))
            }
            _ => panic!("Unable to add"),
        }
    }
}

impl ops::Sub for Type {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if let (Type::Numeric(first), Type::Numeric(second)) = (self, other) {
            return Type::Numeric(first - second);
        } else {
            panic!("Cannot subtract types")
        }
    }
}

impl ops::Mul for Type {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if let (Type::Numeric(first), Type::Numeric(second)) = (self, other) {
            return Type::Numeric(first * second);
        } else {
            panic!("Cannot multiply types")
        }
    }
}

impl ops::Div for Type {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        if let (Type::Numeric(first), Type::Numeric(second)) = (self, other) {
            if second == 0 {
                panic!("Cannot divide by zero.")
            }
            return Type::Numeric(first / second);
        } else {
            panic!("Cannot divide types")
        }
    }
}
