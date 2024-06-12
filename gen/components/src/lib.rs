use makepad_widgets::Cx;

pub mod components;
pub mod themes;
pub mod macros;
pub mod shader;

pub fn live_design(cx: &mut Cx) {
    crate::components::label::live_design(cx);
    crate::components::button::live_design(cx);
    crate::shader::bg_button::live_design(cx);
    crate::components::live_design(cx);
}