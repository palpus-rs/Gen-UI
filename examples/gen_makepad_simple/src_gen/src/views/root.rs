use makepad_widgets :: * ; live_design ! { import makepad_widgets :: base :: * ; import makepad_widgets :: theme_desktop_dark :: * ; import makepad_draw :: shader :: std :: * ; import crate :: views :: components :: labels ::* ; import crate :: views :: components :: checkbox ::* ; import crate :: views :: components :: button_view ::* ; import crate :: views :: components :: drop_down_view ::* ; ui = <Root >{ main_window = < Window >{ window : { position : vec2 (300 , 300) , inner_size : vec2 (600 , 600) , } draw_bg : { fn pixel (self) -> vec4 { return vec4 (0.6 , 0.4 , 0.5 , 1.0) } } , show_bg : true , flow : Down , width : Fill , height : Fill , caption_bar = { caption_label = { label = { text : "GenUI + Makepad Simple Example" } } } , < ScrollYView >{ align : { x : 0.5 , y : 0.5 } , flow : Down , height : All , < Label >{ draw_text : { text_style : { font_size : 16 , } , } , text : "Gen + Makepad Project(Makepad UI Zoo)" , } < label_view >{ } < drop_down_view >{ } < checkbox_view >{ } < button_view >{ } } } } }