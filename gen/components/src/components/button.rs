use crate::{
    shader::bg_button::DrawGButton,
    themes::{
        ThemeColorValue, ThemeDark, ThemeError, ThemePrimary, ThemeSuccess, ThemeWarning, Themes,
    },
};
use makepad_widgets::*;

live_design! {
    import makepad_draw::shader::std::*;
    THEME_COLOR_CONTRAST = 1.0
    THEME_COLOR_W = #FFFFFFFF
    THEME_COLOR_W_H = #FFFFFF00
    THEME_COLOR_B = #000000FF
    THEME_COLOR_B_H = #00000000
    THEME_FONT_SIZE_BASE = 7.5
    THEME_FONT_SIZE_CONTRAST = 2.5
    THEME_COLOR_U_5 = (mix(THEME_COLOR_W, THEME_COLOR_W_H, pow(0.35, THEME_COLOR_CONTRAST)))
    THEME_COLOR_TEXT_DEFAULT = (THEME_COLOR_U_5)
    THEME_FONT_REGULAR = { font: { path: dep("crate://self/resources/font/GoNotoKurrent-Regular.ttf") } }
    THEME_FONT_SIZE_P = (THEME_FONT_SIZE_BASE + 1 * THEME_FONT_SIZE_CONTRAST)
    THEME_SPACE_FACTOR = 8.5
    THEME_SPACE_2 = (1.0 * (THEME_SPACE_FACTOR))
    THEME_MSPACE_2 = {top: (THEME_SPACE_2), right: (THEME_SPACE_2), bottom: (THEME_SPACE_2), left: (THEME_SPACE_2)}
    GButtonBase = {{GButton}}{
        // draw_button: {
        //     instance border_width: 1.0
        //     instance inset: vec4(0.0, 0.0, 0.0, 0.0)
        //     instance radius: 2.5

        //     fn get_color(self) -> vec4 {
        //         return self.background_color
        //     }

        //     fn get_border_color(self) -> vec4 {
        //         return self.border_color
        //     }

        //     fn pixel(self) -> vec4 {
        //         let sdf = Sdf2d::viewport(self.pos * self.rect_size)
        //         sdf.box(
        //             self.inset.x + self.border_width,
        //             self.inset.y + self.border_width,
        //             self.rect_size.x - (self.inset.x + self.inset.z + self.border_width * 2.0),
        //             self.rect_size.y - (self.inset.y + self.inset.w + self.border_width * 2.0),
        //             max(1.0, self.radius)
        //         )
        //         sdf.fill_keep(self.get_color())
        //         // if self.border_width > 0.0 {
        //         //     sdf.stroke(self.get_border_color(), self.border_width)
        //         // }

        //         sdf.stroke(self.get_border_color(), self.border_width)
        //         return sdf.result;
        //     }
        // }
        
        draw_text: {
            instance hover: 0.0,
            instance pressed: 0.0,

            uniform color: #FFFFFF,
            uniform color_hover: (THEME_COLOR_TEXT_DEFAULT)
            uniform color_pressed: (THEME_COLOR_TEXT_DEFAULT)

            text_style: <THEME_FONT_REGULAR> {
                font_size: (THEME_FONT_SIZE_P)
            }

            fn get_color(self) -> vec4 {
                return mix(
                    self.color,
                    mix(self.color_hover, self.color_pressed, self.pressed),
                    self.hover
                )
            }
        }
        padding: <THEME_MSPACE_2>{}
        height: Fit,
        width: Fit,
        text_walk: {
            height: Fit,
            width: Fit,
        }
        align: {x: 0.5, y: 0.5},
    }
}

#[derive(Widget, Live, LiveHook)]
pub struct GButton {
    #[live]
    pub theme: Themes,
    #[live]
    pub background_color: Option<Vec4>,
    #[live]
    pub border_color: Option<Vec4>,
    #[live(0.0)]
    pub border_width: f32,
    #[live(4.0)]
    pub border_radius: f32,
    // text-----------------
    #[live]
    pub text: RcStringMut,
    #[live]
    pub font_size: f64,
    #[live]
    pub font_color: Vec4,
    #[live]
    draw_text: DrawText,
    #[live]
    text_walk: Walk,
    // animator -----------------
    #[animator]
    animator: Animator,
    // deref -----------------
    #[redraw]
    #[live]
    draw_button: DrawGButton,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
}

#[derive(Clone, Debug, DefaultNone)]
pub enum GButtonEvent {
    Clicked(KeyModifiers),
    Released(KeyModifiers),
    None,
}

impl Widget for GButton {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        match event.hits(cx, self.draw_button.area()) {
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Hand);
                self.animator_play(cx, id!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, id!(hover.off));
            }
            Hit::FingerUp(f_up) => {
                if f_up.is_over {
                    cx.widget_action(uid, &scope.path, GButtonEvent::Clicked(f_up.modifiers));
                    cx.widget_action(uid, &scope.path, GButtonEvent::Released(f_up.modifiers));
                    if f_up.device.has_hovers() {
                        self.animator_play(cx, id!(hover.on));
                    } else {
                        self.animator_play(cx, id!(hover.off));
                    }
                } else {
                    cx.widget_action(uid, &scope.path, GButtonEvent::Released(f_up.modifiers));
                    self.animator_play(cx, id!(hover.off));
                }
            }
            _ => (),
        }
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // ----------------- background color ------------------------------------------
        let bg_color = if let Some(bg_color) = self.background_color {
            bg_color
        } else {
            match self.theme {
                Themes::Dark => ThemeDark::default().get(),
                Themes::Primary => ThemePrimary::default().get(),
                Themes::Error => ThemeError::default().get(),
                Themes::Warning => ThemeWarning::default().get(),
                Themes::Success => ThemeSuccess::default().get(),
            }
        };
        // ------------------ border color ----------------------------------------------
        let border_color = if let Some(border_color) = self.border_color {
            border_color
        } else {
            match self.theme {
                Themes::Dark => ThemeDark::v(800),
                Themes::Primary => ThemePrimary::v(800),
                Themes::Error => ThemeError::v(800),
                Themes::Warning => ThemeWarning::v(800),
                Themes::Success => ThemeSuccess::v(800),
            }
        };
        // ------------------ calc padding ----------------------------------------------

        self.apply_over(
            cx,
            live! {
                // show_bg: true,
                draw_button: {
                    background_color: (bg_color),
                    border_color: (border_color),
                    border_width: (self.border_width),
                    border_radius: (self.border_radius),
                }
            },
        );
        // self.redraw(cx);
        
        let _ = self.draw_button.begin(cx, walk, self.layout);
        
        let _ = self
            .draw_text
            .draw_walk(cx, self.text_walk, Align::default(), self.text.as_ref());
        
        self.draw_button.end(cx);
        DrawStep::done()
    }

    fn text(&self) -> String {
        self.text.as_ref().to_string()
    }
    fn set_text(&mut self, v: &str) {
        self.text.as_mut_empty().push_str(v);
    }
    fn set_text_and_redraw(&mut self, cx: &mut Cx, v: &str) {
        self.text.as_mut_empty().push_str(v);
        self.redraw(cx)
    }
}

impl GButton {
    pub fn clicked(&self, actions: &Actions) -> bool {
        if let GButtonEvent::Clicked(_) = actions.find_widget_action(self.widget_uid()).cast() {
            true
        } else {
            false
        }
    }
    pub fn released(&self, actions: &Actions) -> bool {
        if let GButtonEvent::Released(_) = actions.find_widget_action(self.widget_uid()).cast() {
            true
        } else {
            false
        }
    }
}

impl GButtonRef {
    pub fn clicked(&self, actions: &Actions) -> bool {
        if let Some(btn_ref) = self.borrow() {
            return btn_ref.clicked(actions);
        }
        false
    }
    pub fn released(&self, actions: &Actions) -> bool {
        if let Some(btn_ref) = self.borrow() {
            return btn_ref.released(actions);
        }
        false
    }
}

impl GButtonSet {
    pub fn clicked(&self, actions: &Actions) -> bool {
        self.iter().any(|btn_ref| btn_ref.clicked(actions))
    }
    // pub fn pressed(&self, actions: &Actions) -> bool {
    //     self.iter().any(|v| v.pressed(actions))
    // }
    pub fn released(&self, actions: &Actions) -> bool {
        self.iter().any(|btn_ref| btn_ref.released(actions))
    }
}
