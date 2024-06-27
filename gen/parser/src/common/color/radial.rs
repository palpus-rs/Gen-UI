use super::{check_and_fix, trans_hex_percentage, Hex, Percentage};
use crate::Function;
use gen_utils::error::Errors;

/// 语法: `radial_gradient(color percentage, color percentage, ...)`
#[derive(Debug, Clone)]
pub struct RadialGradient {
    pub colors: Vec<(Hex, Percentage)>,
}

impl TryFrom<&Function> for RadialGradient {
    type Error = Errors;

    fn try_from(value: &Function) -> Result<Self, Self::Error> {
        // 检查是否fn的名称叫radial_gradient
        if value.get_name().eq("radial_gradient") {
            // radial的参数至少有2个
            if let Some(params) = value.get_params() {
                let len = params.len();
                if len >= 2 {
                    let mut colors: Vec<(Hex, Percentage, bool)> = vec![];
                    for i in 0..len {
                        colors.push(trans_hex_percentage(&params[i], i, len)?);
                    }
                    let colors = check_and_fix(&mut colors);
                    return Ok(RadialGradient { colors });
                }
            }
            return Err(Errors::ParseError(format!(
                "parse radial_gradient error: {}, radial_gradient fn need two params `(color percentage, color percentage, ...)`",
                value.get_name()
            )));
        }
        return Err(Errors::ParseError(format!(
            "parse radial_gradient error: {}",
            value.get_name()
        )));
    }
}
