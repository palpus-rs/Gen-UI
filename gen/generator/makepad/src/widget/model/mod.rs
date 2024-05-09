use proc_macro2::TokenStream;

use crate::ToToken;

use self::live_design::LiveDesign;

pub mod app_main;
pub mod attr;
pub mod field;
pub mod handler;
pub mod live_design;
pub mod match_event;
pub mod role;
pub mod traits;
pub mod widget;

// #[derive(Debug, Clone)]
// pub enum Model {
//     AppMain(app_main::AppMain),
//     Widget(widget::Widget),
// }

// impl Model {
//     pub fn new(model: gen_converter::model::Model) -> Self {
//         // 判断是否是AppMain
//         match model.is_entry() {
//             true => Model::AppMain(app_main::AppMain::from(model)),
//             false => Model::Widget(widget::Widget::from(model)),
//         }
//     }
// }

// impl ToToken for Model {
//     fn to_token_stream(&self) -> TokenStream {
//         LiveDesign::from(self).to_token_stream()
//     }
// }

pub trait ToLiveDesign {
    fn widget_tree(&self) -> Option<TokenStream>;
    fn widget_logic(&self) -> Option<TokenStream>;
    fn to_live_design(&self) -> LiveDesign;
}
