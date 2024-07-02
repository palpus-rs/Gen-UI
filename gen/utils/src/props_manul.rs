pub struct Font;

impl Font {
    /// 字体类型
    pub const FONT_FAMILY: &'static str = "font_family";
    /// 字体大小
    pub const FONT_SIZE: &'static str = "font_size";
    /// 字体粗细
    pub const FONT_WEIGHT: &'static str = "font_weight";
    /// 字体缩放
    pub const FONT_SCALE: &'static str = "font_scale";
    /// 字体亮度
    pub const BRIGHTNESS: &'static str = "brightness";
    /// 字体曲线
    pub const CURVE: &'static str = "curve";
    /// 字体行间距
    pub const LINE_SPACING: &'static str = "line_spacing";
    /// 起始字符高度
    pub const TOP_DROP: &'static str = "top_drop";
    /// 高度因子
    pub const HEIGHT_FACTOR: &'static str = "height_factor";
}

pub struct Text;

impl Text {
    /// 文本内容
    pub const TEXT: &'static str = "text";
    /// 文本深度
    pub const DRAW_DEPTH: &'static str = "draw_depth";
    /// 忽略换行
    pub const IGNORE_NEWLINES: &'static str = "ignore_newlines";
    /// 合并空格
    pub const COMBINE_SPACES: &'static str = "combine_spaces";
    /// 文本换行行为
    pub const TEXT_WRAP: &'static str = "text_wrap";
    /// 文本颜色
    pub const COLOR: &'static str = "color";
}

pub struct Size;

impl Size {
    /// 宽度
    pub const WIDTH: &'static str = "width";
    /// 高度
    pub const HEIGHT: &'static str = "height";
    /// 最小宽度
    pub const MIN_WIDTH: &'static str = "min_width";
    /// 最小高度
    pub const MIN_HEIGHT: &'static str = "min_height";
    /// 最大宽度
    pub const MAX_WIDTH: &'static str = "max_width";
    /// 最大高度
    pub const MAX_HEIGHT: &'static str = "max_height";
    /// 外边距
    pub const MARGIN: &'static str = "margin";
    /// 内边距
    pub const PADDING: &'static str = "padding";
    pub const CLIP_X: &'static str = "clip_x";
    pub const CLIP_Y: &'static str = "clip_y";
    /// 窗口大小
    pub const WINDOW_SIZE: &'static str = "window_size";
}

pub struct Position;

impl Position {
    /// 定位
    pub const ABS_POS: &'static str = "abs_pos";
    /// 子元素定位
    pub const ALIGN: &'static str = "align";
    /// 排序
    pub const FLOW: &'static str = "flow";
    /// 间距
    pub const SPACING: &'static str = "spacing";
    /// 窗口位置
    pub const WINDOW_POSITION: &'static str = "window_position";
}

pub struct Background;

impl Background{
    /// 背景颜色
    pub const BACKGROUND_COLOR: &'static str = "background_color";
    /// 显示背景
    pub const BACKGROUND_VISIBLE: &'static str = "background_visible";
}

pub struct Border;

impl Border {
    /// 边框颜色
    pub const BORDER_COLOR: &'static str = "border_color";
    /// 边框宽度
    pub const BORDER_WIDTH: &'static str = "border_width";
    /// 边框圆角
    pub const BORDER_RADIUS: &'static str = "border_radius";
}

pub struct Others;

impl Others {
    pub const VISIBLE: &'static str = "visible";
    pub const SCROLL: &'static str = "scroll";
    /// 优化方案
    pub const OPTIMIZE: &'static str = "optimize";
    /// 事件顺序
    pub const EVENT_ORDER: &'static str = "event_order";
    /// 事件透传
    pub const GRAB_KEY_FOCUS: &'static str = "grab_key_focus";
    /// 阻止事件
    pub const BLOCK_SIGNAL_EVENT: &'static str = "block_signal_event";
    /// 鼠标样式
    pub const CURSOR: &'static str = "cursor";
}