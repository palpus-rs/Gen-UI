let a = [
    Ident {
        sym: use,
    },
    Ident {
        sym: makepad_widgets,
    },
    Punct {
        char: ':',
        spacing: Joint,
    },
    Punct {
        char: ':',
        spacing: Joint,
    },
    Punct {
        char: '*',
        spacing: Joint,
    },
    Punct {
        char: ';',
        spacing: Alone,
    },
    Ident {
        sym: live_design,
    },
    Punct {
        char: '!',
        spacing: Alone,
    },
    Group {
        delimiter: Brace,
        stream: TokenStream [
            Ident {
                sym: import,
            },
            Ident {
                sym: makepad_widgets,    
            },
            Punct {
                char: ':',
                spacing: Joint,
            },
            Punct {
                char: ':',
                spacing: Alone,
            },
            Ident {
                sym: base,
            },
            Punct {
                char: ':',
                spacing: Joint,
            },
            Punct {
                char: ':',
                spacing: Joint,
            },
            Punct {
                char: '*',
                spacing: Joint,
            },
            Punct {
                char: ';',
                spacing: Alone,
            },
            Ident {
                sym: import,
            },
            Ident {
                sym: makepad_widgets,    
            },
            Punct {
                char: ':',
                spacing: Joint,
            },
            Punct {
                char: ':',
                spacing: Alone,
            },
            Ident {
                sym: theme_desktop_dark, 
            },
            Punct {
                char: ':',
                spacing: Joint,
            },
            Punct {
                char: ':',
                spacing: Joint,
            },
            Punct {
                char: '*',
                spacing: Joint,
            },
            Punct {
                char: ';',
                spacing: Alone,
            },
            Ident {
                sym: App,
            },
            Punct {
                char: '=',
                spacing: Alone,
            },
            Group {
                delimiter: Brace,        
                stream: TokenStream [    
                    Group {
                        delimiter: Brace,
                        stream: TokenStream [
                            Ident {      
                                sym: App,
                            },
                        ],
                    },
                ],
            },
            Group {
                delimiter: Brace,        
                stream: TokenStream [    
                    Ident {
                        sym: ui,
                    },
                    Punct {
                        char: ':',       
                        spacing: Alone,  
                    },
                    Punct {
                        char: '<',       
                        spacing: Alone,  
                    },
                    Ident {
                        sym: Window,     
                    },
                    Punct {
                        char: '>',       
                        spacing: Alone,  
                    },
                    Group {
                        delimiter: Brace,
                        stream: TokenStream [
                            Ident {      
                                sym: show_bg,
                            },
                            Punct {      
                                char: ':',
                                spacing: Alone,
                            },
                            Ident {      
                                sym: true,
                            },
                            Punct {      
                                char: ',',
                                spacing: Alone,
                            },
                            Ident {      
                                sym: width,
                            },
                            Punct {      
                                char: ':',
                                spacing: Alone,
                            },
                            Ident {      
                                sym: Fill,
                            },
                            Punct {      
                                char: ',',
                                spacing: Alone,
                            },
                            Ident {      
                                sym: draw_bg,
                            },
                            Punct {      
                                char: ':',
                                spacing: Alone,
                            },
                            Group {      
                                delimiter: Brace,
                                stream: TokenStream [
                                    Ident {
                                        sym: color,
                                    },   
                                    Punct {
                                        char: ':',
                                        spacing: Alone,
                                    },   
                                    Punct {
                                        char: '#',
                                        spacing: Alone,
                                    },   
                                    Literal {
                                        lit: 96CEF8,
                                    },   
                                ],       
                            },
                            Punct {      
                                char: ',',
                                spacing: Alone,
                            },
                            Ident {      
                                sym: height,
                            },
                            Punct {      
                                char: ':',
                                spacing: Alone,
                            },
                            Ident {      
                                sym: Fill,
                            },
                            Punct {      
                                char: ',',
                                spacing: Alone,
                            },
                            Ident {      
                                sym: body,
                            },
                            Punct {      
                                char: '=',
                                spacing: Alone,
                            },
                            Punct {      
                                char: '<',
                                spacing: Alone,
                            },
                            Ident {      
                                sym: View,
                            },
                            Punct {      
                                char: '>',
                                spacing: Alone,
                            },
                            Group {      
                                delimiter: Brace,
                                stream: TokenStream [
                                    Ident {
                                        sym: align,
                                    },   
                                    Punct {
                                        char: ':',
                                        spacing: Alone,
                                    },   
                                    Group {
                                        delimiter: Brace,
                                        stream: TokenStream [
                                         
   Ident {
                                         
       sym: x,
                                         
   },
                                         
   Punct {
                                         
       char: ':',
                                         
       spacing: Alone,
                                         
   },
                                         
   Literal {
                                         
       lit: 0.5,
                                         
   },
                                         
   Punct {
                                         
       char: ',',
                                         
       spacing: Alone,
                                         
   },
                                         
   Ident {
                                         
       sym: y,
                                         
   },
                                         
   Punct {
                                         
       char: ':',
                                         
       spacing: Alone,
                                         
   },
                                         
   Literal {
                                         
       lit: 0.5,
                                         
   },
                                        ],
                                    },   
                                    Punct {
                                        char: ',',
                                        spacing: Alone,
                                    },   
                                ],       
                            },
                        ],
                    },
                ],
            },
        ],
    },
    Punct {
        char: '#',
        spacing: Alone,
    },
    Group {
        delimiter: Bracket,
        stream: TokenStream [
            Ident {
                sym: derive,
            },
            Group {
                delimiter: Parenthesis,  
                stream: TokenStream [    
                    Ident {
                        sym: Live,       
                    },
                    Punct {
                        char: ',',       
                        spacing: Alone,  
                    },
                    Ident {
                        sym: LiveHook,   
                    },
                ],
            },
        ],
    },
    Ident {
        sym: pub,
    },
    Ident {
        sym: struct,
    },
    Ident {
        sym: App,
    },
    Group {
        delimiter: Brace,
        stream: TokenStream [
            Punct {
                char: '#',
                spacing: Alone,
            },
            Group {
                delimiter: Bracket,      
                stream: TokenStream [    
                    Ident {
                        sym: live,       
                    },
                ],
            },
            Ident {
                sym: ui,
            },
            Punct {
                char: ':',
                spacing: Alone,
            },
            Ident {
                sym: WidgetRef,
            },
            Punct {
                char: ',',
                spacing: Alone,
            },
        ],
    },
    Ident {
        sym: impl,
    },
    Ident {
        sym: LiveRegister,
    },
    Ident {
        sym: for,
    },
    Ident {
        sym: App,
    },
    Group {
        delimiter: Brace,
        stream: TokenStream [
            Ident {
                sym: fn,
            },
            Ident {
                sym: live_register,      
            },
            Group {
                delimiter: Parenthesis,  
                stream: TokenStream [    
                    Ident {
                        sym: cx,
                    },
                    Punct {
                        char: ':',       
                        spacing: Alone,  
                    },
                    Punct {
                        char: '&',       
                        spacing: Alone,  
                    },
                    Ident {
                        sym: mut,        
                    },
                    Ident {
                        sym: Cx,
                    },
                ],
            },
            Group {
                delimiter: Brace,        
                stream: TokenStream [    
                    Ident {
                        sym: crate,      
                    },
                    Punct {
                        char: ':',       
                        spacing: Joint,  
                    },
                    Punct {
                        char: ':',       
                        spacing: Alone,  
                    },
                    Ident {
                        sym: makepad_widgets,
                    },
                    Punct {
                        char: ':',       
                        spacing: Joint,  
                    },
                    Punct {
                        char: ':',       
                        spacing: Alone,  
                    },
                    Ident {
                        sym: live_design,
                    },
                    Group {
                        delimiter: Parenthesis,
                        stream: TokenStream [
                            Ident {      
                                sym: cx, 
                            },
                        ],
                    },
                    Punct {
                        char: ';',       
                        spacing: Alone,  
                    },
                ],
            },
        ],
    },
    Ident {
        sym: impl,
    },
    Ident {
        sym: AppMain,
    },
    Ident {
        sym: for,
    },
    Ident {
        sym: App,
    },
    Group {
        delimiter: Brace,
        stream: TokenStream [
            Ident {
                sym: fn,
            },
            Ident {
                sym: handle_event,       
            },
            Group {
                delimiter: Parenthesis,  
                stream: TokenStream [    
                    Punct {
                        char: '&',       
                        spacing: Alone,  
                    },
                    Ident {
                        sym: mut,        
                    },
                    Ident {
                        sym: self,       
                    },
                    Punct {
                        char: ',',       
                        spacing: Alone,  
                    },
                    Ident {
                        sym: cx,
                    },
                    Punct {
                        char: ':',       
                        spacing: Alone,  
                    },
                    Punct {
                        char: '&',       
                        spacing: Alone,  
                    },
                    Ident {
                        sym: mut,        
                    },
                    Ident {
                        sym: Cx,
                    },
                    Punct {
                        char: ',',       
                        spacing: Alone,  
                    },
                    Ident {
                        sym: event,      
                    },
                    Punct {
                        char: ':',       
                        spacing: Alone,  
                    },
                    Punct {
                        char: '&',       
                        spacing: Alone,  
                    },
                    Ident {
                        sym: Event,      
                    },
                ],
            },
            Group {
                delimiter: Brace,        
                stream: TokenStream [    
                    Ident {
                        sym: self,       
                    },
                    Punct {
                        char: '.',       
                        spacing: Alone,  
                    },
                    Ident {
                        sym: ui,
                    },
                    Punct {
                        char: '.',       
                        spacing: Alone,  
                    },
                    Ident {
                        sym: handle_event,
                    },
                    Group {
                        delimiter: Parenthesis,
                        stream: TokenStream [
                            Ident {      
                                sym: cx, 
                            },
                            Punct {      
                                char: ',',
                                spacing: Alone,
                            },
                            Ident {      
                                sym: event,
                            },
                            Punct {      
                                sym: mut,                            },
                            Ident {
                                sym: Scope,
                            },
                            Punct {
                                char: ':',
                                spacing: Joint,
                            },
                            Punct {
                                char: ':',
                                spacing: Alone,
                            },
                            Ident {
                                sym: empty,
                            },
                            Group {
                                delimiter: Parenthesis,
                                stream: TokenStream [],
                            },
                        ],
                    },
                    Punct {
                        char: ';',
                        spacing: Alone,
                    },
                ],
            },
        ],
    },
    Ident {
        sym: app_main,
    },
    Punct {
        char: '!',
        spacing: Alone,
    },
    Group {
        delimiter: Parenthesis,
        stream: TokenStream [
            Ident {
                sym: App,
            },
        ],
    },
    Punct {
        char: ';',
        spacing: Alone,
    },
]