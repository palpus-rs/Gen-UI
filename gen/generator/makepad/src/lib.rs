use core::panic;
use std::{
    io::Write,
    path::{Path, PathBuf},
};

// use gen::{sc_builder_to_token_stream, template};
use gen_converter::model::{script::ScriptModel, Model, Source};
use gen_utils::common::{token_stream_to_tree, token_tree_ident};
use proc_macro2::{TokenStream, TokenTree};
use quote::{quote, ToTokens};
use utils::create_file;
use widget::model::{app_main::AppMain, widget::Widget, ToLiveDesign};

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

#[derive(Debug, Clone)]
pub struct RsFile {
    pub source: Source,
    pub content: TokenStream,
}

impl RsFile {
    pub fn new(source: Source, content: TokenStream) -> Self {
        RsFile { source, content }
    }
    pub fn new_empty(source: Source) -> Self {
        RsFile {
            source,
            content: TokenStream::new(),
        }
    }
    pub fn compile(&self) -> () {
        let mut file = create_file(self.source.compiled_file.as_path());
        file.write_all(self.content.to_string().as_bytes()).unwrap();
    }
}

impl From<Model> for RsFile {
    fn from(value: Model) -> Self {
        if let ScriptModel::Rs(rs) = value.script.as_ref().unwrap() {
            RsFile::new(value.special, rs.to_token_stream())
        } else {
            panic!("Model to RsFile error")
        }
    }
}

#[derive(Debug, Clone)]
pub enum ModelNode {
    Widget(Widget),
    RsFile(RsFile),
}

impl ModelNode {
    pub fn source(&self) -> Option<&Source> {
        match self {
            ModelNode::Widget(widget) => widget.source.as_ref(),
            ModelNode::RsFile(rs) => Some(&rs.source),
        }
    }
    pub fn content(&self) -> TokenStream {
        match self {
            ModelNode::Widget(widget) => widget.to_live_design().to_token_stream(),
            ModelNode::RsFile(rs) => rs.content.clone(),
        }
    }
    pub fn level(&self) -> (usize, PathBuf) {
        let path = self.source().unwrap().level_gen();
        (path.components().count(), path)
    }
}

impl From<Model> for ModelNode {
    fn from(value: Model) -> Self {
        let source = &value.special;
        match &value.strategy {
            gen_parser::Strategy::None => RsFile::new_empty(source.clone()).into(),
            gen_parser::Strategy::SingleScript => RsFile::from(value).into(),
            gen_parser::Strategy::Error(e) => panic!("{}", e),
            _ => Widget::from(value).into(),
        }
    }
}

impl From<Widget> for ModelNode {
    fn from(value: Widget) -> Self {
        ModelNode::Widget(value)
    }
}

impl From<RsFile> for ModelNode {
    fn from(value: RsFile) -> Self {
        ModelNode::RsFile(value)
    }
}

/// ## 定root多叉模型树
/// ### struct example
/// ```
/// {
/// node: src/views/root.gen,
/// children: [
///     {node: src/a.gen, children: [
///             {node: src/views/b.gen, children: None},
///             {node: src/views/d.gen, children: None},
///             {node: src/components/c.gen, children: None}
///         ]
///     },
///   ]
/// }
/// ```
#[derive(Debug, Clone)]
pub struct ModelTree {
    /// model node can be widget or rs file, but the root node must be widget
    pub node: ModelNode,
    pub children: Option<Vec<ModelTree>>,
}

impl ModelTree {
    pub fn new(node: ModelNode) -> ModelTree {
        Self {
            node,
            children: None,
        }
    }
    /// add node to widget tree
    /// compare path, src is the same root
    /// eg:
    /// - item_path:  src/a1/b/c
    /// - current_path: src/a2
    /// means: item should in 4 level
    pub fn add(&mut self, item: ModelNode) -> () {
        fn similarity(path1: &PathBuf, path2: &PathBuf) -> usize {
            let components1: Vec<_> = path1.components().collect();
            let components2: Vec<_> = path2.components().collect();

            components1
                .iter()
                .zip(components2.iter())
                .take_while(|(a, b)| a == b)
                .count()
        }

        // get level and compare
        let (item_level, item_path) = item.level();
        // let (_, current_path) = self.level();

        if let Some(children) = &mut self.children {
            // 查找子节点中任意的path的节点，首先使用level匹配，level相同，可以直接push
            // level不同，若当前level比item的level小，继续遍历子节点，大则将当前children放到item的children中，再把item放回父节点进行替换
            let (current_level, _current_path) = children[0].level();
            let step = item_level - current_level;
            if step.eq(&0_usize) {
                children.push(item.into())
            } else if step.lt(&0_usize) {
                // 说明item节点比当前节点层级高，将item节点替换当前的节点
                let mut node: ModelTree = item.into();
                node.children.replace(self.children.take().unwrap());
                // add into parent node
                let _ = std::mem::replace(&mut self.children, Some(vec![node]));
            } else {
                // 说明item节点比当前节点层级低，继续遍历子节点
                // 需要查找当前所有子节点的path，找到符合前缀的节点，查看子节点数量，哪个少往哪个去遍历（符合前缀指的是前缀匹配优先级最大的）
                // 不能使用start_with去匹配，因为无法知道若前缀没有完全相同的情况下的优先级长度
                // 例如： [src/a/z/y]
                // 1. src/a/b/c , 2. src/a/z , 3. src/a/z/y
                // 那么应该选择第三个节点进行遍历，因为第三个节点的前缀匹配优先级最大
                // 递归调用当前这个方法
                let mut target_node: Option<ModelTree> = None;
                let mut max_sim = 0_usize;
                for child in children.iter() {
                    let (_, child_path) = child.level();
                    // compare child path and item path
                    let sim = similarity(&item_path, &child_path);
                    dbg!(item_path.as_path());
                    dbg!(child_path.as_path());
                    if sim.eq(&0_usize) {
                        // 相似度为0，说明没有相同的前缀，直接跳过
                        continue;
                    } else {
                        // 有相似度，和当前max相似度比较, 大于max则替换target_node
                        if sim.gt(&max_sim) {
                            max_sim = sim;
                            target_node.replace(child.clone());
                        }
                    }
                }
                // 查看target_node是否存在，存在说明找到了优先级最大的节点，递归调用这个add方法，不存在则直接push
                if let Some(target_node) = &mut target_node {
                    target_node.add(item);
                } else {
                    children.push(item.into());
                }
            }
        } else {
            // now have no children, just set
            self.children.replace(vec![item.into()]);
        }
        dbg!(&self);
    }
    /// ## get widget tree level
    /// tree level can get from node source path
    /// ### return
    /// (level, path)
    /// - `level: usize`: path length which can easy know the level of the tree, if compare with another level can know the tree is child or parent, acturally you can think level is just offset of dir path
    /// - `path: PathBuf`: level path which only contain dir level
    pub fn level(&self) -> (usize, PathBuf) {
        let source = self.node.source().unwrap().level_gen();

        (source.components().count(), source)
    }
    pub fn default_root() -> ModelTree {
        ModelTree {
            node: Widget::default_ui_root().into(),
            children: None,
        }
    }
    /// get super ui root name
    pub fn super_ui_root(&self) -> String {
        self.node.source().unwrap().source_name_lower()
    }
    /// convert widget tree to lib.rs mod
    pub fn to_lib(&self) -> TokenStream {
        // get node widget source
        self.to_lib_list()
            .iter()
            .fold(TokenStream::new(), |mut acc, item| {
                let item = token_tree_ident(item);
                acc.extend(quote! {
                    pub mod #item;
                });
                acc
            })
    }
    pub fn to_lib_list(&self) -> Vec<String> {
        let mut mods = vec![];

        let source = self.node.source().unwrap();

        mods.push(source.source_name_lower());

        if let Some(children) = &self.children {
            for child in children {
                let child_mod = child.to_lib_list();
                mods.extend(child_mod);
            }
        }

        mods
    }
    /// compile model tree
    pub fn compile(&self) -> () {
        let print = |node: &ModelNode| -> () {
            let content = node.content().to_string();
            let mut file = create_file(node.source().unwrap().compiled_file.as_path());
            file.write_all(content.as_bytes()).unwrap();
        };

        // 遍历整个树，将每个节点的内容写入到文件中
        let _ = print(&self.node);
        // children
        if let Some(children) = self.children.as_ref() {
            for child in children {
                let _ = child.compile();
            }
        }
    }
}

impl From<Widget> for ModelTree {
    fn from(value: Widget) -> Self {
        Self {
            node: value.into(),
            children: None,
        }
    }
}
impl From<RsFile> for ModelTree {
    fn from(value: RsFile) -> Self {
        Self {
            node: value.into(),
            children: None,
        }
    }
}

impl From<ModelNode> for ModelTree {
    fn from(value: ModelNode) -> Self {
        Self {
            node: value,
            children: None,
        }
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
