use std::fmt::Display;

use gen_converter::error::Errors;
use gen_parser::Value;
use proc_macro2::TokenStream;

use crate::{
    prop::{
        builtin::{draw_color::DrawColor, draw_icon::DrawIcon, draw_text::DrawText, Layout, Walk},
        ABS_POS, ALIGN, CLIP_X, CLIP_Y, COLOR, COMBINE_SPACES, DRAW_BG, DRAW_ICON, DRAW_TEXT, FLOW,
        FONT, FONT_SCALE, FONT_SIZE, GRAB_KEY_FOCUS, HEIGHT, HEIGHT_FACTOR, ICON_WALK,
        INGORE_NEWLINES, LABEL_WALK, LINEARIZE, LINE_SPACING, MARGIN, PADDING, SCALE, SCROLL,
        SPACING, SVG_FILE, TEXT, TOP_DROP, WIDTH, WRAP,
    },
    widget::{
        prop_ignore,
        utils::{bool_prop, string_prop},
        StaticProps,
    },
    ToToken,
};

enum NodeType {
    Icon,
    Label,
    Button,
}

#[derive(Debug, Clone, Default)]
pub struct ButtonProps {
    pub draw_bg: Option<DrawColor>,
    pub draw_text: Option<DrawText>,
    pub draw_icon: Option<DrawIcon>,
    pub icon_walk: Option<Walk>,
    pub label_walk: Option<Walk>,
    pub walk: Option<Walk>,
    pub layout: Option<Layout>,
    pub grab_key_focus: Option<bool>,
    pub text: Option<String>,
}

impl StaticProps for ButtonProps {
    fn props(props: &std::collections::HashMap<gen_parser::PropsKey, gen_parser::Value>) -> Self
    where
        Self: Sized,
    {
        let mut btn = ButtonProps::default();
        for (k, v) in props {
            btn.prop(k.name(), v.clone())
        }
        btn
    }

    fn prop(&mut self, prop_name: &str, value: gen_parser::Value) -> () {
        let _ = match prop_name {
            // ----------------- draw_bg -----------------
            DRAW_BG => self.draw_bg(&value),
            // ----------------- draw_text ---------------
            //      ----------------- text_style
            FONT => self.font(&value),
            FONT_SIZE => self.font_size(&value),
            "font_brightness" => self.brightness(&value, NodeType::Label),
            "font_curve" => self.curve(&value, NodeType::Label),
            "font_line_spacing" => self.line_spacing(&value, NodeType::Label),
            TOP_DROP => self.top_drop(&value),
            HEIGHT_FACTOR => self.height_factor(&value),
            //      ----------------- other
            WRAP => self.wrap(&value),
            INGORE_NEWLINES => self.ignore_newlines(&value),
            COMBINE_SPACES => self.combine_spaces(&value),
            FONT_SCALE => self.font_scale(&value),
            "font_draw_depth" => self.draw_depth(&value, NodeType::Label),
            "font_color" => self.color(&value, NodeType::Label),
            // ----------------- draw_icon ---------------
            "icon_brightness" => self.brightness(&value, NodeType::Icon),
            "icon_curve" => self.curve(&value, NodeType::Icon),
            LINEARIZE => self.linearize(&value),
            SVG_FILE => self.svg_file(&value),
            SCALE => self.scale(&value),
            "icon_draw_depth" => self.draw_depth(&value, NodeType::Icon),
            "icon_color" => self.color(&value, NodeType::Icon),
            // ----------------- icon_walk ---------------
            "icon_height" => self.height(&value, NodeType::Icon),
            "icon_width" => self.width(&value, NodeType::Icon),
            "icon_abs_pos" => self.abs_pos(&value, NodeType::Icon),
            "icon_margin" => self.margin(&value, NodeType::Icon),
            // ----------------- label_walk ---------------
            "label_height" => self.height(&value, NodeType::Label),
            "label_width" => self.width(&value, NodeType::Label),
            "label_abs_pos" => self.abs_pos(&value, NodeType::Label),
            "label_margin" => self.margin(&value, NodeType::Label),
            // ----------------- walk -----------------
            HEIGHT => self.height(&value, NodeType::Button),
            WIDTH => self.width(&value, NodeType::Button),
            ABS_POS => self.abs_pos(&value, NodeType::Button),
            MARGIN => self.margin(&value, NodeType::Button),
            // ------------------- layout -----------------
            SCROLL => self.scroll(&value),
            CLIP_X => self.clip_x(&value),
            CLIP_Y => self.clip_y(&value),
            PADDING => self.padding(&value),
            ALIGN => self.align(&value),
            FLOW => self.flow(&value),
            SPACING => self.spacing(&value),
            LINE_SPACING => self.line_spacing(&value, NodeType::Button),
            // ------------------- other ------------------
            GRAB_KEY_FOCUS => self.grab_key_focus(&value),
            TEXT => self.text(&value),
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

impl ToToken for ButtonProps {
    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        self.to_string().parse::<TokenStream>().unwrap()
    }
}

impl ButtonProps {
    fn draw_bg(&mut self, value: &Value) -> Result<(), Errors> {
        self.draw_bg = Some((value, false).try_into()?);
        Ok(())
    }
    fn check_draw_text(&mut self) -> &mut DrawText {
        if self.draw_text.is_none() {
            self.draw_text = Some(DrawText::default());
        }
        self.draw_text.as_mut().unwrap()
    }
    fn check_draw_icon(&mut self) -> &mut DrawIcon {
        if self.draw_icon.is_none() {
            self.draw_icon = Some(DrawIcon::default());
        }
        self.draw_icon.as_mut().unwrap()
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
    fn grab_key_focus(&mut self, value: &Value) -> Result<(), Errors> {
        bool_prop(value, |b| {
            self.grab_key_focus = Some(b);
        })
    }
    fn text(&mut self, value: &Value) -> Result<(), Errors> {
        string_prop(value, |s| {
            self.text = Some(s.to_string());
        })
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
    fn line_spacing(&mut self, value: &Value, ty: NodeType) -> Result<(), Errors> {
        match ty {
            NodeType::Icon => Ok(()),
            NodeType::Label => self.check_draw_text().line_spacing(value),
            NodeType::Button => self.check_layout().line_spacing(value),
        }
    }
    fn align(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().align(value)
    }
    fn height(&mut self, value: &Value, ty: NodeType) -> Result<(), Errors> {
        match ty {
            NodeType::Icon => self.check_icon_walk().height(value),
            NodeType::Label => self.check_label_walk().height(value),
            NodeType::Button => self.check_walk().height(value),
        }
    }
    fn width(&mut self, value: &Value, ty: NodeType) -> Result<(), Errors> {
        match ty {
            NodeType::Icon => self.check_icon_walk().width(value),
            NodeType::Label => self.check_label_walk().width(value),
            NodeType::Button => self.check_walk().width(value),
        }
    }
    fn abs_pos(&mut self, value: &Value, ty: NodeType) -> Result<(), Errors> {
        match ty {
            NodeType::Icon => self.check_icon_walk().abs_pos(value),
            NodeType::Label => self.check_label_walk().abs_pos(value),
            NodeType::Button => self.check_walk().abs_pos(value),
        }
    }
    fn margin(&mut self, value: &Value, ty: NodeType) -> Result<(), Errors> {
        match ty {
            NodeType::Icon => self.check_icon_walk().margin(value),
            NodeType::Label => self.check_label_walk().margin(value),
            NodeType::Button => self.check_walk().margin(value),
        }
    }
    fn color(&mut self, value: &Value, ty: NodeType) -> Result<(), Errors> {
        match ty {
            NodeType::Icon => self.check_draw_icon().color(value),
            NodeType::Label => self.check_draw_text().color(value),
            NodeType::Button => Ok(()),
        }
    }
    fn draw_depth(&mut self, value: &Value, ty: NodeType) -> Result<(), Errors> {
        match ty {
            NodeType::Icon => self.check_draw_icon().draw_depth(value),
            NodeType::Label => self.check_draw_text().draw_depth(value),
            NodeType::Button => Ok(()),
        }
    }
    fn brightness(&mut self, value: &Value, ty: NodeType) -> Result<(), Errors> {
        match ty {
            NodeType::Icon => self.check_draw_icon().brightness(value),
            NodeType::Label => self.check_draw_text().brightness(value),
            NodeType::Button => Ok(()),
        }
    }
    fn curve(&mut self, value: &Value, ty: NodeType) -> Result<(), Errors> {
        match ty {
            NodeType::Icon => self.check_draw_icon().curve(value),
            NodeType::Label => self.check_draw_text().curve(value),
            NodeType::Button => Ok(()),
        }
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
    fn svg_file(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_icon().svg_file(value)
    }
    fn scale(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_icon().scale(value)
    }
    fn linearize(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_icon().linearize(value)
    }
    fn font(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().font(value)
    }
    fn font_size(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().font_size(value)
    }
    fn height_factor(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().height_factor(value)
    }
    fn top_drop(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_text().top_drop(value)
    }
}

impl Display for ButtonProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(draw_bg) = &self.draw_bg {
            let _ = f.write_fmt(format_args!("{}: {{ {}: {}}},", DRAW_BG, COLOR, draw_bg));
        }
        if let Some(draw_text) = &self.draw_text {
            let _ = f.write_fmt(format_args!("{}: {{{}}},", DRAW_TEXT, draw_text));
        }
        if let Some(draw_icon) = &self.draw_icon {
            let _ = f.write_fmt(format_args!("{}: {{{}}},", DRAW_ICON, draw_icon));
        }
        if let Some(icon_walk) = &self.icon_walk {
            let _ = f.write_fmt(format_args!("{}: {{{}}},", ICON_WALK, icon_walk));
        }
        if let Some(label_walk) = &self.label_walk {
            let _ = f.write_fmt(format_args!("{}: {{{}}},", LABEL_WALK, label_walk));
        }
        if let Some(walk) = &self.walk {
            let _ = f.write_fmt(format_args!("{},", walk));
        }
        if let Some(layout) = &self.layout {
            let _ = f.write_fmt(format_args!("{},", layout));
        }
        if let Some(grab_key_focus) = &self.grab_key_focus {
            let _ = f.write_fmt(format_args!("{}: {},", GRAB_KEY_FOCUS, grab_key_focus));
        }
        if let Some(text) = &self.text {
            let _ = f.write_fmt(format_args!("{}: \"{}\",", TEXT, text));
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod test_button_props {
    use super::*;

    #[test]
    fn to_tk() {
        let mut button = ButtonProps::default();
        button.draw_bg = Some("#ddd".try_into().unwrap());
        button.text = Some("hello".to_string());
        let mut draw_text = DrawText::default();
        draw_text.color = Some("#fff".try_into().unwrap());
        draw_text.wrap = Some("Word".try_into().unwrap());
        button.draw_text = Some(draw_text);
        let mut draw_icon = DrawIcon::default();
        draw_icon.brightness = Some(0.5);
        draw_icon.svg_file = Some(
            "crate://self/resources/icons/Icon_Search.svg"
                .try_into()
                .unwrap(),
        );
        button.draw_icon = Some(draw_icon);
        let tk = button.to_token_stream();
        let prop = "draw_bg : { color : # dddddd } , draw_text : { wrap : Word , color : { # ffffff } , } , draw_icon : { brightness : 0.5 , svg_file : dep (\"crate://self/resources/icons/Icon_Search.svg\") , } , text : \"hello\" ,";
        assert_eq!(tk.to_string().as_str(), prop);
    }
}
