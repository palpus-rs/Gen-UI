use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 
    import gen_components::components::*;

    GIconExample = <ScrollYView>{
        height: 200.0,
        width: Fill,
        spacing: 10.0,
        flow: Down,
        <GLabel>{
            text: "GIcon",
        }
        <GIcon>{
            cursor: Help,
            src: dep("crate://self/resources/lightning.svg"),
        }
        <GIcon>{
            theme: Dark,
            src: dep("crate://self/resources/config.svg"),
        }
        <GIcon>{
            theme: Error,
            src: dep("crate://self/resources/lightning.svg"),
        }
        <GIcon>{
            theme: Warning,
            src: dep("crate://self/resources/lightning.svg"),
        }
        <GIcon>{
            height: 60,
            width: 160,
            cursor: Help,
            color: #fff,
            src: dep("crate://self/resources/logo_makepad.svg"),
        }
        
    }
}