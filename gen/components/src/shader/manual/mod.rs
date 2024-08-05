use makepad_widgets::*;

#[derive(Live, LiveHook)]
#[live_ignore]
#[repr(u32)]
pub enum Direction {
    #[pick] Horizontal = shader_enum(1),
    Vertical = shader_enum(2),
}