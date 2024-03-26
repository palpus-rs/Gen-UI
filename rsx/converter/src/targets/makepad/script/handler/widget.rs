
use crate::{context::RIGHT_HOLDER, targets::makepad::constants::{DRAW_STEP_DONE, FN_DRAW_WALK, FN_HANDLE_EVENT, IMPL_WIDGET}};

pub fn build_handle_event(handle_event:&str) -> String{
    format!("{} {} {}",FN_HANDLE_EVENT, handle_event,RIGHT_HOLDER)
}

pub fn build_draw_walk(draw_walk:&str)->String{
    format!("{} {} {}", FN_DRAW_WALK,draw_walk,DRAW_STEP_DONE)
}

pub fn build_widget_trait(widget:&str, widget_events:Vec<String>)->String{
    format!(
        "{} {} {{ {} }}",
        IMPL_WIDGET,widget,widget_events.join(" ")
    )
}