use makepad_widgets :: * ; live_design ! { import makepad_widgets :: base ::*; import makepad_widgets :: theme_desktop_dark ::*; import crate :: views :: checkbox ::* ; import crate :: views :: header :: header ::* ; ui = <Window >{ draw_bg : { color : # 1C2128 } , show_bg : true , flow : Down , width : Fill , height : Fill , < View >{ <checkbox_view>{}} } }