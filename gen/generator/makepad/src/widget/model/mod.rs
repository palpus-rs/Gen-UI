use gen_converter::strategy::{id,class,inherits,style};

use crate::gen;

use self::live_design::LiveDesign;

pub mod app_main;
pub mod widget;
pub mod live_design;
pub mod attr;
pub mod field;
pub mod match_event;
pub mod traits;

#[derive(Debug, Clone)]
pub enum Model{
    AppMain(app_main::AppMain),
    Widget(widget::Widget),
}

impl Model {
    pub fn new(mut model: gen_converter::model::Model)->Self{
         // [这一部分是为了对Model进一步进行处理]-----------------------------------------------------
        // 处理template部分
        let _ = id(&mut model, gen::id());
        let _ = class(&mut model, gen::class());
        let _ = inherits(&mut model, gen::inherits());
        // 处理style部分
        let _ = style(&mut model, gen::style());
        // [处理并生成script部分]------------------------------------------------------------------

        if let Ok(sc) = script(
            model,
            gen::r#use(),
            gen::prop(),
            gen::event(),
            gen::lifetime(),
            gen::other(),
        ) {
            dbg!(sc);
        }


    }
}