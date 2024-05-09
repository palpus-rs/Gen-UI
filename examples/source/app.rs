use makepad_widgets ::* ; 
live_design ! { 
    import makepad_widgets::base::*; 
    import makepad_widgets::theme_desktop_dark::*; 
    import crate::app_main::*;
    

    App = {{ App }}{ 
       ui : <UIROOT>{}
    } 
} 


# [derive (Live , LiveHook ,)] 
pub struct App { 
    #[live] pub ui : WidgetRef , 
} 

impl MatchEvent for App {
    
} 

impl AppMain for App { 
    fn handle_event (& mut self , cx : & mut Cx , event : & Event) { 
        self .match_event (cx , event) ; 
        self . ui . handle_event (cx , event , & mut Scope :: empty ()) ; 
    } 
} 

impl LiveRegister for App { 
    fn live_register (cx : & mut Cx) { 
        crate :: makepad_widgets ::live_design (cx) ;
        crate::app_main::live_design(cx);
    } 
} 

app_main ! (App) ;