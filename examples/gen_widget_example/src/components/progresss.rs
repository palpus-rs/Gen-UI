use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 
    import gen_components::components::*;

    GProgressExample = <ScrollYView>{
        height: 150.0,
        width: Fill,
        spacing: 10.0,
        flow: Down,
        <GLabel>{
            text: "GProgress"
        }
        <GProgress>{
            value: 0.5,
        }
        <GProgress>{
            theme: Dark,
            height: 20.0,
            border_radius: 2.0,
            value: 0.36,
            read_only: false,
        }
    }
}