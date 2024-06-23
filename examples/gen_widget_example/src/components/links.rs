use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 
    import gen_components::components::*;

    GLinkExample = <ScrollYView>{
        height: 120.0,
        width: Fill,
        spacing: 10.0,
        flow: Down,
        <GLabel>{
            text: "GLink"
        }
        <GLink>{
            text: "Link",
        }
        <GLink>{
            theme: Dark,
            text: "Theme Dark",
        }
        <GLink>{
            theme: Error,
            text: "Define hover color and pressed color",
            hover_color: #FF00FF,
            pressed_color: #00FF00,
        }
        <GLink>{
            theme: Success,
            text: "No underline",
            underline: false,
        }
        <GLink>{
            theme: Warning,
            text: "Custom More",
            font_size: 14.0,
            hover_color: #FF00FF,
            background_color: #00FF00,
            margin: 10.0,
            padding: 10.0,
            color: #FF0000,
            underline_width: 2.0,
            font_family: dep("E:/Rust/try/makepad/Gen-UI/examples/gen_widget_example/resources/GoNotoKurrent-Bold.ttf"),

        }
        
    }
}