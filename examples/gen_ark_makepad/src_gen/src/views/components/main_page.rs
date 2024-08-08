use makepad_widgets::*;
live_design! { 
    import makepad_widgets :: base :: * ; 
    import makepad_widgets :: theme_desktop_dark :: * ; 
    import makepad_draw :: shader :: std :: * ; 
    MainPage = <View >{ 
        show_bg : true , padding : { top : 32 , right : 16 , bottom : 0 , left : 16 } , 
        spacing : 0 , width : All , height : 600 , < View >{ align : { x : 0.5 , y : 0.5 } , 
        spacing : 32 , width : Fill , height : Fill , < View >{ flow : Down , spacing : 12 , width : 400 , 
            gosim_img = < Image >{ 
                width : 400 , height : 360 , source : dep ("crate://self/resources/gosim.png") , 
            } 
            < Label >{ 
                draw_text : { text_style : { font : { path : dep ("crate://self/resources/GoNotoKurrent-Bold.ttf") } , font_size : 16 , } , 
                color : vec4 (0.21960784 , 0.21568628 , 0.23921569 , 1.0) , } , width : Fill , height : Fit , text : "Gosim Example" , 
            } 
            < Label >{ 
                draw_text : { text_style : { font : { path : dep ("crate://self/resources/GoNotoKurrent-Bold.ttf") } , font_size : 10 , } , 
                color : vec4 (0.21960784 , 0.21568628 , 0.23921569 , 1.0) , } , width : Fill , height : Fit , 
                text : "We imitated a set of static page examples created by Gosim, through which you can learn how to write GenUI's static pages" , 
            } 
            < LinkLabel >{ 
                draw_text : { wrap : Word , fn get_color (self) -> vec4 { return mix (# ff7733 , # ffdb69 , self . hover) } } , 
                width : Fill , text : "https://github.com/palpus-rs/GenUI/tree/ark/examples/gosim_example" , } 
            } 
            < View >{ 
                flow : Down , spacing : 12 , width : 400 , 
                components_img = < Image >{ 
                    width : 400 , height : 360 , source : dep ("crate://self/resources/easy.png") , 
                } 
                < Label >{ 
                    draw_text : { 
                        text_style : { font : { path : dep ("crate://self/resources/GoNotoKurrent-Bold.ttf") } , font_size : 16 , } , 
                        color : vec4 (0.21960784 , 0.21568628 , 0.23921569 , 1.0) , } , width : Fill , height : Fit , text : "Easy Example" , 
                    } < Label >{ draw_text : { text_style : { font : { path : dep ("crate://self/resources/GoNotoKurrent-Bold.ttf") } , font_size : 10 , } , color : vec4 (0.21960784 , 0.21568628 , 0.23921569 , 1.0) , } , width : Fill , height : Fit , text : "This case will demonstrate how to use GenUI syntax to build Makepad widgets" , } < LinkLabel >{ draw_text : { wrap : Word , fn get_color (self) -> vec4 { return mix (# ff7733 , # ffdb69 , self . hover) } } , width : Fill , text : "https://github.com/palpus-rs/GenUI/tree/ark/examples/gen_makepad_simple" , } } } } }
