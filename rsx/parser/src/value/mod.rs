mod color;
mod function;

use std::fmt::Display;

pub use color::Color;
pub use function::Function;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    // u type number
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    USize(usize),
    // i type number
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    ISize(isize),
    // float
    Float(f32),
    Double(f64),
    /// bool
    Bool(bool),
    Vec(Vec<Value>),
    /// String
    String(String),
    /// value inject
    /// <xxx :value="xValue" />
    /// <script> let xValue:&str = "hello!";</script>
    /// <script> let xValue:Vec<&str> = vec!["a","b"];</script>
    Bind(String),
    /// function inject
    /// <xxx @click="doClick" />
    Function(Function),
    /// function return ()  as :`fn xxx()->(){}`
    Void,
    /// color value
    /// - hex color: #fff00f
    /// - rgb color: rgb(211,23,255)
    /// - rgba color: rgba(255,255,87,0.4)
    Color(Color),
    UnKnown(String),
}

impl Value {
    pub fn bind(variable: &str) -> Self {
        Self::Bind(variable.to_string())
    }
    pub fn void() -> Self {
        Self::Void
    }
}

impl From<(&str, Option<Vec<&str>>, bool)> for Value {
    fn from(value: (&str, Option<Vec<&str>>, bool)) -> Self {
        Value::Function(value.into())
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Value::U8(value)
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Value::U16(value)
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Value::U32(value)
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Value::U64(value)
    }
}

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        Value::USize(value)
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Self {
        Value::I8(value)
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Value::I16(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::I32(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::I64(value)
    }
}

impl From<isize> for Value {
    fn from(value: isize) -> Self {
        Value::ISize(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_string())
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<&String> for Value {
    fn from(value: &String) -> Self {
        Value::String(value.to_string())
    }
}

impl From<Color> for Value {
    fn from(value: Color) -> Self {
        Value::Color(value)
    }
}

impl From<Function> for Value {
    fn from(value: Function) -> Self {
        Value::Function(value)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Value::U8(n) => n.to_string(),
            Value::U16(n) => n.to_string(),
            Value::U32(n) => n.to_string(),
            Value::U64(n) => n.to_string(),
            Value::USize(n) => n.to_string(),
            Value::I8(n) => n.to_string(),
            Value::I16(n) => n.to_string(),
            Value::I32(n) => n.to_string(),
            Value::I64(n) => n.to_string(),
            Value::ISize(n) => n.to_string(),
            Value::Float(n) => n.to_string(),
            Value::Double(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::String(s) => s.to_string(),
            Value::Bind(bind) => bind.to_string(),
            Value::Function(func) => func.to_string(),
            Value::Void => String::new(),
            Value::Color(color) => color.to_string(),
            Value::Vec(v) => format!(
                "{:?}",
                v.into_iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
            ),
            Value::UnKnown(u) => u.to_string(),
        };

        f.write_str(&res)
    }
}

#[cfg(test)]
mod test_value {}
