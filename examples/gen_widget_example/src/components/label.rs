use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 
    import gen_components::components::*;

    GLabelExample = <ScrollYView>{
        height: 100.0,
        width: Fill,
        spacing: 10.0,
        <Label>{
            text: "GLabel"
        }
        <GLabel>{
            text: "Hello, world! This is a long message, but I use wrap Word to wrap it!",
            height: 48.0,
            width: 120.0,
            wrap: Word,
            brightness: 1.5,
            margin: {left: 12.0},
        }
        <GLabel>{
            text: "bold, test bold!!",
            font_size: 12.0,
            padding: 16.0,
            color: #FF0000,
            // font_family: dep("crate://self/resources/GoNotoKurrent-Bold.ttf"),
            font_family: dep("E:/Rust/try/makepad/Gen-UI/examples/gen_widget_example/resources/GoNotoKurrent-Bold.ttf"),
        }
    }
}