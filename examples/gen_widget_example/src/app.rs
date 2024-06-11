use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 
    import gen_components::components::*;

    // FONT = dep("crate://self/resources/IBMPlexSans-BoldItalic.ttf")

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
                    <GLabel>{
                        text: "Hello, world!",
                        height: 48.0,
                        width: 120.0,
                        // wrap: Word,
                        brightness: 1.5
                    }
                    <GLabel>{
                        text: "bold, test bold!!",
                        
                        // font_family: dep("crate://self/resources/GoNotoKurrent-Bold.ttf"),
                    }
                    // <GLabel>{
                    //     text: "Hello, world!",
                    // }
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
