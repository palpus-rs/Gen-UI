use makepad_widgets ::* ; 
live_design ! { 
    import makepad_widgets :: base ::*; 
    import makepad_widgets :: theme_desktop_dark ::*; 

    UIROOT = < Window >{ 
        show_bg : true , 
        body = < View >{ 
            btn = < Button >{
                text: "Click Me!",
            } 
        } 
    } 
} 
