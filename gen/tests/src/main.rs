use std::path::Path;

use gen_converter::{model::Model, strategy::id};
use gen_parser::*;

fn main() {
    let mut view_model = Model::new(Path::new(
        "E:/Rust/try/makepad/Gen-UI/gen/tests/ui/view/index.gen",
    ))
    .unwrap();

    let _ = id(&mut view_model, |t_model, id_style| {
        id_style.into_iter().for_each(|(k, v)| {
            t_model.push_prop(k, v);
        });
    });

    dbg!(view_model);
}
