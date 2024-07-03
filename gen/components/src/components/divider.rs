use makepad_widgets::*;

use crate::{
    shader::draw_card::DrawCard,
    themes::{get_color, Themes},
};

use super::card::{Card, CardRef};

live_design! {
    import makepad_draw::shader::std::*;

    GDividerBase = {{GDivider}}{

    }
}

#[derive(Live, Widget)]
pub struct GDivider {
    #[live]
    pub theme: Themes,
    #[live]
    pub background_color: Option<Vec4>,
    #[live]
    pub hover_color: Option<Vec4>,
    #[live]
    pub border_color: Option<Vec4>,
    #[live(0.0)]
    pub border_width: f32,
    #[live(0.0)]
    pub border_radius: f32,
    #[live(2.0)]
    pub stroke_width: f64,
    // center view ----------
    #[rust]
    center: Option<CardRef>,
    // left divider ---------
    #[live]
    left: DrawCard,
    // right divider --------
    #[live]
    right: DrawCard,
    // deref ---------------
    #[redraw]
    #[live]
    draw_divider: DrawCard,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
}

impl Widget for GDivider {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let  mut stroke_walk = walk.clone();
        stroke_walk.width = match stroke_walk.width {
            Size::Fixed(len) => Size::Fixed(len / 2.0),
            other => other,
        };
        stroke_walk.height = Size::Fixed(self.stroke_width);
        // dbg!(&stroke_walk);
        self.draw_divider.begin(cx, walk, self.layout);
        self.left.begin(cx, stroke_walk.clone(),  self.layout);
        self.left.end(cx);
        // let _ = self.center.draw_walk(cx, scope, walk);
        self.right.begin(cx, stroke_walk,  self.layout);
        self.right.end(cx);
        self.draw_divider.end(cx);

        DrawStep::done()
    }
}

impl LiveHook for GDivider {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        // ----------------- background color -------------------------------------------
        let bg_color = get_color(self.theme, self.background_color, 25);
        let bg_color2 = get_color(self.theme, self.background_color, 500);
        // ------------------ hover color -----------------------------------------------
        let hover_color = get_color(self.theme, self.hover_color, 400);
        // ------------------ border color ----------------------------------------------
        let border_color = get_color(self.theme, self.border_color, 800);
        self.draw_divider.apply_over(
            cx,
            live! {
                background_color: (bg_color),
                border_color: (border_color),
                border_width: (self.border_width),
                border_radius: (self.border_radius),
                hover_color: (hover_color),
                transparent: 1.0,
            },
        );
        self.right.apply_over(
            cx,
            live! {
                background_color: (bg_color2),
                border_color: (border_color),
                border_width: (self.border_width),
                border_radius: (self.border_radius),
                hover_color: (hover_color),
                
            },
        );
        self.left.apply_over(
            cx,
            live! {
                background_color: (bg_color2),
                border_color: (border_color),
                border_width: (self.border_width),
                border_radius: (self.border_radius),
                hover_color: (hover_color),
            },
        );
        self.draw_divider.redraw(cx);
    }
}
