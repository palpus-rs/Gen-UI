//-------------[for Makepad::Walk::Size]----------------------------
pub const FILL: &str = "Fill";
pub const FIT: &str = "Fit";
pub const ALL: &str = "All";
//-------------[for Makepad::Flow]--------------------------------
pub const RIGHT: &str = "Right";
pub const DOWN: &str = "Down";
pub const OVERLAY: &str = "Overlay";
pub const RIGHTWRAP: &str = "RightWrap";
//-------------[for Makepad::ViewOptimize]--------------------------------
pub const NONE: &str = "None";
pub const DRAWLIST: &str = "DrawList";
pub const TEXTURE: &str = "Texture";
//-------------[for Makepad::EventOrder]--------------------------------
pub const UP: &str = "Up";
//-------------[for Makepad::Cursor]--------------------------------
pub const HIDDEN: &str = "Hidden";
pub const DEFAULT: &str = "Default";
pub const CROSSHAIR: &str = "Crosshair";
pub const HAND: &str = "Hand";
pub const ARROW: &str = "Arrow";
pub const MOVE: &str = "Move";
pub const TEXT: &str = "Text";
pub const WAIT: &str = "Wait";
pub const HELP: &str = "Help";
pub const NOT_ALLOWED: &str = "NotAllowed";
pub const N_RESIZE: &str = "NResize";
pub const NE_RESIZE: &str = "NeResize";
pub const E_RESIZE: &str = "EResize";
pub const SE_RESIZE: &str = "SeResize";
pub const S_RESIZE: &str = "SResize";
pub const SW_RESIZE: &str = "SwResize";
pub const W_RESIZE: &str = "WResize";
pub const NW_RESIZE: &str = "NwResize";
pub const NS_RESIZE: &str = "NsResize";
pub const NESW_RESIZE: &str = "NeswResize";
pub const EW_RESIZE: &str = "EwResize";
pub const NWSE_RESIZE: &str = "NwseResize";
pub const COL_RESIZE: &str = "ColResize";
pub const ROW_RESIZE: &str = "RowResize";
//-------------[for Makepad::TextWrap]----------------------------------
pub const ELLIPSIS: &str = "Ellipsis";
pub const WORD: &str = "Word";
pub const LINE: &str = "Line";
//-------------[for Makepad live design macro bind]--------------------------------
pub const BIND_IMPORT:&str = "use makepad_widgets::*;\nlive_design!{\nimport makepad_widgets::base::*;\nimport makepad_widgets::theme_desktop_dark::*;";

pub const IMPL_WIDGET:&str = "impl Widget for";
pub const FN_DRAW_WALK:&str = "fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {";
pub const DRAW_STEP_DONE:&str = "DrawStep::done() }";
pub const FN_HANDLE_EVENT:&str = "fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {";


pub const LIVE_REGISTER: &str =
    "fn live_register(cx: &mut Cx) {crate::makepad_widgets::live_design(cx);}";
pub const APP_MAIN: &str = "fn handle_event(&mut self, cx: &mut Cx, event: &Event)";
