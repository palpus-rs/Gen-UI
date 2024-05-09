use std::{
    io::Write,
    path::{Path, PathBuf},
};

// use gen::{sc_builder_to_token_stream, template};
use gen_converter::model::{self, Source};
use gen_utils::common::{token_stream_to_tree, token_tree_ident};
use proc_macro2::{TokenStream, TokenTree};
use quote::quote;
use utils::create_file;
use widget::model::{app_main::AppMain, widget::Widget};

pub mod error;
// pub mod gen;
pub mod instance;
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
    // pub app_main: AppMain,
    pub widget_tree: Option<WidgetTree>,
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
        // let main_rs = Makepad::create_app_rs(entry, path);
        Makepad {
            // app_main: todo!(),
            widget_tree: Some(widget_tree),
            main_rs,
        }
    }
    fn create_widget_tree<P>(path: P, root: Option<&PathBuf>) -> WidgetTree
    where
        P: AsRef<Path>,
    {
        match root {
            Some(root) => {
                let gen_model =
                    gen_converter::model::Model::new(root, &path.as_ref().to_path_buf(), false)
                        .unwrap();
                WidgetTree::new(gen_model.into())
            }
            None => WidgetTree::default_root(),
        }
    }
    fn create_app_rs<P>(entry: &str, path: P)
    where
        P: AsRef<Path>,
    {
        let app_path = path.as_ref().join("src").join(entry);
        let mut app_file = create_file(app_path.as_path());
    }
    /// makepad main rs is easy, which just need to use app_main fn to run app
    fn create_main_rs<P>(entry: &str, path: P) -> RsFile
    where
        P: AsRef<Path>,
    {
        let main_path = path.as_ref().join("src").join("main.rs");

        let entry = token_tree_ident(entry);
        let project_name = quote! {src-gen};
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
}

#[derive(Debug)]
pub struct RsFile {
    pub source: Source,
    pub content: TokenStream,
}

impl RsFile {
    pub fn new(source: Source, content: TokenStream) -> Self {
        RsFile { source, content }
    }
}

#[derive(Debug, Clone)]
pub struct WidgetTree {
    pub node: Widget,
    pub children: Option<Vec<WidgetTree>>,
}

impl WidgetTree {
    pub fn new(node: Widget) -> WidgetTree {
        Self {
            node,
            children: None,
        }
    }
    pub fn default_root() -> WidgetTree {
        WidgetTree {
            node: Widget::default_ui_root(),
            children: None,
        }
    }
}

impl From<Widget> for WidgetTree {
    fn from(value: Widget) -> Self {
        Self {
            node: value,
            children: None,
        }
    }
}

// impl Default for WidgetTree {
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
