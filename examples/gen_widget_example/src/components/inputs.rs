use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 
    import gen_components::components::*;

    GInputExample = <ScrollYView>{
        height: 160.0,
        width: Fill,
        spacing: 10.0,
        flow:Down,
        <Label>{
            text: "GInput"
        }
        <GInput>{
            theme: Dark,   
        }
        <GInput>{
            theme: Dark,
            border_width: 1.0,
            // border_radius: 4.0,
            value: "Hello",
            placeholder: "please",
            
        }
        <GInput>{
            border_radius: 7.0,
            border_width: 1.0,
            input_type: Pwd,
        }
    }
}