use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 
    import gen_components::components::*;
    import makepad_draw::shader::std::*;
    GImageExample = <ScrollYView>{
        height: 60.0,
        width: Fill,
        spacing: 20.0,
        <GLabel>{
            text: "GImage",
        }
        <GImage>{
            height: 32.0,
            width: 36.0,
            src: dep("crate://self/resources/rust.png"),
            rotation:30.0,
        }
        <GImage>{
            rotation: 1.0,
            opacity: 0.6,
            src: dep("crate://self/resources/robius.png"),
        }
        <GImage>{
            scale: 0.6,
            src: dep("crate://self/resources/robius.png"),
        }
        <GImage>{
            scale: 2.0,
            src: dep("crate://self/resources/robius.png"),
        }
       
        
    }
}