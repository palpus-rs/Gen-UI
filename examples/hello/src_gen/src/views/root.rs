use makepad_widgets::*;
live_design! { import makepad_widgets :: base ::*; import makepad_widgets :: theme_desktop_dark ::*; ui = <Window >{ draw_bg : { color : # 1C2128 } , show_bg : true , width : Fill , height : Fill , < View >{ < Label >{ text : "hello!!!" , } icon1 = < Icon >{ draw_icon : { svg_file : dep ("crate://self/static/all.svg") , } , } } } }
