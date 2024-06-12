use crate::{
    shader::bg_button::DrawGButton,
    themes::{
        get_color, ThemeColorValue, ThemeDark, ThemeError, ThemePrimary, ThemeSuccess,
        ThemeWarning, Themes,
    },
};
use makepad_widgets::*;

use super::label::get_font_family;

live_design! {
    import makepad_draw::shader::std::*;
    GLOBAL_DURATION = 0.25

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
        height: Fit,
        width: Fit,
        text_walk: {
            height: Fit,
            width: Fit,
        }

        draw_text: {
            instance hover: 0.0,
            instance pressed: 0.0,

            // uniform color: #FFFFFF,
            // uniform color_hover: (THEME_COLOR_TEXT_DEFAULT)
            // uniform color_pressed: (THEME_COLOR_TEXT_DEFAULT)

            // text_style: <THEME_FONT_REGULAR> {
            //     font_size: (THEME_FONT_SIZE_P)
            // }

            fn get_color(self) -> vec4 {
                let hover_color = self.color - vec4(0.0, 0.0, 0.0, 0.1);
                let pressed_color = self.color - vec4(0.0, 0.0, 0.0, 0.2);
                // return mix(
                //     self.color,
                //     mix(self.color_hover, self.color_pressed, self.pressed),
                //     self.hover
                // )
                return mix(
                    mix(
                        self.color,
                        hover_color,
                        self.hover
                    ),
                    pressed_color,
                    self.pressed
                )
            }
        }

        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_button: {pressed: 0.0, hover: 0.0}
                        // draw_icon: {pressed: 0.0, hover: 0.0}
                        draw_text: {pressed: 0.0, hover: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: (GLOBAL_DURATION)}
                        pressed: Forward {duration: (GLOBAL_DURATION)}
                    }
                    apply: {
                        draw_button: {pressed: 0.0, hover: [{time: 0.0, value: 1.0}],}
                        // draw_icon: {pressed: 0.0, hover: [{time: 0.0, value: 1.0}],}
                        draw_text: {pressed: 0.0, hover: [{time: 0.0, value: 1.0}],}
                    }
                }

                pressed = {
                    from: {all: Forward {duration: (GLOBAL_DURATION)}}
                    apply: {
                        draw_button: {pressed: [{time: 0.0, value: 1.0}], hover: 1.0,}
                        // draw_icon: {pressed: [{time: 0.0, value: 1.0}], hover: 1.0,}
                        draw_text: {pressed: [{time: 0.0, value: 1.0}], hover: 1.0,}
                    }
                }
            }
        }
    }
}

#[derive(Widget, Live, LiveHook)]
pub struct GButton {
    #[live]
    pub theme: Themes,
    #[live]
    pub background_color: Option<Vec4>,
    #[live]
    pub hover_color: Option<Vec4>,
    #[live]
    pub pressed_color: Option<Vec4>,
    #[live]
    pub border_color: Option<Vec4>,
    #[live(0.0)]
    pub border_width: f32,
    #[live(4.0)]
    pub border_radius: f32,
    // text-----------------
    #[live]
    pub text: RcStringMut,
    #[live(10.0)]
    pub font_size: f64,
    #[live]
    pub font_color: Option<Vec4>,
    #[live]
    pub font_family: LiveDependency,
    #[live]
    draw_text: DrawText,
    #[live]
    text_walk: Walk,
    #[live(true)]
    grab_key_focus: bool,
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
    Pressed(KeyModifiers),
    None,
}

impl Widget for GButton {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        if self.animator_handle_event(cx, event).must_redraw() {
            self.draw_button.redraw(cx);
        }
        match event.hits(cx, self.draw_button.area()) {
            Hit::FingerDown(f_down) => {
                if self.grab_key_focus {
                    cx.set_key_focus(self.draw_button.area());
                }
                cx.widget_action(uid, &scope.path, GButtonEvent::Pressed(f_down.modifiers));
                self.animator_play(cx, id!(hover.pressed));
            }
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
        // ----------------- background color -------------------------------------------
        let bg_color = get_color(self.theme, self.background_color, 500);
        // ------------------ hover color -----------------------------------------------
        let hover_color = get_color(self.theme, self.hover_color, 400);
        // ------------------ pressed color ---------------------------------------------
        let pressed_color = get_color(self.theme, self.pressed_color, 600);
        // ------------------ border color ----------------------------------------------
        let border_color = get_color(self.theme, self.border_color, 800);
        // ------------------ font ------------------------------------------------------
        let font = get_font_family(&self.font_family, cx);
        let font_color = get_color(self.theme, self.font_color, 100);
        // apply over props to draw_button ----------------------------------------------
        self.apply_over(
            cx,
            live! {
                // show_bg: true,
                draw_button: {
                    background_color: (bg_color),
                    border_color: (border_color),
                    border_width: (self.border_width),
                    border_radius: (self.border_radius),
                    pressed_color: (pressed_color),
                    hover_color: (hover_color),
                },
                draw_text: {
                    color: (font_color),
                    text_style: {
                        font_size: (self.font_size),
                    },
                }
            },
        );
        self.draw_text.text_style.font = font;
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
    pub fn pressed(&self, actions: &Actions) -> bool {
        if let GButtonEvent::Pressed(_) = actions.find_widget_action(self.widget_uid()).cast() {
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
    pub fn pressed(&self, actions: &Actions) -> bool {
        if let Some(btn_ref) = self.borrow() {
            return btn_ref.pressed(actions);
        }
        false
    }
}

impl GButtonSet {
    pub fn clicked(&self, actions: &Actions) -> bool {
        self.iter().any(|btn_ref| btn_ref.clicked(actions))
    }
    pub fn pressed(&self, actions: &Actions) -> bool {
        self.iter().any(|btn_ref| btn_ref.pressed(actions))
    }
    pub fn released(&self, actions: &Actions) -> bool {
        self.iter().any(|btn_ref| btn_ref.released(actions))
    }
}
