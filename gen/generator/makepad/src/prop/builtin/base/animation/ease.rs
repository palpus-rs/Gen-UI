use std::fmt::Display;

use gen_parser::Value;
use gen_utils::error::Errors;

const ANIMATION_FN_ERROR: &str = r#"
    animation fn must have params: 
    1. constant: f64
    2. exp_decay: f64, f64, usize
    3. pow: f64, f64
    4. bezier: f64, f64, f64, f64
"#;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Ease {
    Linear,
    None,
    Constant(f64),
    InQuad,
    OutQuad,
    InOutQuad,
    InCubic,
    OutCubic,
    InOutCubic,
    InQuart,
    OutQuart,
    InOutQuart,
    InQuint,
    OutQuint,
    InOutQuint,
    InSine,
    OutSine,
    InOutSine,
    InExp,
    OutExp,
    InOutExp,
    InCirc,
    OutCirc,
    InOutCirc,
    InElastic,
    OutElastic,
    InOutElastic,
    InBack,
    OutBack,
    InOutBack,
    InBounce,
    OutBounce,
    InOutBounce,
    ExpDecay {
        d1: f64,
        d2: f64,
        max: usize,
    },

    Pow {
        begin: f64,
        end: f64,
    },
    Bezier {
        cp0: f64,
        cp1: f64,
        cp2: f64,
        cp3: f64,
    },
}

impl Default for Ease {
    fn default() -> Self {
        Ease::Linear
    }
}

impl TryFrom<&Value> for Ease {
    type Error = Errors;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if let Some(u) = value.is_unknown_and_get() {
            match u.as_str() {
                "linear" => Ok(Ease::Linear),
                "none" => Ok(Ease::None),
                "in" => Ok(Ease::InQuad),
                "out" => Ok(Ease::OutQuad),
                "in_out" => Ok(Ease::InOutQuad),
                "in_cubic" => Ok(Ease::InCubic),
                "out_cubic" => Ok(Ease::OutCubic),
                "in_out_cubic" => Ok(Ease::InOutCubic),
                "in_quart" => Ok(Ease::InQuart),
                "out_quart" => Ok(Ease::OutQuart),
                "in_out_quart" => Ok(Ease::InOutQuart),
                "in_quint" => Ok(Ease::InQuint),
                "out_quint" => Ok(Ease::OutQuint),
                "in_out_quint" => Ok(Ease::InOutQuint),
                "in_sine" => Ok(Ease::InSine),
                "out_sine" => Ok(Ease::OutSine),
                "in_out_sine" => Ok(Ease::InOutSine),
                "in_exp" => Ok(Ease::InExp),
                "out_exp" => Ok(Ease::OutExp),
                "in_out_exp" => Ok(Ease::InOutExp),
                "in_circ" => Ok(Ease::InCirc),
                "out_circ" => Ok(Ease::OutCirc),
                "in_out_circ" => Ok(Ease::InOutCirc),
                "in_elastic" => Ok(Ease::InElastic),
                "out_elastic" => Ok(Ease::OutElastic),
                "in_out_elastic" => Ok(Ease::InOutElastic),
                "in_back" => Ok(Ease::InBack),
                "out_back" => Ok(Ease::OutBack),
                "in_out_back" => Ok(Ease::InOutBack),
                "in_bounce" => Ok(Ease::InBounce),
                "out_bounce" => Ok(Ease::OutBounce),
                "in_out_bounce" => Ok(Ease::InOutBounce),
                _ => Err(Errors::PropConvertFail(format!(
                    "can not convert to animation ease: {}",
                    u
                ))),
            }
        } else if let Some(f) = value.is_fn_and_get() {
            let params = f.get_params().as_ref().expect(ANIMATION_FN_ERROR);
            match f.get_name() {
                "constant" => {
                    if params.len() != 1 {
                        return Err(Errors::PropConvertFail(ANIMATION_FN_ERROR.to_string()));
                    }
                    let constant = if let Ok(f) = params[0].parse::<f64>() {
                        f
                    } else {
                        return Err(Errors::PropConvertFail(
                            "constant fn need param: `f64`".to_string(),
                        ));
                    };
                    Ok(Ease::Constant(constant))
                }
                "exp_decay" => {
                    if params.len() != 3 {
                        return Err(Errors::PropConvertFail(ANIMATION_FN_ERROR.to_string()));
                    }
                    let d1 = if let Ok(f) = params[0].parse::<f64>() {
                        f
                    } else {
                        return Err(Errors::PropConvertFail(
                            "exp_decay fn need params: `f64` `f64` `usize`".to_string(),
                        ));
                    };
                    let d2 = if let Ok(f) = params[1].parse::<f64>() {
                        f
                    } else {
                        return Err(Errors::PropConvertFail(
                            "exp_decay fn need params: `f64` `f64` `usize`".to_string(),
                        ));
                    };
                    let max = if let Ok(f) = params[2].parse::<usize>() {
                        f
                    } else {
                        return Err(Errors::PropConvertFail(
                            "exp_decay fn need params: `f64` `f64` `usize`".to_string(),
                        ));
                    };
                    Ok(Ease::ExpDecay { d1, d2, max })
                }
                "pow" => {
                    if params.len() != 2 {
                        return Err(Errors::PropConvertFail(ANIMATION_FN_ERROR.to_string()));
                    }
                    let begin = if let Ok(f) = params[0].parse::<f64>() {
                        f
                    } else {
                        return Err(Errors::PropConvertFail(
                            "pow fn need params: `f64` `f64`".to_string(),
                        ));
                    };
                    let end = if let Ok(f) = params[1].parse::<f64>() {
                        f
                    } else {
                        return Err(Errors::PropConvertFail(
                            "pow fn need params: `f64` `f64`".to_string(),
                        ));
                    };
                    Ok(Ease::Pow { begin, end })
                }
                "bezier" => {
                    if params.len() != 4 {
                        return Err(Errors::PropConvertFail(ANIMATION_FN_ERROR.to_string()));
                    }
                    let cp0 = if let Ok(f) = params[0].parse::<f64>() {
                        f
                    } else {
                        return Err(Errors::PropConvertFail(
                            "bezier fn need params: `f64` `f64` `f64` `f64`".to_string(),
                        ));
                    };
                    let cp1 = if let Ok(f) = params[1].parse::<f64>() {
                        f
                    } else {
                        return Err(Errors::PropConvertFail(
                            "bezier fn need params: `f64` `f64` `f64` `f64`".to_string(),
                        ));
                    };
                    let cp2 = if let Ok(f) = params[2].parse::<f64>() {
                        f
                    } else {
                        return Err(Errors::PropConvertFail(
                            "bezier fn need params: `f64` `f64` `f64` `f64`".to_string(),
                        ));
                    };
                    let cp3 = if let Ok(f) = params[3].parse::<f64>() {
                        f
                    } else {
                        return Err(Errors::PropConvertFail(
                            "bezier fn need params: `f64` `f64` `f64` `f64`".to_string(),
                        ));
                    };
                    Ok(Ease::Bezier { cp0, cp1, cp2, cp3 })
                }
                _ => Err(Errors::PropConvertFail(format!(
                    "can not convert to animation ease: {}",
                    f.get_name()
                ))),
            }
        } else {
            Err(Errors::PropConvertFail(format!(
                "can not convert to animation ease: {}",
                value
            )))
        }
    }
}

impl Display for Ease {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ease::Linear => f.write_str("Linear"),
            Ease::None => f.write_str("None"),
            Ease::Constant(c) => f.write_fmt(format_args!("Constant: ({})", c)),
            Ease::InQuad => f.write_str("InQuad"),
            Ease::OutQuad => f.write_str("OutQuad"),
            Ease::InOutQuad => f.write_str("InOutQuad"),
            Ease::InCubic => f.write_str("InCubic"),
            Ease::OutCubic => f.write_str("OutCubic"),
            Ease::InOutCubic => f.write_str("InOutCubic"),
            Ease::InQuart => f.write_str("InQuart"),
            Ease::OutQuart => f.write_str("OutQuart"),
            Ease::InOutQuart => f.write_str("InOutQuart"),
            Ease::InQuint => f.write_str("InQuint"),
            Ease::OutQuint => f.write_str("OutQuint"),
            Ease::InOutQuint => f.write_str("InOutQuint"),
            Ease::InSine => f.write_str("InSine"),
            Ease::OutSine => f.write_str("OutSine"),
            Ease::InOutSine => f.write_str("InOutSine"),
            Ease::InExp => f.write_str("InExp"),
            Ease::OutExp => f.write_str("OutExp"),
            Ease::InOutExp => f.write_str("InOutExp"),
            Ease::InCirc => f.write_str("InCirc"),
            Ease::OutCirc => f.write_str("OutCirc"),
            Ease::InOutCirc => f.write_str("InOutCirc"),
            Ease::InElastic => f.write_str("InElastic"),
            Ease::OutElastic => f.write_str("OutElastic"),
            Ease::InOutElastic => f.write_str("InOutElastic"),
            Ease::InBack => f.write_str("InBack"),
            Ease::OutBack => f.write_str("OutBack"),
            Ease::InOutBack => f.write_str("InOutBack"),
            Ease::InBounce => f.write_str("InBounce"),
            Ease::OutBounce => f.write_str("OutBounce"),
            Ease::InOutBounce => f.write_str("InOutBounce"),
            Ease::ExpDecay { d1, d2, max } => f.write_fmt(format_args!(
                "ExpDecay: {{d1: {}, d2: {}, max: {}}}",
                d1, d2, max
            )),
            Ease::Pow { begin, end } => {
                f.write_fmt(format_args!("Pow: {{begin: {}, end: {}}}", begin, end))
            }
            Ease::Bezier { cp0, cp1, cp2, cp3 } => f.write_fmt(format_args!(
                "Bezier: {{cp0: {}, cp1: {}, cp2: {}, cp3: {}}}",
                cp0, cp1, cp2, cp3
            )),
        }
    }
}
