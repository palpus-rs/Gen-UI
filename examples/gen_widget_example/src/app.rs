use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 
    import gen_components::components::*;

    App = {{App}}{
        root: <Root>{
            main_window = <Window>{
                show_bg: true,
                width: Fill,
                height: Fill,
                draw_bg: {color: #1C2128},
                body = <View>{
                    height: All,
                    width: All,
                    flow: Down,
                    spacing: 10.0,
                    padding: 10.0,
                    <GLabel>{
                        text: "Hello, world! This is a long message",
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
                    <GButton>{
                        text: "hello"
                    }
                    <GButton>{
                        theme: Primary,
                        // cover theme color
                        background_color: #FF0000,
                        text: "test"
                    }
                    <GButton>{
                        height: 46,
                        width: 160,
                        theme: Success,
                        border_width: 1.4,
                        border_color: #FFF,
                        border_radius: 11.0,
                        text: "Theme Success",
                    }
                    <GButton>{
                        theme: Dark,
                        border_width: 1.2,
                        hover_color: #FF0000,
                        pressed_color: #00FF00,
                        text: "GButton!",
                        font_family: dep("E:/Rust/try/makepad/Gen-UI/examples/gen_widget_example/resources/GoNotoKurrent-Bold.ttf"),
                        font_size: 12.0,
                        font_color: #000,
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    root: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        crate::gen_components::live_design(cx);
        // crate::gen_components::live_design!(cx);
    }
}

impl MatchEvent for App {}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.root.handle_event(cx, event, &mut Scope::empty());
    }
}

app_main!(App);
