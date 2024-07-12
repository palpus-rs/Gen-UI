use std::fmt::Display;

use gen_parser::Value;
use gen_utils::error::Errors;

use crate::widget::utils::fn_prop;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Play {
    Forward { duration: f64 },
    Snap,
    Reverse { duration: f64, end: f64 },
    Loop { duration: f64, end: f64 },
    ReverseLoop { duration: f64, end: f64 },
    BounceLoop { duration: f64, end: f64 },
}

impl Default for Play {
    fn default() -> Self {
        Play::Forward { duration: 1.0 }
    }
}

impl Display for Play {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Play::Forward { duration } => {
                f.write_fmt(format_args!("Forward{{duration: {}.0}}", duration))
            }
            Play::Snap => f.write_str("Snap"),
            Play::Reverse { duration, end } => f.write_fmt(format_args!(
                "Reserve{{duration: {}, end: {}.0}}",
                duration, end
            )),
            Play::Loop { duration, end } => f.write_fmt(format_args!(
                "Loop{{duration: {}, end: {}.0}}",
                duration, end
            )),
            Play::ReverseLoop { duration, end } => f.write_fmt(format_args!(
                "ReverseLoop{{duration: {}, end: {}.0}}",
                duration, end
            )),
            Play::BounceLoop { duration, end } => f.write_fmt(format_args!(
                "BounceLoop{{duration: {}, end: {}.0}}",
                duration, end
            )),
        }
    }
}

impl TryFrom<&Value> for Play {
    type Error = Errors;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let mut play = Err(Errors::PropConvertFail(format!(
            "{} can not convert to Play",
            value
        )));

        let _ = fn_prop(value, |name, params| {
            if params.is_none() {
                if name == "snap" {
                    play = Ok(Play::Snap);
                }
            } else {
                match name {
                    "forward" => {
                        if params.unwrap().len() == 1 {
                            if let Ok(duration) = params.unwrap().get(0).unwrap().parse::<f64>() {
                                play = Ok(Play::Forward { duration });
                            }
                        }
                    }
                    "reverse" => {
                        if params.unwrap().len() == 1 {
                            if let Ok(duration) = params.unwrap().get(0).unwrap().parse::<f64>() {
                                play = Ok(Play::Reverse { duration, end: 1.0 });
                            }
                        }
                        if params.unwrap().len() == 2 {
                            match (
                                params.unwrap().get(0).unwrap().parse::<f64>(),
                                params.unwrap().get(1).unwrap().parse::<f64>(),
                            ) {
                                (Ok(duration), Ok(end)) => {
                                    play = Ok(Play::Reverse { duration, end });
                                }
                                _ => {}
                            }
                        }
                    }
                    "loop" => {
                        if params.unwrap().len() == 1 {
                            if let Ok(duration) = params.unwrap().get(0).unwrap().parse::<f64>() {
                                play = Ok(Play::Loop { duration, end: 1.0 });
                            }
                        }
                        if params.unwrap().len() == 2 {
                            match (
                                params.unwrap().get(0).unwrap().parse::<f64>(),
                                params.unwrap().get(1).unwrap().parse::<f64>(),
                            ) {
                                (Ok(duration), Ok(end)) => {
                                    play = Ok(Play::Loop { duration, end });
                                }
                                _ => {}
                            }
                        }
                    }
                    "reverse_loop" => {
                        if params.unwrap().len() == 1 {
                            if let Ok(duration) = params.unwrap().get(0).unwrap().parse::<f64>() {
                                play = Ok(Play::ReverseLoop { duration, end: 1.0 });
                            }
                        }
                        if params.unwrap().len() == 2 {
                            match (
                                params.unwrap().get(0).unwrap().parse::<f64>(),
                                params.unwrap().get(1).unwrap().parse::<f64>(),
                            ) {
                                (Ok(duration), Ok(end)) => {
                                    play = Ok(Play::ReverseLoop { duration, end });
                                }
                                _ => {}
                            }
                        }
                    }
                    "bounce_loop" => {
                        if params.unwrap().len() == 1 {
                            if let Ok(duration) = params.unwrap().get(0).unwrap().parse::<f64>() {
                                play = Ok(Play::BounceLoop { duration, end: 1.0 });
                            }
                        }
                        if params.unwrap().len() == 2 {
                            match (
                                params.unwrap().get(0).unwrap().parse::<f64>(),
                                params.unwrap().get(1).unwrap().parse::<f64>(),
                            ) {
                                (Ok(duration), Ok(end)) => {
                                    play = Ok(Play::BounceLoop { duration, end });
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        });

        play
    }
}
