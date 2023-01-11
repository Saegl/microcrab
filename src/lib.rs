use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Value {
    data: f64,
}

impl Value {
    pub fn from(data: f64) -> Self {
        Self { data }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Value(data={}, grad=?)", self.data)
    }
}

impl Add<Value> for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::from(self.data + rhs.data)
    }
}

impl Add<f64> for Value {
    type Output = Value;

    fn add(self, rhs: f64) -> Self::Output {
        Self::Output::from(self.data + rhs)
    }
}

impl Mul<Value> for Value {
    type Output = Value;

    fn mul(self, rhs: Value) -> Self::Output {
        Self::Output::from(self.data * rhs.data)
    }
}

impl Mul<f64> for Value {
    type Output = Value;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output::from(self.data * rhs)
    }
}
