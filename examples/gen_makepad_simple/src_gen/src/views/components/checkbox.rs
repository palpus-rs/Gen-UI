use makepad_widgets :: * ; live_design ! { import makepad_widgets :: base :: * ; import makepad_widgets :: theme_desktop_dark :: * ; import makepad_draw :: shader :: std :: * ; checkbox_view = <View >{ flow : Down , width : 300 , height : 300 , < Slider >{ text : "default slider" , min : 1 , max : 10 , step : 0.5 , } < CheckBox >{ margin : { top : 10 , right : 10 , bottom : 10 , left : 10 } , icon_walk : { margin : { top : 0 , right : 0 , bottom : 0 , left : 10 } , } , draw_check : { check_type : Radio , } , text : "CheckBox1" , } < CheckBox >{ margin : { top : 10 , right : 10 , bottom : 10 , left : 10 } , icon_walk : { margin : { top : 0 , right : 0 , bottom : 0 , left : 10 } , } , text : "CheckBox1" , } checkbox2 = < CheckBox >{ icon_walk : { margin : { top : 0 , right : 0 , bottom : 0 , left : 16 } , } , draw_check : { check_type : Toggle , } , draw_text : { text_style : { brightness : 1.5 , } , } , text : "Checkbox Toggle" , } radio1 = < RadioButton >{ draw_text : { text_style : { font_size : 16 , } , } , margin : { top : 16 , right : 16 , bottom : 16 , left : 16 } , text : "Radio1" , } radio2 = < RadioButton >{ draw_radio : { radio_type : Tab , } , margin : { top : 0 , right : 16 , bottom : 0 , left : 16 } , height : 32 , padding : { top : 12 , right : 12 , bottom : 12 , left : 12 } , label_walk : { margin : { top : 0 , right : 0 , bottom : 0 , left : 0 } , } , label_align : { x : 0.5 , y : 0.5 } , text : "Radio Tab" , } < CheckBox >{ margin : { top : 10 , right : 10 , bottom : 10 , left : 10 } , icon_walk : { margin : { top : 0 , right : 0 , bottom : 0 , left : 10 } , } , draw_check : { check_type : None , } , text : "CheckBox1" , } < Button >{ text : "click" , } < Label >{ margin : { top : 16 , right : 16 , bottom : 16 , left : 16 } , } } }