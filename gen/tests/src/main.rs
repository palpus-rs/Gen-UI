use gen_converter::model::Model;
use gen_parser::*;

fn main() {
    // let file = read_to_string("/Users/user/Workspace/others/beyond-framework/gen/tests/ui/view/index.gen");
    // let file = read_to_string("E:/Rust/try/makepad/Gen-UI/gen/tests/ui/view/index.gen");

    // let input = file.unwrap();
    // // let ast = ParseResult::try_from(ParseTarget::try_from(input).unwrap()).unwrap();
    // let target = ParseResult::try_from(ParseTarget::try_from(input.as_str()).unwrap()).unwrap();
    // dbg!(ast);
    // let mut f = File::create("/Users/user/Workspace/others/beyond-framework/gen/tests/release/ast.txt").unwrap();
    //    dbg!(target.template());

    let model = Model::new("E:/Rust/try/makepad/Gen-UI/gen/tests/ui/view/index.gen");
    dbg!(model.unwrap());
}
