use makepad_widgets::Cx;

pub mod components;
pub mod themes;
pub mod macros;
pub mod shader;
pub mod utils;

pub fn live_design(cx: &mut Cx) {
    crate::components::label::live_design(cx);
    crate::components::button::live_design(cx);
    crate::components::card::live_design(cx);
    crate::components::link::live_design(cx);
    crate::components::icon::live_design(cx);
    crate::components::radio::live_design(cx);
    crate::shader::draw_button::live_design(cx);
    crate::shader::draw_card::live_design(cx);
    crate::shader::draw_link::live_design(cx);
    crate::shader::draw_text::live_design(cx);
    crate::shader::draw_radio::live_design(cx);
    crate::components::live_design(cx);
}