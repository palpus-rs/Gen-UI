/// bind parsers
mod bind;
mod color;
/// comment parsers
mod comment;
/// function parsers
mod function;
/// normal parsers
mod normal;
#[cfg(feature = "makepad")]
mod shader;
mod string;
mod tag;
mod special;

pub use bind::*;
pub use color::*;
pub use comment::parse_comment;
pub use function::*;
pub use normal::*;
pub use string::*;
pub use tag::{end, parse_all, until_end};
pub use special::Special;
pub use shader::MakepadShader;