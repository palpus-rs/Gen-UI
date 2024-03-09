/// bind parsers
mod bind;
/// comment parsers
mod comment;
/// function parsers
mod function;
/// normal parsers
mod normal;
mod string;
mod tag;
mod color;

pub use bind::*;
pub use comment::parse_comment;
pub use function::*;
pub use normal::*;
pub use string::*;
pub use tag::{end, parse_all, until_end};
pub use color::parse_hex_color;