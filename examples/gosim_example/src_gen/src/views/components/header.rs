use makepad_widgets::*;
live_design! { 
    import makepad_widgets :: base :: * ; 
    import makepad_widgets :: theme_desktop_dark :: * ; 
    import makepad_draw :: shader :: std :: * ; 
    header = <View >{ 
        draw_bg : { 
            fn pixel (self) -> vec4 { return vec4 (1.0 , 1.0 , 1.0 , 1.0) } } , 
            show_bg : true , padding : { top : 0 , right : 32 , bottom : 0 , left : 32 } , 
            spacing : 0 , width : All , height : 60 , 
            logo_wrap = < View >{ 
                align : { x : 0.5 , y : 0.5 } , width : Fit , height : Fill , 
                logo = < Image >{ width : 120 , height : 30 , source : dep ("crate://self/static/logo.png") , } 
                menu_list = < View >{ 
                    draw_bg : { color : # FF0000FF } , show_bg : true , align : { x : 0.5 , y : 0.5 } , spacing : 20 , width : 500 , height : Fill , 
                    < Label >{ draw_text : { text_style : { font_size : 12 , } , fn get_color (self) -> vec4 { return vec4 (0.0 , 0.0 , 0.0 , 1.0) } } , text : "About" , } 
                    < Label >{ draw_text : { text_style : { font_size : 12 , } , fn get_color (self) -> vec4 { return vec4 (0.0 , 0.0 , 0.0 , 1.0) } } , text : "Founders" , } 
                    < Label >{ draw_text : { text_style : { font_size : 12 , } , fn get_color (self) -> vec4 { return vec4 (0.0 , 0.0 , 0.0 , 1.0) } } , text : "Events" , } 
                    btn_wrap = < View >{ align : { x : 0.5 , y : 0.5 } , width : 220 , height : Fill , event_btn = < Button >{ draw_bg : { uniform border_radius : 3.0 fn pixel (self) -> vec4 { let sdf = Sdf2d :: viewport (self . pos * self . rect_size) let bg_color = mix (vec4 (0.0 , 0.427 , 0.529 , 1.0) , vec4 (0.517 , 0.912 , 0.972 , 1.0) , self . hover) sdf . box (1.0 , 1.0 , self . rect_size . x - 2.0 , self . rect_size . y - 2.0 , self . border_radius) sdf . fill_keep (bg_color) return sdf . result } } , draw_text : { text_style : { font : { path : dep ("crate://self/static/GoNotoKurrent-Bold.ttf") } , } , fn get_color (self) -> vec4 { return mix (vec4 (1.0 , 1.0 , 1.0 , 1.0) , vec4 (0.0 , 0.0 , 0.0 , 1.0) , self . hover) } } , width : 180 , height : Fit , padding : { top : 16 , right : 16 , bottom : 16 , left : 16 } , text : "Upcoming Events" , } } } } } }
