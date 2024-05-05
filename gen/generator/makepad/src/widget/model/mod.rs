pub mod app_main;
pub mod attr;
pub mod field;
pub mod handler;
pub mod live_design;
pub mod match_event;
pub mod role;
pub mod traits;
pub mod widget;

#[derive(Debug, Clone)]
pub enum Model {
    AppMain(app_main::AppMain),
    Widget(widget::Widget),
}

impl Model {
    pub fn new(mut model: gen_converter::model::Model) -> Self {
        // 判断是否是AppMain
        match model.is_entry() {
            true => Model::AppMain(app_main::AppMain::from(model)),
            false => Model::Widget(widget::Widget::from(model)),
        }
    }
}
