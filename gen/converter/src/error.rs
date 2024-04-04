use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, Copy)]
pub enum Errors {
    MissMatchKeyWord,
    StrategyNoTemplateStyles,
    StrategyNoTemplateId,
    StrategyNoTemplateClass,
    StrategyNoScript,
    StrategyNoInherits,
}

impl Error for Errors {}

impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Errors::MissMatchKeyWord => "Gen-Converter: MissMatchKeyWord",

            Errors::StrategyNoTemplateStyles => {
                "Gen-Converter[strategy]: Model not have styles or template"
            }
            Errors::StrategyNoTemplateId => "Gen-Converter[strategy]: Model not have template id",
            Errors::StrategyNoTemplateClass => {
                "Gen-Converter[strategy]: Model not have template class"
            }
            Errors::StrategyNoScript => "Gen-Converter[strategy]: Model not have script",
            Errors::StrategyNoInherits => "Gen-Converter[strategy]: Model not have inherits",
        })
    }
}
