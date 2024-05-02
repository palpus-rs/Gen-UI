use gen_converter::error::Errors;

#[derive(Clone, Debug, PartialEq)]
pub enum LiveValue {
    Bool,
    I64,
    U64,
    F32,
    F64,
    String,
    Vec2,
    Vec3,
    Vec4,
    Color,
}

impl TryFrom<&str> for LiveValue {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "bool" => Ok(LiveValue::Bool),
            "i8" | "i16" | "i32" | "i64" | "isize" => Ok(LiveValue::I64),
            "u8" | "u16" | "u32" | "u64" | "usize" => Ok(LiveValue::U64),
            "f32" => Ok(LiveValue::F32),
            "f64" => Ok(LiveValue::F64),
            "String" => Ok(LiveValue::String),
            "Vec2" => Ok(LiveValue::Vec2),
            "Vec3" => Ok(LiveValue::Vec3),
            "Vec4" => Ok(LiveValue::Vec4),
            "Color" => Ok(LiveValue::Color),
            _ => Err(Errors::PropConvertFail(format!(
                "Cannot convert {} to LiveValue",
                value
            ))),
        }
    }
}
