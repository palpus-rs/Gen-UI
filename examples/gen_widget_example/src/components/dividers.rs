use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 
    import gen_components::components::*;

    GDividerExample = <ScrollYView>{
        height: 100.0,
        width: Fill,
        flow: Down,
        spacing: 10.0,
        <Label>{
            text: "GDivider",
        }
        <GHLayout>{
            height: Fit,
            width: 300,
            background_color: #FFFFFF,
            spacing: 10.0,
            <GDivider>{
                
                height: 40.0;
                width: 160.0;
                <GCard>{
                    <GLabel>{
                        text: "Hello",
                        color: #0,
                        margin: 10.0,
                    }
                }
            }
            // <GHLayout>{
            //     height: Fit,
            //     width: 300.0,
            //     spacing: 0.0,
            //     align: {
            //         x: 0.5,
            //         y: 0.5,
            //     },
            //     <GCard>{
            //         height: 4.0,
            //         width: 60.0,
            //         border_radius: 0.0,
            //     }
            //     <GLabel>{
            //         text: "Hello",
            //     }
            //     <GCard>{
            //         height: 4.0,
            //         width: 60.0,
            //         border_radius: 0.0,
            //     }
            // }
        }
    }
}