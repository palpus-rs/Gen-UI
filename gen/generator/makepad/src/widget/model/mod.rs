use self::live_design::LiveDesign;

pub mod app_main;
pub mod widget;
pub mod live_design;
pub mod attr;
pub mod field;
pub mod match_event;
pub mod traits;

pub enum Model{
    AppMain(app_main::AppMain),
    Widget(widget::Widget),
}


