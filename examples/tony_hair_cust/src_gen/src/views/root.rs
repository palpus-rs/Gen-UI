use makepad_widgets :: * ; live_design ! { import makepad_widgets :: base :: * ; import makepad_widgets :: theme_desktop_dark :: * ; import makepad_draw :: shader :: std :: * ; ui = <Root >{ main_window = < Window >{ window : { position : vec2 (300 , 300) , inner_size : vec2 (1024 , 660) , } flow : Down , width : Fill , height : Fill , main_view = < ScrollYView >{ draw_bg : { fn pixel (self) -> vec4 { return vec4 (1.0 , 1.0 , 1.0 , 1.0) } } , show_bg : true , flow : Down , width : All , height : All , bg = < View >{ draw_bg : { } , show_bg : true , } } } } }