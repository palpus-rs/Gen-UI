use makepad_widgets::Widget;
use proc_macro2::TokenStream;
/// 对于Widget来说
/// draw_walk是必须实现的
/// 其他的方法是可选的
#[derive(Debug, Default, Clone)]
pub struct WidgetTrait{
    /// 必须实现
    pub draw_walk: TokenStream,
    /// 可选实现
    pub handle_event: Option<TokenStream>,
    pub widget: Option<TokenStream>,
    pub widgets: Option<TokenStream>,
    pub widget_id: Option<TokenStream>,
    pub widget_to_data: Option<TokenStream>,
    pub data_to_widget: Option<TokenStream>,
    pub draw: Option<TokenStream>,
    pub draw_walk_all: Option<TokenStream>,
    pub is_visible: Option<TokenStream>,
    pub draw_all: Option<TokenStream>,
    pub text: Option<TokenStream>,
    pub set_text: Option<TokenStream>,
    pub set_text_and_redraw: Option<TokenStream>,
    pub ref_cast_type_id: Option<TokenStream>,
}