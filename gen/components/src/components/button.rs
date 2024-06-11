use makepad_widgets::*;
use crate::themes::{ThemeColorValue, ThemeDark, ThemeError, ThemePrimary, ThemeSuccess, ThemeWarning, Themes};

live_design! {
    import makepad_draw::shader::std::*;
    THEME_BEVELING = 0.75
    THEME_COLOR_CONTRAST = 1.0
    THEME_COLOR_W = #FFFFFFFF
    THEME_COLOR_W_H = #FFFFFF00
    THEME_COLOR_B = #000000FF
    THEME_COLOR_B_H = #00000000
    THEME_COLOR_U_1 = (mix(THEME_COLOR_W, THEME_COLOR_W_H, pow(0.95, THEME_COLOR_CONTRAST)))
    THEME_COLOR_U_2 = (mix(THEME_COLOR_W, THEME_COLOR_W_H, pow(0.9, THEME_COLOR_CONTRAST)))
    THEME_COLOR_U_3 = (mix(THEME_COLOR_W, THEME_COLOR_W_H, pow(0.75, THEME_COLOR_CONTRAST)))
    THEME_COLOR_D_1 = (mix(THEME_COLOR_B, THEME_COLOR_B_H, pow(0.85, THEME_COLOR_CONTRAST)))
    THEME_COLOR_D_3 = (mix(THEME_COLOR_B, THEME_COLOR_B_H, pow(0.6, THEME_COLOR_CONTRAST)))
    THEME_COLOR_CTRL_DEFAULT = (THEME_COLOR_U_1)
    THEME_COLOR_BEVEL_SHADOW = (THEME_COLOR_D_3)
    THEME_COLOR_CTRL_PRESSED = (THEME_COLOR_D_1)
    THEME_COLOR_CTRL_HOVER = (THEME_COLOR_U_2)
    HEME_COLOR_BEVEL_LIGHT = (THEME_COLOR_U_3)
    
    GButtonBase = {{GButton}}{
        draw_bg: {
            instance border_width: 0.0
            instance border_color: #0000
            instance inset: vec4(0.0, 0.0, 0.0, 0.0)
            instance radius: 2.5
            
            fn get_color(self) -> vec4 {
                return self.color
            }
            
            fn get_border_color(self) -> vec4 {
                return self.border_color
            }
            
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size)
                sdf.box(
                    self.inset.x + self.border_width,
                    self.inset.y + self.border_width,
                    self.rect_size.x - (self.inset.x + self.inset.z + self.border_width * 2.0),
                    self.rect_size.y - (self.inset.y + self.inset.w + self.border_width * 2.0),
                    max(1.0, self.radius)
                )
                sdf.fill_keep(self.get_color())
                if self.border_width > 0.0 {
                    sdf.stroke(self.get_border_color(), self.border_width)
                }
                return sdf.result;
            }
        }
    }
}

#[derive(Widget, Live, LiveHook)]
pub struct GButton{
    #[live]
    theme: Themes,
    #[live]
    pub text: RcStringMut,
    #[live]
    background_color: Option<Vec4>,
    // deref -----------------
    #[deref]
    view: View
}


impl Widget for GButton {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // dbg!(&self.background_color);
        let bg_color = if let Some(bg_color) = self.background_color  {
            bg_color
        }else{
            match self.theme {
                Themes::Dark => ThemeDark::default().get(),
                Themes::Primary => ThemePrimary::default().get(),
                Themes::Error => ThemeError::default().get(),
                Themes::Warning => ThemeWarning::default().get(),
                Themes::Success => ThemeSuccess::default().get(),
            }
        };

        self.apply_over(cx, live!{
            show_bg: true,
            draw_bg: {
                color: (bg_color)
            }
        }); 

        let _ = self.view.draw_walk(cx, scope, walk);
        DrawStep::done()
    }
}
