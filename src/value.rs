use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Number {
    I64(i64),
    F64(f64),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    String(String),
    Number(Number),
    Boolean(bool),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
    Null,
}

impl TryFrom<&Value> for String {
    type Error = ();

    fn try_from(value: &Value) -> Result<Self, ()> {
        match value {
            Value::String(value) => Ok(value.clone()),
            _ => Err(()),
        }
    }
}

impl TryFrom<&Value> for i64 {
    type Error = ();

    #[allow(clippy::cast_possible_truncation)]
    fn try_from(value: &Value) -> Result<Self, ()> {
        match value {
            Value::Number(value) => match value {
                Number::I64(value) => Ok(*value),
                Number::F64(value) => Ok(*value as i64),
            },
            _ => Err(()),
        }
    }
}

impl TryFrom<&Value> for f64 {
    type Error = ();

    fn try_from(value: &Value) -> Result<Self, ()> {
        match value {
            Value::Number(value) => match value {
                Number::F64(value) => Ok(*value),
                Number::I64(value) => Ok(*value as f64),
            },
            _ => Err(()),
        }
    }
}

impl TryFrom<&Value> for bool {
    type Error = ();

    fn try_from(value: &Value) -> Result<Self, ()> {
        match value {
            Value::Boolean(value) => Ok(*value),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a Value> for &'a Vec<Value> {
    type Error = ();

    fn try_from(value: &'a Value) -> Result<Self, ()> {
        match value {
            Value::Array(value) => Ok(value),
            _ => Err(()),
        }
    }
}

#[allow(clippy::implicit_hasher)]
impl<'a> TryFrom<&'a Value> for &'a HashMap<String, Value> {
    type Error = ();

    fn try_from(value: &'a Value) -> Result<Self, ()> {
        match value {
            Value::Object(value) => Ok(value),
            _ => Err(()),
        }
    }
}
