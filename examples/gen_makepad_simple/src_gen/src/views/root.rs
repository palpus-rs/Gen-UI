use makepad_widgets :: * ; live_design ! { import makepad_widgets :: base ::*; import makepad_widgets :: theme_desktop_dark ::*; import crate :: views :: components :: labels ::* ; import crate :: views :: components :: checkbox ::* ; import crate :: views :: components :: button_view ::* ; ui = <Root >{ main_window = < Window >{ window : { position : vec2 (300 , 300) , inner_size : vec2 (600 , 600) , } draw_bg : { fn pixel (self) -> vec4 { return vec4 (0.6 , 0.4 , 0.5 , 1.0) } } , show_bg : true , flow : Down , width : Fill , height : Fill , < View >{ align : { x : 0.5 , y : 0.5 } , flow : Down , height : All , < Label >{ draw_text : { text_style : { font_size : 16 , } , } , text : "Gen + Makepad Project Hello World!!!" , } < label_view >{ } < checkbox_view >{ } < button_view >{ } } } } }