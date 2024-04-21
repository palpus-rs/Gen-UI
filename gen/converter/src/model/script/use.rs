use quote::ToTokens;
use syn::ItemUse;

/// 用来表示组件引入的依赖
#[derive(Debug, Clone, Default)]
pub struct UseMod {
    /// 表示引入的gen系列的库（这个部分一般在生成最终代码时会忽略）
    pub gen: Option<Vec<ItemUse>>,
    /// 表示引入的widget，这些widget都是自定义的widget需要在template部分中使用（这个部分可能会在最终生成代码时进行处理）
    /// 这些widget会以普通use的形式引入
    /// 例如：引入一个名叫`my_button`的widget
    /// ```
    /// <script>
    /// use crate::MyButton;
    /// </script>
    /// ```
    pub widget: Option<Vec<ItemUse>>,
    /// 表示其他引入的mod，这些mod会在最终的代码中显示
    pub other: Option<Vec<ItemUse>>,
}

impl UseMod {
    pub fn push(&mut self, item: ItemUse) {
        let ident = item.to_token_stream().to_string();
        if ident.contains("gen") {
            self.push_gen(item);
        } else if ident.contains("crate") || ident.contains("super") {
            self.push_widget(item);
        } else {
            self.push_other(item);
        }
    }
    pub fn push_gen(&mut self, item: ItemUse) {
        Self::push_item(&mut self.gen, item);
    }
    pub fn push_widget(&mut self, item: ItemUse) {
        Self::push_item(&mut self.widget, item);
    }
    pub fn push_other(&mut self, item: ItemUse) {
        Self::push_item(&mut self.other, item);
    }
    fn push_item(target: &mut Option<Vec<ItemUse>>, item: ItemUse) -> () {
        if target.is_none() {
            target.replace(vec![item]);
        } else {
            target.as_mut().unwrap().push(item);
        }
    }
}
