use makepad_widgets::*;

pub mod label;

live_design!{
    // imports -----------------------------------------------------
    import crate::components::label::GLabelBase;
    import makepad_draw::shader::std::*;
    // globals -----------------------------------------------------
    // -------- colors ---------------------------------------------
    // each theme color has [25, 50, 100, 200, 300, 400, 500, 600, 700, 800, 900]
    // the default color is 500
    COLOR_WHITE = #FFFFFF;
    COLOR_BLACK = #000000;
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
    COLOR_PRIMARY_25 = #FCFAFF;
    COLOR_PRIMARY_50 = #F9F5FF;
    COLOR_PRIMARY_100 = #F4EBFF;
    COLOR_PRIMARY_200 = #E9D7FE;
    COLOR_PRIMARY_300 = #D6BBFB;
    COLOR_PRIMARY_400 = #B692F6;
    COLOR_PRIMARY_500 = #9E77ED;
    COLOR_PRIMARY_600 = #7F56D9;
    COLOR_PRIMARY_700 = #6941C6;
    COLOR_PRIMARY_800 = #53389F;
    COLOR_PRIMARY_900 = #42307D;
    // -------- font-family ------------------------------------
    FONT_FAMILY = dep("crate://self/resources/font/GoNotoKurrent-Regular.ttf")
    FONT_FAMILY_BOLD = dep("crate://self/resources/font/GoNotoKurrent-Bold.ttf")
    // -------- font-styles --------------------------------------
    FONT_STYLES = {
        font: { path: (FONT_FAMILY) }
        font_size: 12.0
        brightness: 1.1
        top_drop: 1.3
        line_spacing: 1.5
    }
    // components --------------------------------------------------

    GLabel = <GLabelBase>{
        // draw_text: {
        //     text_style:  <FONT_STYLES>{},
        //     color: (COLOR_DARK_500),
        // },
        width: Fit, 
        height: Fit,
        color: (COLOR_WHITE),
        font_family: (FONT_FAMILY),
        // font_family: (FONT_FAMILY),
        brightness: 1.0,
        top_drop: 1.3,
        line_spacing: 1.5,
        font_size: 10.0,
    }
}
