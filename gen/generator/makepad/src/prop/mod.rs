mod enum_ident;
pub mod builtin;

pub use enum_ident::*;


// -------------------- appearance ----------------
pub const VISIBLE: &str = "visible";
pub const HEIGHT: &str = "height";
pub const WIDTH: &str = "width";
pub const MARGIN: &str = "margin";
pub const PADDING: &str = "padding";
pub const ALIGN: &str = "align";
pub const SPACING: &str = "spacing";
pub const LINE_SPACING: &str = "line_spacing";
pub const CLIP_X: &str = "clip_x";
pub const CLIP_Y: &str = "clip_y";
pub const FLOW: &str = "flow";
pub const SCALE: &str = "scale";
// -------------------- color --------------------
pub const COLOR: &str = "color";
pub const BRIGHTNESS: &str = "brightness";
pub const DRAW_BG: &str = "draw_bg";
pub const SHOW_BG: &str = "show_bg";
// -------------------- font ---------------------
pub const FONT: &str = "font";
pub const TEXT: &str = "text";
pub const FONT_SCALE: &str = "font_scale";
pub const FONT_SIZE: &str = "font_size";
pub const DRAW_DEPTH:&str = "draw_depth";
pub const DRAW_TEXT:&str = "draw_text";
pub const TEXT_STYLE:&str = "text_style";
pub const INGORE_NEWLINES :&str = "ignore_newlines";
pub const COMBINE_SPACES :&str = "combine_spaces";
// -------------------- action -------------------
/// maybe layout wrap or textwrap
pub const WRAP:&str = "wrap";
pub const EVENT_ORDER: &str = "event_order";
pub const GRAB_KEY_FOCUS: &str = "grab_key_focus";
pub const BLOCK_SIGNAL_EVENT: &str = "block_signal_event";
// -------------------- position -----------------
pub const ABS_POS: &str = "abs_pos";
// -------------------- other --------------------
pub const OPTIMIZE: &str = "optimize";
pub const CURSOR: &str = "cursor";
pub const SCROLL_BARS: &str = "scroll_bars";
pub const SCROLL: &str = "scroll";
pub const CURVE: &str = "curve";
pub const TOP_DROP: &str = "top_drop";
pub const HEIGHT_FACTOR: &str = "height_factor";
pub const SVG_PATH: &str = "svg_path";
pub const SVG_FILE: &str = "svg_file";
pub const LINEARIZE: &str = "linearize";
pub const DRAW_ICON: &str = "draw_icon";
pub const ICON_WALK: &str = "icon_walk";
pub const LABEL_WALK: &str = "label_walk";