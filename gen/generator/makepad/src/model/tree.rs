use std::{collections::HashSet, io::Write, path::PathBuf};

use gen_utils::common::token_tree_ident;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{utils::create_file, widget::model::widget::Widget};

use super::{ModelNode, RsFile};

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
    /// convert model tree to lib.rs mod
    pub fn to_lib(&self) -> TokenStream {
        // get node model source
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
    /// convert model tree to lib.rs mod list
    /// acutally this method is used to get all mod name
    /// what need to do is get the first level file name or dir name
    pub fn to_lib_list(&self) -> Vec<String> {
        let mut mods = HashSet::new();

        if let Some(children) = &self.children {
            for child in children {
                // let mod_name = child.node.source().unwrap().source_name_rs();

                let mod_name = child.node.source().unwrap().to_lib();

                mods.insert(mod_name);
            }
        }

        mods.into_iter().collect()
    }
    /// compile model tree
    pub fn compile(&self) -> () {
        let loop_tree = |node: &ModelNode| -> () {
            let content = node.content().to_string();
            let mut file = create_file(node.source().unwrap().compiled_file.as_path());
            file.write_all(content.as_bytes()).unwrap();
        };

        // 遍历整个树，将每个节点的内容写入到文件中
        let _ = loop_tree(&self.node);
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
