use std::fmt::Display;

use gen_parser::Value;
use gen_utils::{
    error::Errors,
    props_manul::{Others, Position, Size},
};
use proc_macro2::TokenStream;

use crate::{
    prop::{
        builtin::{draw_icon::DrawIcon, Layout, Walk},
        ABS_POS, ALIGN, BRIGHTNESS, CLIP_X, CLIP_Y, COLOR, CURVE, DRAW_DEPTH, DRAW_ICON, FLOW,
        HEIGHT, ICON_WALK, LINEARIZE, LINE_SPACING, MARGIN, PADDING, SCALE, SCROLL, SPACING,
        SVG_FILE, WIDTH,
    },
    props_to_token,
    widget::{
        prop_ignore,
        utils::{bind_prop_value, quote_prop},
        DynProps, StaticProps,
    },
    ToToken,
};

enum NodeType {
    Inner,
    Outer,
}

#[derive(Debug, Clone, Default)]
pub struct IconProps {
    // todo!(DrawQuad pixel())
    // pub draw_bg: Option<DrawQuad>,
    pub draw_icon: Option<DrawIcon>,
    pub icon_walk: Option<Walk>,
    pub walk: Option<Walk>,
    pub layout: Option<Layout>,
}

impl DynProps for IconProps {
    fn prop_bind(
        prop: &gen_parser::PropsKey,
        value: &gen_parser::Value,
        is_prop: bool,
        ident: &str,
    ) -> proc_macro2::TokenStream {
        let value = bind_prop_value(value, is_prop, ident);
        match prop.name() {
            // ----------------- draw_icon ---------------
            BRIGHTNESS => quote_prop(vec![DRAW_ICON, BRIGHTNESS], &value),
            CURVE => quote_prop(vec![DRAW_ICON, CURVE], &value),
            LINEARIZE => quote_prop(vec![DRAW_ICON, LINEARIZE], &value),
            SVG_FILE => quote_prop(vec![DRAW_ICON, SVG_FILE], &value),
            SCALE => quote_prop(vec![DRAW_ICON, SCALE], &value),
            DRAW_DEPTH => quote_prop(vec![DRAW_ICON, DRAW_DEPTH], &value),
            COLOR => quote_prop(vec![DRAW_ICON, COLOR], &value),
            // ----------------- icon_walk ---------------
            "icon_height" => quote_prop(vec![ICON_WALK, HEIGHT], &value),
            "icon_width" => quote_prop(vec![ICON_WALK, WIDTH], &value),
            "icon_abs_pos" => quote_prop(vec![ICON_WALK, ABS_POS], &value),
            "icon_margin" => quote_prop(vec![ICON_WALK, MARGIN], &value),
            // ----------------- walk -----------------
            Size::HEIGHT => quote_prop(vec![HEIGHT], &value),
            Size::WIDTH => quote_prop(vec![WIDTH], &value),
            Position::ABS_POS => quote_prop(vec![ABS_POS], &value),
            Size::MARGIN => quote_prop(vec![MARGIN], &value),
            // ------------------- layout -----------------
            Others::SCROLL => quote_prop(vec![SCROLL], &value),
            Size::CLIP_X => quote_prop(vec![CLIP_X], &value),
            Size::CLIP_Y => quote_prop(vec![CLIP_Y], &value),
            Size::PADDING => quote_prop(vec![PADDING], &value),
            Position::ALIGN => quote_prop(vec![ALIGN], &value),
            Position::FLOW => quote_prop(vec![FLOW], &value),
            Position::SPACING => quote_prop(vec![SPACING], &value),
            LINE_SPACING => quote_prop(vec![LINE_SPACING], &value),
            _ => panic!("cannot match prop in BuiltIn Icon"),
        }
    }
}

impl StaticProps for IconProps {
    fn props(props: &std::collections::HashMap<gen_parser::PropsKey, gen_parser::Value>) -> Self
    where
        Self: Sized,
    {
        let mut icon = IconProps::default();
        for (k, v) in props {
            icon.prop(k.name(), v)
        }
        icon
    }

    fn prop(&mut self, prop_name: &str, value: &gen_parser::Value) -> () {
        let _ = match prop_name {
            // ----------------- draw_icon ---------------
            BRIGHTNESS => self.brightness(&value),
            CURVE => self.curve(&value),
            LINEARIZE => self.linearize(&value),
            SVG_FILE => self.svg_file(&value),
            SCALE => self.scale(&value),
            DRAW_DEPTH => self.draw_depth(&value),
            COLOR => self.color(&value),
            // ----------------- icon_walk ---------------
            "icon_height" => self.height(&value, NodeType::Inner),
            "icon_width" => self.width(&value, NodeType::Inner),
            "icon_abs_pos" => self.abs_pos(&value, NodeType::Inner),
            "icon_margin" => self.margin(&value, NodeType::Inner),
            // ----------------- walk -----------------
            Size::HEIGHT => self.height(&value, NodeType::Outer),
            Size::WIDTH => self.width(&value, NodeType::Outer),
            Position::ABS_POS => self.abs_pos(&value, NodeType::Outer),
            Size::MARGIN => self.margin(&value, NodeType::Outer),
            // ----------------- layout ---------------
            Others::SCROLL => self.scroll(&value),
            Size::CLIP_X => self.clip_x(&value),
            Size::CLIP_Y => self.clip_y(&value),
            Size::PADDING => self.padding(&value),
            Position::ALIGN => self.align(&value),
            Position::FLOW => self.flow(&value),
            Position::SPACING => self.spacing(&value),
            LINE_SPACING => self.line_spacing(&value),
            _ => {
                if !prop_ignore(prop_name) {
                    panic!("cannot match prop: {}", prop_name);
                } else {
                    panic!("unslolved prop: {}", prop_name);
                }
            }
        };
    }
}

#[allow(dead_code)]
impl IconProps {
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
    fn brightness(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_icon().brightness(value)
    }
    fn curve(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_icon().curve(value)
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
    fn draw_depth(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_icon().draw_depth(value)
    }
    fn color(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_draw_icon().color(value)
    }
    fn height(&mut self, value: &Value, ty: NodeType) -> Result<(), Errors> {
        match ty {
            NodeType::Inner => self.check_icon_walk().height(value),
            NodeType::Outer => self.check_walk().height(value),
        }
    }
    fn width(&mut self, value: &Value, ty: NodeType) -> Result<(), Errors> {
        match ty {
            NodeType::Inner => self.check_icon_walk().width(value),
            NodeType::Outer => self.check_walk().width(value),
        }
    }
    fn abs_pos(&mut self, value: &Value, ty: NodeType) -> Result<(), Errors> {
        match ty {
            NodeType::Inner => self.check_icon_walk().abs_pos(value),
            NodeType::Outer => self.check_walk().abs_pos(value),
        }
    }
    fn margin(&mut self, value: &Value, ty: NodeType) -> Result<(), Errors> {
        match ty {
            NodeType::Inner => self.check_icon_walk().margin(value),
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
    fn line_spacing(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().line_spacing(value)
    }
    fn align(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_layout().align(value)
    }
}

impl Display for IconProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // todo!(DrawQuard pixel())
        // if let Some(draw_bg) = &self.draw_bg {
        //     let _ = f.write_fmt(format_args!("{}: {{{}}}", DRAW_BG, draw_bg));
        // }
        if let Some(draw_icon) = &self.draw_icon {
            let _ = f.write_fmt(format_args!("{}: {{{}}},", DRAW_ICON, draw_icon));
        }
        if let Some(icon_walk) = &self.icon_walk {
            let _ = f.write_fmt(format_args!("{}: {{{}}},", ICON_WALK, icon_walk));
        }
        if let Some(walk) = &self.walk {
            let _ = f.write_fmt(format_args!("{}", walk));
        }
        if let Some(layout) = &self.layout {
            let _ = f.write_fmt(format_args!("{}", layout));
        }
        write!(f, "")
    }
}

props_to_token!(IconProps);

#[cfg(test)]
mod test_icon {
    use crate::{prop::builtin::draw_icon::DrawIcon, ToToken};

    use super::IconProps;

    #[test]
    fn icon() {
        let mut icon = IconProps::default();
        // draw icon
        let mut draw_icon = DrawIcon::default();
        draw_icon.svg_file = Some(
            "crate://self/resources/icons/Icon_Search.svg"
                .try_into()
                .unwrap(),
        );
        let _ = draw_icon.brightness(&0.5.into());

        icon.draw_icon = Some(draw_icon);

        let tk = icon.to_token_stream().to_string();

        let prop = "draw_icon : { brightness : 0.5 , svg_file : dep (\"crate://self/resources/icons/Icon_Search.svg\") , } ,";

        assert_eq!(tk.as_str(), prop);
    }
}
