use crate::makepad_platform::*;

live_design!{
    // Imports ------------------------------------------------------------------------------------
    import crate::widgets::root::RootBase;
    import crate::widgets::view::ViewBase;
    import crate::widgets::label::LabelBase;
    import crate::widgets::designer::DesignerBase;
    import makepad_draw::shader::std::*;
    // Globals ------------------------------------------------------------------------------------
    GEN_FONT_FAMILY = { font: { path: dep("crate://self/resources/GoNotoKurrent-Regular.ttf") } }
    // Components ---------------------------------------------------------------------------------
    GView = <ViewBase>{}
    GLabel = <LabelBase>{
        width: Fit, 
        height: Fit,
        draw_text: {
            color: #FF00FF,
            text_style: <GEN_FONT_FAMILY> {},
            wrap: Word
        }
    }
    Designer = <DesignerBase>{
        <Window> {
            window: { kind_id: 2 }
            body = <View> {
                // designer_outline = <DesignerOutline> {
                //     flow: Down,
                //     <DockToolbar> {
                //         content = {
                //             align: { x: 0., y: 0.0 }
                //             spacing: (THEME_SPACE_3)
                //             <Pbold> {
                //                 width: Fit,
                //                 margin: {left: (THEME_SPACE_1) },
                //                 text: "Filter"
                //             }
                //             <View> {
                //                 width: Fit
                //                 flow: Right,
                //                 spacing: (THEME_SPACE_2)
                //                 <CheckBoxCustom> {
                //                     margin: {left: (THEME_SPACE_1)}
                //                     text: ""
                //                     draw_check: { check_type: None }
                //                     icon_walk: {width: 13.5 }
                //                     draw_icon: {
                //                         color: (THEME_COLOR_D_3),
                //                         color_active: (STUDIO_PALETTE_2),
                //                         svg_file: dep("crate://self/resources/icons/icon_widget.svg"),
                //                     }
                //                 }
                //                 <CheckBoxCustom> {
                //                     text: ""
                //                     draw_check: { check_type: None }
                //                     icon_walk: {width: 12.}
                //                     draw_icon: {
                //                         color: (THEME_COLOR_D_3),
                //                         color_active: (STUDIO_PALETTE_6),
                //                         svg_file: dep("crate://self/resources/icons/icon_layout.svg"),
                //                     }
                //                 }
                //                 <CheckBoxCustom> {
                //                     text: ""
                //                     draw_check: { check_type: None }
                //                     icon_walk: {width: 10.5}
                //                     draw_icon: {
                //                         color: (THEME_COLOR_D_3),
                //                         color_active: (STUDIO_PALETTE_1),
                //                         svg_file: dep("crate://self/resources/icons/icon_text.svg"),
                //                     }
                //                 }
                //                 <CheckBoxCustom> {
                //                     text:""
                //                     draw_check: { check_type: None }
                //                     icon_walk: {width: 13.}
                //                     draw_icon: {
                //                         color: (THEME_COLOR_D_3),
                //                         color_active: (STUDIO_PALETTE_5),
                //                         svg_file: dep("crate://self/resources/icons/icon_image.svg"),
                //                     }
                //                 }
                //             }
                //             <TextInput> {
                //                 width: Fill,
                //                 empty_message: "Filter",
                //             }
                //         }
                //     }
                //     outline_tree = <DesignerOutlineTree>{

                //     }
                // }
            }
        }
        <Window>{
            window:{ kind_id: 1 }
            body = <View>{
                flow: Overlay
                designer_view = <DesignerView> {
                    width: Fill, height: Fill
                }
                toolbox = <DesignerToolbox>{
                }
            }
        }
    }


    GRoot = <RootBase> { design_window = <Designer> {} }
    
}