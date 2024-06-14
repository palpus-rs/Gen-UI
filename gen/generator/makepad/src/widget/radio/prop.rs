#[allow(unused_imports)]
use std::{default, fmt::Display};

use gen_converter::error::Errors;
use gen_parser::Value;
use proc_macro2::TokenStream;

use crate::{
    prop::{
        builtin::{
            draw_icon::DrawIcon, draw_radio_button::DrawRadioButton, draw_text::DrawText, Align,
            Layout, Walk,
        },
        ABS_POS, ALIGN, BRIGHTNESS, CLIP_X, CLIP_Y, COLOR, COMBINE_SPACES, CURVE, DRAW_DEPTH,
        DRAW_ICON, DRAW_TEXT, FLOW, FONT, FONT_SCALE, FONT_SIZE, HEIGHT, HEIGHT_FACTOR, ICON_WALK,
        INGORE_NEWLINES, LINEARIZE, LINE_SPACING, MARGIN, PADDING, SCALE, SCROLL, SPACING,
        SVG_FILE, TOP_DROP, WIDTH, WRAP,
    },
    props_to_token, str_to_string_try_from,
    widget::{
        prop_ignore,
        utils::{bind_prop_value, quote_prop, string_prop},
        DynProps, StaticProps,
    },
    ToToken,
};

enum NodeType {
    Icon,
    Label,
    Outer,
}

const IMAGE: &str = "Image";
const ICON: &str = "Icon";
const NONE: &str = "None";
const LABEL_WALK: &str = "label_walk";

#[derive(Debug, Clone, Default)]
pub enum MediaType {
    Image,
    #[default]
    Icon,
    None,
}

impl TryFrom<&str> for MediaType {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            IMAGE => Ok(MediaType::Image),
            ICON => Ok(MediaType::Icon),
            NONE => Ok(MediaType::None),
            _ => Err(Errors::PropConvertFail(format!(
                "{} cannot be converted to MediaType!",
                value
            ))),
        }
    }
}

str_to_string_try_from!(MediaType);

impl TryFrom<&Value> for MediaType {
    type Error = Errors;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if let Some(s) = value.is_unknown_and_get() {
            s.try_into()
        } else {
            value
                .is_string_and_get()
                .map(|s| s.try_into())
                .unwrap_or_else(|| {
                    Err(Errors::PropConvertFail(format!(
                        "{:?} cannot be converted to Makepad::MediaType!",
                        value
                    )))
                })
        }
    }
}

impl Display for MediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MediaType::Image => f.write_str(IMAGE),
            MediaType::Icon => f.write_str(ICON),
            MediaType::None => f.write_str(NONE),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct RadioButtonProps {
    pub draw_radio: Option<DrawRadioButton>,
    // todo!(DrawQuad pixel())
    // pub draw_bg: Option<DrawQuad>,
    pub draw_icon: Option<DrawIcon>,
    pub draw_text: Option<DrawText>,
    // pub value: Option<LiveValue>
    pub value: Option<String>,
    pub media: Option<MediaType>,
    pub icon_walk: Option<Walk>,
    pub walk: Option<Walk>,
    // pub image: Image
    pub layout: Option<Layout>,
    pub label_walk: Option<Walk>,
    pub label_align: Option<Align>,
    pub label: Option<String>,
    pub bind: Option<String>,
}

impl DynProps for RadioButtonProps {
    fn prop_bind(
        prop: &gen_parser::PropsKey,
        value: &gen_parser::Value,
        is_prop: bool,
        ident: &str,
    ) -> proc_macro2::TokenStream {
        let value = bind_prop_value(value, is_prop, ident);
        match prop.name() {
            // ----------------- draw_icon ---------------
            "icon_brightness" => quote_prop(vec![DRAW_ICON, BRIGHTNESS], &value),
            "icon_curve" => quote_prop(vec![DRAW_ICON, CURVE], &value),
            LINEARIZE => quote_prop(vec![DRAW_ICON, LINEARIZE], &value),
            SVG_FILE => quote_prop(vec![DRAW_ICON, SVG_FILE], &value),
            SCALE => quote_prop(vec![DRAW_ICON, SCALE], &value),
            "icon_draw_depth" => quote_prop(vec![DRAW_ICON, DRAW_DEPTH], &value),
            "icon_color" => quote_prop(vec![DRAW_ICON, COLOR], &value),
            // ----------------- draw_text -----------------
            FONT => quote_prop(vec![DRAW_TEXT, FONT], &value),
            FONT_SIZE => quote_prop(vec![DRAW_TEXT, FONT_SIZE], &value),
            BRIGHTNESS => quote_prop(vec![DRAW_TEXT, BRIGHTNESS], &value),
            CURVE => quote_prop(vec![DRAW_TEXT, CURVE], &value),
            "label_line_spacing" => quote_prop(vec![DRAW_TEXT, LINE_SPACING], &value),
            TOP_DROP => quote_prop(vec![DRAW_TEXT, TOP_DROP], &value),
            HEIGHT_FACTOR => quote_prop(vec![DRAW_TEXT, HEIGHT_FACTOR], &value),
            WRAP => quote_prop(vec![DRAW_TEXT, WRAP], &value),
            INGORE_NEWLINES => quote_prop(vec![DRAW_TEXT, INGORE_NEWLINES], &value),
            COMBINE_SPACES => quote_prop(vec![DRAW_TEXT, COMBINE_SPACES], &value),
            FONT_SCALE => quote_prop(vec![DRAW_TEXT, FONT_SCALE], &value),
            DRAW_DEPTH => quote_prop(vec![DRAW_TEXT, DRAW_DEPTH], &value),
            COLOR => quote_prop(vec![DRAW_TEXT, COLOR], &value),
            // ----------------- icon_walk ---------------
            "icon_height" => quote_prop(vec![ICON_WALK, HEIGHT], &value),
            "icon_width" => quote_prop(vec![ICON_WALK, WIDTH], &value),
            "icon_abs_pos" => quote_prop(vec![ICON_WALK, ABS_POS], &value),
            "icon_margin" => quote_prop(vec![ICON_WALK, MARGIN], &value),
            // ----------------- label walk -----------------
            "label_height" => quote_prop(vec![LABEL_WALK, HEIGHT], &value),
            "label_width" => quote_prop(vec![LABEL_WALK, WIDTH], &value),
            "label_abs_pos" => quote_prop(vec![LABEL_WALK, ABS_POS], &value),
            "label_margin" => quote_prop(vec![LABEL_WALK, MARGIN], &value),
            // ----------------- walk -----------------
            HEIGHT => quote_prop(vec![HEIGHT], &value),
            WIDTH => quote_prop(vec![WIDTH], &value),
            ABS_POS => quote_prop(vec![ABS_POS], &value),
            MARGIN => quote_prop(vec![MARGIN], &value),
            // ------------------- layout -----------------
            SCROLL => quote_prop(vec![SCROLL], &value),
            CLIP_X => quote_prop(vec![CLIP_X], &value),
            CLIP_Y => quote_prop(vec![CLIP_Y], &value),
            PADDING => quote_prop(vec![PADDING], &value),
            ALIGN => quote_prop(vec![ALIGN], &value),
            FLOW => quote_prop(vec![FLOW], &value),
            SPACING => quote_prop(vec![SPACING], &value),
            LINE_SPACING => quote_prop(vec![LINE_SPACING], &value),
            // ----------------- other ------------------
            "label_align" => quote_prop(vec!["label_align"], &value),
            "bind" => quote_prop(vec!["bind"], &value),
            // "label" => self.label(&value),
            "text" => quote_prop(vec!["text"], &value),
            "radio_type" => quote_prop(vec!["radio_type"], &value),
            "media" => quote_prop(vec!["media"], &value),
            _ => panic!("cannot match prop in BuiltIn Icon"),
        }
    }
}

impl StaticProps for RadioButtonProps {
    fn props(props: &std::collections::HashMap<gen_parser::PropsKey, gen_parser::Value>) -> Self
    where
        Self: Sized,
    {
        let mut radio = RadioButtonProps::default();
        for (k, v) in props {
            radio.prop(k.name(), v.clone())
        }
        radio
    }

    fn prop(&mut self, prop_name: &str, value: gen_parser::Value) -> () {
        let _ = match prop_name {
            // ----------------- draw_icon ---------------
            "icon_brightness" => self.brightness(&value, NodeType::Icon),
            "icon_curve" => self.curve(&value, NodeType::Icon),
            LINEARIZE => self.linearize(&value),
            SVG_FILE => self.svg_file(&value),
            SCALE => self.scale(&value),
            "icon_draw_depth" => self.draw_depth(&value, NodeType::Icon),
            "icon_color" => self.color(&value, NodeType::Icon),
            // ----------------- draw_text -----------------
            FONT => self.font(&value),
            FONT_SIZE => self.font_size(&value),
            BRIGHTNESS => self.brightness(&value, NodeType::Label),
            CURVE => self.curve(&value, NodeType::Label),
            "label_line_spacing" => self.line_spacing(&value, NodeType::Label),
            TOP_DROP => self.top_drop(&value),
            HEIGHT_FACTOR => self.height_factor(&value),
            WRAP => self.wrap(&value),
            INGORE_NEWLINES => self.ignore_newlines(&value),
            COMBINE_SPACES => self.combine_spaces(&value),
            FONT_SCALE => self.font_scale(&value),
            DRAW_DEPTH => self.draw_depth(&value, NodeType::Label),
            COLOR => self.color(&value, NodeType::Label),
            // ----------------- icon_walk ---------------
            "icon_height" => self.height(&value, NodeType::Icon),
            "icon_width" => self.width(&value, NodeType::Icon),
            "icon_abs_pos" => self.abs_pos(&value, NodeType::Icon),
            "icon_margin" => self.margin(&value, NodeType::Icon),
            // ----------------- label walk -----------------
            "label_height" => self.height(&value, NodeType::Label),
            "label_width" => self.width(&value, NodeType::Label),
            "label_abs_pos" => self.abs_pos(&value, NodeType::Label),
            "label_margin" => self.margin(&value, NodeType::Label),
            // ----------------- walk -----------------
            HEIGHT => self.height(&value, NodeType::Outer),
            WIDTH => self.width(&value, NodeType::Outer),
            ABS_POS => self.abs_pos(&value, NodeType::Outer),
            MARGIN => self.margin(&value, NodeType::Outer),
            // ----------------- layout ---------------
            SCROLL => self.scroll(&value),
            CLIP_X => self.clip_x(&value),
            CLIP_Y => self.clip_y(&value),
            PADDING => self.padding(&value),
            ALIGN => self.align(&value, NodeType::Outer),
            FLOW => self.flow(&value),
            SPACING => self.spacing(&value),
            LINE_SPACING => self.line_spacing(&value, NodeType::Outer),
            // ----------------- other ------------------
            "label_align" => self.align(&value, NodeType::Label),
            "bind" => self.bind(&value),
            // "label" => self.label(&value),
            "text" => self.label(&value),
            "radio_type" => self.radio_type(&value),
            "media" => self.media(&value),
            _ => {
                if !prop_ignore(prop_name) {
                    panic!("cannot match prop");
                } else {
                    panic!("unslolved prop");
                }
            }
        };
    }
}

#[allow(dead_code)]
impl RadioButtonProps {
    fn bind(&mut self, value: &Value) -> Result<(), Errors> {
        string_prop(value, |s| {
            let _ = self.bind.replace(s.to_string());
        })
    }
    fn radio_type(&mut self, value: &Value) -> Result<(), Errors> {
        self.draw_radio.as_mut().unwrap().radio_type(value)
    }
    fn media(&mut self, value: &Value) -> Result<(), Errors> {
        self.media.replace(value.try_into()?);
        Ok(())
    }
    fn check_draw_icon(&mut self) -> &mut DrawIcon {
        if self.draw_icon.is_none() {
            self.draw_icon = Some(DrawIcon::default());
        }
        self.draw_icon.as_mut().unwrap()
    }
    fn check_draw_text(&mut self) -> &mut DrawText {
        if self.draw_text.is_none() {
            self.draw_text = Some(DrawText::default());
        }
        self.draw_text.as_mut().unwrap()
    }
    fn check_icon_walk(&mut self) -> &mut Walk {
        if self.icon_walk.is_none() {
            self.icon_walk = Some(Walk::default());
        }
        self.icon_walk.as_mut().unwrap()
    }
    fn check_label_walk(&mut self) -> &mut Walk {
        if self.label_walk.is_none() {
            self.label_walk = Some(Walk::default());
        }
        self.label_walk.as_mut().unwrap()
    }
    fn check_walk(&mut self) -> &mut Walk {
        if self.walk.is_none() {
            self.walk = Some(Walk::default());
        }
        self.walk.as_mut().unwrap()
    }
    fn check_layout(&mut self) -> &mut Layout {
        if self.layout.is_none() {
            self.layout = Some(Layout::default());
        }
        self.layout.as_mut().unwrap()
    }
    fn font(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().font(value)
    }
    fn font_size(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().font_size(value)
    }

    fn curve(&mut self, value: &Value, ty: NodeType) -> Result<(), Errors> {
        match ty {
            NodeType::Icon => self.check_draw_icon().curve(value),
            NodeType::Label => self.check_draw_text().curve(value),
            NodeType::Outer => Err(Errors::PropConvertFail(format!(
                "Makepad RadioButton has no outer curve! (icon_curve|curve)",
            ))),
        }
    }
    fn line_spacing(&mut self, value: &Value, ty: NodeType) -> Result<(), Errors> {
        match ty {
            NodeType::Icon => Err(Errors::PropConvertFail(format!(
                "Makepad RadioButton has no icon line_spacing! (label_line_spacing|line_spacing)",
            ))),
            NodeType::Label => self.check_draw_text().line_spacing(value),
            NodeType::Outer => self.check_layout().line_spacing(value),
        }
    }
    fn top_drop(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().top_drop(value)
    }
    fn height_factor(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().height_factor(value)
    }
    fn wrap(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().wrap(value)
    }
    fn ignore_newlines(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().ignore_newlines(value)
    }
    fn combine_spaces(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().combine_spaces(value)
    }
    fn font_scale(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().font_scale(value)
    }
    fn draw_depth(&mut self, value: &Value, ty: NodeType) -> Result<(), Errors> {
        match ty {
            NodeType::Icon => self.check_draw_icon().draw_depth(value),
            NodeType::Label => self.check_draw_text().draw_depth(value),
            NodeType::Outer => Err(Errors::PropConvertFail(format!(
                "Makepad RadioButton has no outer draw_depth! (icon_draw_depth|draw_depth)",
            ))),
        }
    }
    fn color(&mut self, value: &Value, ty: NodeType) -> Result<(), Errors> {
        match ty {
            NodeType::Icon => self.check_draw_icon().color(value),
            NodeType::Label => self.check_draw_text().color(value),
            NodeType::Outer => Err(Errors::PropConvertFail(format!(
                "Makepad RadioButton has no outer color! (icon_color|color)",
            ))),
        }
    }

    fn label(&mut self, value: &Value) -> Result<(), Errors> {
        string_prop(value, |s| {
            let _ = self.label.replace(s.to_string());
        })
    }
    fn brightness(&mut self, value: &Value, ty: NodeType) -> Result<(), Errors> {
        match ty {
            NodeType::Icon => self.check_draw_icon().brightness(value),
            NodeType::Label => self.check_draw_text().brightness(value),
            NodeType::Outer => Err(Errors::PropConvertFail(format!(
                "Makepad RadioButton has no outer brightness! (icon_brightness|brightness)",
            ))),
        }
    }

    fn linearize(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_icon().linearize(value)
    }
    fn svg_file(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_icon().svg_file(value)
    }
    fn scale(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_icon().scale(value)
    }

    fn height(&mut self, value: &Value, ty: NodeType) -> Result<(), Errors> {
        match ty {
            NodeType::Label => self.check_label_walk().height(value),
            NodeType::Icon => self.check_icon_walk().height(value),
            NodeType::Outer => self.check_walk().height(value),
        }
    }
    fn width(&mut self, value: &Value, ty: NodeType) -> Result<(), Errors> {
        match ty {
            NodeType::Label => self.check_label_walk().width(value),
            NodeType::Icon => self.check_icon_walk().width(value),
            NodeType::Outer => self.check_walk().width(value),
        }
    }
    fn abs_pos(&mut self, value: &Value, ty: NodeType) -> Result<(), Errors> {
        match ty {
            NodeType::Label => self.check_label_walk().abs_pos(value),
            NodeType::Icon => self.check_icon_walk().abs_pos(value),
            NodeType::Outer => self.check_walk().abs_pos(value),
        }
    }
    fn margin(&mut self, value: &Value, ty: NodeType) -> Result<(), Errors> {
        match ty {
            NodeType::Label => self.check_label_walk().margin(value),
            NodeType::Icon => self.check_icon_walk().margin(value),
            NodeType::Outer => self.check_walk().margin(value),
        }
    }
    fn scroll(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().scroll(value)
    }
    fn clip_x(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().clip_x(value)
    }
    fn clip_y(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().clip_y(value)
    }
    fn padding(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().padding(value)
    }
    fn flow(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().flow(value)
    }
    fn spacing(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().spacing(value)
    }

    fn align(&mut self, value: &Value, ty: NodeType) -> Result<(), Errors> {
        match ty {
            NodeType::Icon => Err(Errors::PropConvertFail(format!(
                "Makepad RadioButton has no icon align! (icon_align|align)",
            ))),
            NodeType::Label => {
                self.label_align.replace(Align::try_from(value)?);
                Ok(())
            }
            NodeType::Outer => self.check_layout().align(value),
        }
    }
}

impl Display for RadioButtonProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(draw_radio) = &self.draw_radio {
            let _ = f.write_fmt(format_args!("{}: {{{}}},", "draw_radio", draw_radio));
        }
        if let Some(draw_icon) = &self.draw_icon {
            let _ = f.write_fmt(format_args!("{}: {{{}}},", DRAW_ICON, draw_icon));
        }
        if let Some(draw_text) = &self.draw_text {
            let _ = f.write_fmt(format_args!("{}: {{{}}},", DRAW_TEXT, draw_text));
        }
        if let Some(value) = self.value.as_ref() {
            let _ = f.write_fmt(format_args!("value: \"{}\",", value));
        }
        if let Some(media) = self.media.as_ref() {
            let _ = f.write_fmt(format_args!("media: {},", media));
        }
        if let Some(icon_walk) = &self.icon_walk {
            let _ = f.write_fmt(format_args!("{}: {{{}}},", ICON_WALK, icon_walk));
        }
        if let Some(walk) = &self.walk {
            let _ = f.write_fmt(format_args!("{},", walk));
        }
        if let Some(layout) = &self.layout {
            let _ = f.write_fmt(format_args!("{},", layout));
        }
        if let Some(label_walk) = self.label_walk.as_ref() {
            let _ = f.write_fmt(format_args!("{}: {{{}}},", "label_walk", label_walk));
        }
        if let Some(label_align) = self.label_align.as_ref() {
            let _ = f.write_fmt(format_args!("label_align: {},", label_align));
        }
        if let Some(label) = &self.label {
            let _ = f.write_fmt(format_args!("text: \"{}\",", label));
            // let _ = f.write_fmt(format_args!("label: \"{}\",", label));
        }
        if let Some(bind) = self.bind.as_ref() {
            let _ = f.write_fmt(format_args!("bind: \"{}\",", bind));
        }
        write!(f, "")
    }
}

props_to_token!(RadioButtonProps);