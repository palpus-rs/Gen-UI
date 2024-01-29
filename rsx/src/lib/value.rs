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
    // bool
    Bool(bool),
    // String
    String(String),
    // value inject
    // <xxx :value="xValue" />
    // <script> let xValue:&str = "hello!";</script>
    // <script> let xValue:Vec<&str> = vec!["a","b"];</script>
    Bind(String),
    // function inject
    // <xxx @click="doClick" />
    Function(String),
}
