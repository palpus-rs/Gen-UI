use gen_converter::{
    model::script::{ScriptBuilder, ScriptModel},
    strategy::{class, id, inherits, script, style},
};



use self::live_design::LiveDesign;

pub mod app_main;
pub mod attr;
pub mod field;
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
        match model.is_entry(){
            true => Model::AppMain(app_main::AppMain::from(model)),
            false =>Model::Widget(widget::Widget::from(model)),
        }















        // [这一部分是为了对Model进一步进行处理]-----------------------------------------------------
        // let _ = Model::handle_template(&mut model);
        // let _ = Model::handle_style(&mut model);
        // todo!("{:#?}",model);
        // let is_app_main = model.

        // [处理并生成script部分]------------------------------------------------------------------
        // let block = model.script.unwrap().to_origin();
        
        // let script_model = ScriptModel::new(block, &model.template.as_ref().unwrap().get_props_tree());
       
        // if let Ok(sc) = script(
        //     model,
        //     gen::r#use(),
        //     gen::prop(),
        //     gen::event(),
        //     gen::lifetime(),
        //     gen::other(),
        // ) {
        //     dbg!(sc);
        // };
        // todo!("{:#?}", script_model);
    }

    // // 处理template部分
    // fn handle_template(model: &mut gen_converter::model::Model) {
    //     let _ = id(model, gen::id());
    //     let _ = class(model, gen::class());
    //     let _ = inherits(model, gen::inherits());
    // }
    // // 处理style部分
    // fn handle_style(model: &mut gen_converter::model::Model) {
    //     let _ = style(model, gen::style());
    // }

}
