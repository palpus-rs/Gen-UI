mod app_main;
mod widget;



use app_main::AppMain;
use widget::Widget;
/// 使用WidgetInstance来实例化Widget
/// 这里当前存在2种情况
/// - 1. AppMainWidget
/// - 2. Widget
pub enum WidgetInstance {
    AppMain(AppMain),
    Widget(Widget),
}