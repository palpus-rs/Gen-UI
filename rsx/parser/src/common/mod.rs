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

pub use bind::*;
pub use comment::parse_comment;
pub use function::*;
pub use normal::*;
pub use string::*;
pub use tag::{parse_all,end};