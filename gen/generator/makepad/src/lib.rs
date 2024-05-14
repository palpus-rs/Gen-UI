use std::{
    io::Write,
    path::{Path, PathBuf},
};

// use gen::{sc_builder_to_token_stream, template};
use gen_converter::model::{Model, Source};
use gen_utils::common::{token_stream_to_tree, token_tree_ident};
use model::{ModelTree, RsFile};
use proc_macro2::{TokenStream, TokenTree};
use quote::{quote, ToTokens};
use utils::create_file;
use widget::model::{app_main::AppMain, widget::Widget, ToLiveDesign};

pub mod error;
// pub mod gen;
pub mod model;
pub mod prop;
pub mod utils;
pub mod widget;

pub trait ToToken {
    fn to_token_stream(&self) -> TokenStream;
    fn to_token_trees(&self) -> Vec<TokenTree> {
        token_stream_to_tree(self.to_token_stream())
    }
}

#[derive(Debug)]
pub struct Makepad {
    pub app_main: AppMain,
    pub tree: Option<ModelTree>,
    pub main_rs: RsFile,
}

impl Makepad {
    /// init makepad project
    /// - create main.rs
    /// - create app entry rs file (eg: app.rs)
    /// - create lib.rs (depend on root)
    pub fn new<P>(entry: &str, path: P, root: Option<&PathBuf>) -> Self
    where
        P: AsRef<Path>,
    {
        let main_rs = Makepad::create_main_rs(entry, path.as_ref());
        let widget_tree = Makepad::create_widget_tree(path.as_ref(), root);
        let app_main = Makepad::create_app_main(entry, path, &widget_tree);
        Makepad {
            app_main,
            tree: Some(widget_tree),
            main_rs,
        }
    }
    fn create_widget_tree<P>(path: P, root: Option<&PathBuf>) -> ModelTree
    where
        P: AsRef<Path>,
    {
        match root {
            Some(root) => {
                let gen_model: Widget =
                    gen_converter::model::Model::new(root, &path.as_ref().to_path_buf(), false)
                        .unwrap()
                        .into();
                ModelTree::new(gen_model.into())
            }
            None => ModelTree::default_root(),
        }
    }
    fn create_app_main<P>(entry: &str, path: P, widget_tree: &ModelTree) -> AppMain
    where
        P: AsRef<Path>,
    {
        let ui_root = widget_tree.super_ui_root();
        let live_register = widget_tree.to_lib_list();
        let app_path = path.as_ref().join(format!("{}.gen", entry).as_str());
        let source = Source::from((app_path.as_path(), path.as_ref()));
        let mut app = AppMain::new(&source);
        app.set_root_ref(ui_root).set_live_register(live_register);

        app
    }
    /// makepad main rs is easy, which just need to use app_main fn to run app
    fn create_main_rs<P>(entry: &str, path: P) -> RsFile
    where
        P: AsRef<Path>,
    {
        let main_path = path.as_ref().join("src").join("main.rs");
        let entry = token_tree_ident(entry);
        let project_name = quote! {src_gen};
        // let mut main_file = create_file(main_path.as_path());

        let content = quote! {
            fn main(){
                #project_name::#entry::app_main()
            }
        };
        RsFile::new((main_path, path).into(), content)
        // main_file
        //     .write_all(main_content.to_string().as_bytes())
        //     .unwrap();
    }
    pub fn compile_app_main(&self) -> () {
        let content = self.app_main.to_live_design().to_token_stream().to_string();
        let mut file = create_file(self.app_main.source.compiled_file.as_path());
        file.write_all(content.as_bytes()).unwrap();
    }
    pub fn compile_lib_rs(&self) -> () {
        let lib_mods = self.tree.as_ref().unwrap().to_lib();
        let content = quote! {
            pub use makepad_widgets;
            pub use makepad_widgets::makepad_draw;
            pub mod app;
            #lib_mods
        }
        .to_string();

        let mut lib_path = self.main_rs.source.compiled_file.clone();
        lib_path.pop();
        lib_path.push("lib.rs");
        let mut file = create_file(lib_path.as_path());
        file.write_all(content.as_bytes()).unwrap();
    }
    /// add item to model tree
    pub fn add(&mut self, item: Model) -> () {
        self.tree.as_mut().unwrap().add(item.into());
    }
    /// Makepad Compile
    /// - compile main.rs
    /// - compile app.rs
    /// - compile lib.rs
    /// - compile other widget.rs (which is in ModelTree, use ModelTree compile method to compile)
    pub fn compile(&self) {
        // compile main.rs
        self.main_rs.compile();
        // compile app.rs
        self.compile_app_main();
        // compile lib.rs
        self.compile_lib_rs();
        // compile other widget.rs
        self.tree.as_ref().unwrap().compile();
    }
}

// impl Default for ModelTree {
//     fn default() -> Self {
//         Self {
//             node: Widget::from(value),
//             children: None,
//         }
//     }
// }

#[cfg(test)]
mod test_makepad {
    use crate::{
        widget::model::{widget::Widget, ToLiveDesign},
        ToToken,
    };

    #[test]
    fn widget_default() {
        let widget = Widget::default_ui_root();
        dbg!(widget.to_live_design().to_token_stream().to_string());
    }
}
