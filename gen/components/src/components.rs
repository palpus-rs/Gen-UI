use makepad_widgets::*;

pub mod label;
pub mod button;
pub mod card;
pub mod link;
pub mod icon;
pub mod image;
pub mod radio;
pub mod checkbox;
pub mod input;

live_design!{
    // imports -----------------------------------------------------
    import crate::components::label::GLabelBase;
    import crate::components::button::GButtonBase;
    import crate::components::card::CardBase;
    import crate::components::link::GLinkBase;
    import crate::components::icon::GIconBase;
    import makepad_widgets::base::*;
    import makepad_draw::shader::std::*;
    // globals -----------------------------------------------------
    // -------- colors ---------------------------------------------
    // each theme color has [25, 50, 100, 200, 300, 400, 500, 600, 700, 800, 900]
    // the default color is 500
    COLOR_WHITE = #FFFFFF;
    COLOR_BLACK = #000000;
    // -------- dark-opacity ---------------------------------------
    DARK_OPACITY_25 = #66666640;
    DARK_OPACITY_50 = #66666680;
    DARK_OPACITY_75 = #666666BF;
    // -------- color-dark -----------------------------------------
    COLOR_DARK_25 = #FCFCFD;
    COLOR_DARK_50 = #F9FAFB;
    COLOR_DARK_100 = #F2F4F7;
    COLOR_DARK_200 = #EAECF0;
    COLOR_DARK_300 = #D0D5DD;
    COLOR_DARK_400 = #95A2D3;
    COLOR_DARK_500 = #667085;
    COLOR_DARK_600 = #475467;
    COLOR_DARK_700 = #344054;
    COLOR_DARK_800 = #1D2939;
    COLOR_DARK_900 = #101828;
    // -------- color-primary --------------------------------------
    COLOR_PRIMARY_25 = #F5FEFF;
    COLOR_PRIMARY_50 = #ECFDFF;
    COLOR_PRIMARY_100 = #CFF9FE;
    COLOR_PRIMARY_200 = #A5F0FC;
    COLOR_PRIMARY_300 = #67E3F9;
    COLOR_PRIMARY_400 = #22CCEE;
    COLOR_PRIMARY_500 = #06AED4;
    COLOR_PRIMARY_600 = #088AB2;
    COLOR_PRIMARY_700 = #0E6F90;
    COLOR_PRIMARY_800 = #155B75;
    COLOR_PRIMARY_900 = #164C63;
    // -------- color-error ------------------------------------
    COLOR_ERROR_25 = #FFFBFA;
    COLOR_ERROR_50 = #FEF3F2;
    COLOR_ERROR_100 = #FEE4E2;
    COLOR_ERROR_200 = #FECDCA;
    COLOR_ERROR_300 = #FDA29B;
    COLOR_ERROR_400 = #F97066;
    COLOR_ERROR_500 = #F04438;
    COLOR_ERROR_600 = #D92D2D;
    COLOR_ERROR_700 = #B42318;
    COLOR_ERROR_800 = #912018;
    COLOR_ERROR_900 = #7A271A;
    // -------- color-warning ------------------------------------
    COLOR_WARNING_25 = #FFFCF5;
    COLOR_WARNING_50 = #FFFAEB;
    COLOR_WARNING_100 = #FEF0C7;
    COLOR_WARNING_200 = #FEDF89;
    COLOR_WARNING_300 = #FEC84B;
    COLOR_WARNING_400 = #FDB022;
    COLOR_WARNING_500 = #F79009;
    COLOR_WARNING_600 = #DC6803;
    COLOR_WARNING_700 = #B54708;
    COLOR_WARNING_800 = #93370D;
    COLOR_WARNING_900 = #7A2E0E;
    // -------- color-success ------------------------------------
    COLOR_SUCCESS_25 = #F6FEF9;
    COLOR_SUCCESS_50 = #ECFDF3;
    COLOR_SUCCESS_100 = #D1FADF;
    COLOR_SUCCESS_200 = #A6F4C5;
    COLOR_SUCCESS_300 = #6CE9A6;
    COLOR_SUCCESS_400 = #32D583;
    COLOR_SUCCESS_500 = #12B76A;
    COLOR_SUCCESS_600 = #039855;
    COLOR_SUCCESS_700 = #027A48;
    COLOR_SUCCESS_800 = #05603A;
    COLOR_SUCCESS_900 = #054F31;
    // -------- font-family ------------------------------------
    FONT_FAMILY = dep("crate://self/resources/font/GoNotoKurrent-Regular.ttf");
    FONT_FAMILY_BOLD = dep("crate://self/resources/font/GoNotoKurrent-Bold.ttf");
    FONT_SIZE = 10.0;
    // padding -----------------------------------------------------
    
    GLOBAL_PADDING = {top: 10.0, left: 16.0, bottom: 10.0, right: 16.0};
    // align -------------------------------------------------------
    ALIGN_CENTER_WALK = {x: 0.5, y: 0.5};
    // components --------------------------------------------------

    // ## GLabel
    // A label component use white color
    GLabel = <GLabelBase>{
        width: Fit, 
        height: Fit,
        color: (COLOR_WHITE),
        font_family: (FONT_FAMILY),
        brightness: 1.0,
        top_drop: 1.0,
        line_spacing: 1.5,
        font_size: (FONT_SIZE),
    }
    GLink = <GLinkBase>{
        height: Fit,
        width: Fit,
        padding: 0,
        font_size: (FONT_SIZE),
        align: <ALIGN_CENTER_WALK>{},
    }
    // ## GButton
    // A button component which only has a text
    // if you wanna add some other components like icon, you can create a new component use CardBase
    // CardBase can help you create a wonderful button quickly and easily
    GButton = <GButtonBase>{
        theme: Primary,
        text: " ",
        padding: <GLOBAL_PADDING>{}
        font_size: (FONT_SIZE),
        align: <ALIGN_CENTER_WALK>{},
    }
    // ## GCard
    // A card component that you can use to wrap other components
    // card has default styles for border, background color, ...
    GCard = <CardBase>{
        width: 300.0,
        height: 200.0,
    }
    // ## GHLayout
    // A horizontal layout component use CardBase
    // layout don't have border, background color, border-radius, ... (but you can add if you want)
    GHLayout = <CardBase>{
        height: Fill,
        width: Fill,
        flow: Right,
        padding: 0,
        border_radius: 0,
        border_width: 0,
        transparent: true,
        spacing: 0,
        margin: 0,
    }
    // ## GVLayout
    // A vertical layout component use CardBase
    GVLayout = <CardBase>{
        height: Fill,
        width: Fill,
        flow: Down,
        padding: 0,
        border_radius: 0,
        border_width: 0,
        transparent: true,
        spacing: 0,
        margin: 0,
    }
    // ## GScrollBar
    // A scroll bar component use ScrollBarBase, it is a single scroll bar
    GScrollBar = <ScrollBarBase> {
        bar_size: 10.0,
        bar_side_margin: 3.0
        min_handle_size: 20.0
        draw_bar: {
            instance pressed: 0.0
            instance hover: 0.0
            
            instance color: (DARK_OPACITY_50)
            instance color_hover: (DARK_OPACITY_25)
            instance color_pressed: (DARK_OPACITY_75)
            
            uniform bar_width: 6.0
            uniform border_radius: 1.5

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                if self.is_vertical > 0.5 {
                    sdf.box(
                        1.,
                        self.rect_size.y * self.norm_scroll,
                        self.bar_width,
                        self.rect_size.y * self.norm_handle,
                        self.border_radius
                    );
                }
                else {
                    sdf.box(
                        self.rect_size.x * self.norm_scroll,
                        1.,
                        self.rect_size.x * self.norm_handle,
                        self.bar_width,
                        self.border_radius
                    );
                }
                return sdf.fill(mix(
                    self.color, 
                    mix(
                        self.color_hover,
                        self.color_pressed,
                        self.pressed
                    ),
                    self.hover
                ));
            }
        }
        animator: {
            hover = {
                default: off
                off = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_bar: {pressed: 0.0, hover: 0.0}
                    }
                }

                on = {
                    cursor: Default,
                    from: {
                        all: Forward {duration: 0.1}
                        pressed: Forward {duration: 0.01}
                    }
                    apply: {
                        draw_bar: {
                            pressed: 0.0,
                            hover: [{time: 0.0, value: 1.0}],
                        }
                    }
                }

                pressed = {
                    cursor: Default,
                    from: {all: Snap}
                    apply: {
                        draw_bar: {
                            pressed: 1.0,
                            hover: 1.0,
                        }
                    }
                }
            }
        }
    }
    // ## GScrollBars
    // A scroll bars component use ScrollBarsBase, it has two scroll bars (x, y)
    // It often use in a Card
    GScrollBars = <ScrollBarsBase> {
        show_scroll_x: true,
        show_scroll_y: true,
        scroll_bar_x: <GScrollBar> {}
        scroll_bar_y: <GScrollBar> {}
    }
    GIcon = <GIconBase>{
        width: Fit,
        height: Fit,
        // color: (COLOR_DARK_500),
        // font_size: 16.0,
        // align: <ALIGN_CENTER_WALK>{},
    }
}
