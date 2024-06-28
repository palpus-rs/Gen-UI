pub fn float_to_str(num: f32) -> String {
    if num.fract() == 0.0 {
        format!("{}.0", num)
    } else {
        format!("{}", num)
    }
}