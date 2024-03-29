use std::{fs::{read_to_string, File}, io::Write};
use gen_parser::*;

fn main() {
    let file = read_to_string("/Users/user/Workspace/others/beyond-framework/gen/tests/ui/view/index.gen");
    let input = file.unwrap();
    // let ast = ParseResult::try_from(ParseTarget::try_from(input).unwrap()).unwrap();
    let target = ParseResult::try_from(ParseTarget::try_from(input.as_str()).unwrap()).unwrap();
    // dbg!(ast);
    // let mut f = File::create("/Users/user/Workspace/others/beyond-framework/gen/tests/release/ast.txt").unwrap();
   
    
}
