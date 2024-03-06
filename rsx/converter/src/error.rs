use std::{error, fmt::Display};

#[derive(Debug,Clone,PartialEq)]
pub enum Errors{
    UppercaseTitleFail,
    UnMatchedWidget,
    UnMatchedProp(String,String),
    KnownPropType,
    UnAcceptConvertRange
}

impl Errors {
    pub fn unmatched_prop<S>(prop:&str,widget:S)->Self
    where S:Display{
        Self::UnMatchedProp(prop.to_string(), widget.to_string())
    }
}

impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Errors::UppercaseTitleFail => f.write_str("Uppercase title char failure: The first title char of the string is not ascii alphabetic!"),
                Errors::UnMatchedWidget => f.write_str("This widget cannot be found in the current library"),
                Errors::UnMatchedProp(prop,widget) => f.write_fmt(format_args!(
                    "This prop: {} cannot be found in the current widget:{}", prop, widget
                )),
                Errors::KnownPropType =>f.write_str("This prop type is not unknown"),
                Errors::UnAcceptConvertRange => f.write_str("Unacceptable conversion for this range"),
            }
    }
}

impl error::Error for Errors {
    
}