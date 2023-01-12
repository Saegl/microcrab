use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Value {
    data: f64,
}

impl Value {
    const MAX_DIFF: f64 = 0.001;

    pub fn from(data: f64) -> Self {
        Self { data }
    }

    pub fn close_to(self, x: f64) -> bool {
        (self.data - x).abs() <= Self::MAX_DIFF
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Value(data={}, grad=?)", self.data)
    }
}

impl Add<Value> for f64 {
    type Output = Value;

    fn add(self, rhs: Value) -> Self::Output {
        Self::Output::from(self + rhs.data)
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

impl Sub<Value> for f64 {
    type Output = Value;

    fn sub(self, rhs: Value) -> Self::Output {
        Self::Output::from(self - rhs.data)
    }
}

impl Sub<Value> for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::from(self.data - rhs.data)
    }
}

impl Sub<f64> for Value {
    type Output = Value;

    fn sub(self, rhs: f64) -> Self::Output {
        Self::Output::from(self.data - rhs)
    }
}

impl Mul<Value> for f64 {
    type Output = Value;

    fn mul(self, rhs: Value) -> Self::Output {
        Self::Output::from(self * rhs.data)
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

impl Div<Value> for f64 {
    type Output = Value;

    fn div(self, rhs: Value) -> Self::Output {
        Self::Output::from(self / rhs.data)
    }
}

impl Div<Value> for Value {
    type Output = Value;

    fn div(self, rhs: Value) -> Self::Output {
        Self::Output::from(self.data / rhs.data)
    }
}

impl Div<f64> for Value {
    type Output = Value;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output::from(self.data / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add() {
        let a = Value::from(3.0) + 2.0;
        let b = 1.0 + a;
        assert!(b.close_to(6.0))
    }

    #[test]
    fn sub() {
        let a = Value::from(3.0) - 2.0;
        let b = 1.0 - a;
        assert!(b.close_to(0.0))
    }

    #[test]
    fn mul() {
        let a = Value::from(3.0) * 2.0;
        let b = 1.0 * a;
        assert!(b.close_to(6.0))
    }

    #[test]
    fn div() {
        let a = Value::from(3.0) / 2.0;
        let b = 3.0 / a;
        assert!(b.close_to(2.0))
    }
}
