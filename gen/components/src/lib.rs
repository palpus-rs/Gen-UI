use makepad_widgets::Cx;

pub mod components;

pub fn live_design(cx: &mut Cx) {
    crate::components::label::live_design(cx);
    crate::components::live_design(cx);
}