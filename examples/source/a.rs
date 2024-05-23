[E:\Rust\try\makepad\Gen-UI\gen\compiler\src\core\compiler.rs:145:25] &self.target = Makepad(
    Some(
        Makepad {
            app_main: AppMain {
                name: "App",
                root_ref: "root",
                root_ref_ptr: "ui",
                props: None,
                match_event: MatchEventTrait {
                    global: None,
                    startup: None,
                    shutdown: None,
                    foreground: None,
                    background: None,
                    pause: None,
                    resume: None,
                    app_got_focus: None,
                    app_lost_focus: None,
                    next_frame: None,
                    action: None,
                    actions: None,
                    signal: None,
                    audio_devices: None,
                    midi_ports: None,
                    video_inputs: None,
                    http_response: None,
                    http_request_error: None,
                    http_progress: None,
                    network_responses: None,
                    draw: None,
                    timer: None,
                    draw_2d: None,
                    key_down: None,
                    key_up: None,
                    back_pressed: None,
                    match_event: None,
                    match_event_with_draw_2d: None,
                },
                app_main: AppMainTrait {
                    handle_event: None,
                },
                live_register: Some(
                    [
                        "views::root",
                        "views::a",
                    ],
                ),
                source: Source {
                    origin_dir: "E:\\Rust\\try\\makepad\\Gen-UI\\examples\\simple1\\ui",
                    origin_file: "E:\\Rust\\try\\makepad\\Gen-UI\\examples\\simple1\\ui\\app.gen",
                    compiled_dir: "E:\\Rust\\try\\makepad\\Gen-UI\\examples\\simple1\\src_gen",
                    compiled_file: "E:\\Rust\\try\\makepad\\Gen-UI\\examples\\simple1\\src_gen\\src\\app.rs",
                },
                uses: None,
            },
            tree: Some(
                ModelTree {
                    node: Widget(
                        Widget {
                            is_root: true,
                            is_prop: false,
                            is_built_in: true,
                            is_static: true,
                            id: Some(
                                "ui",
                            ),
                            name: "Window",
                            source: Some(
                                Source {
                                    origin_dir: "E:\\Rust\\try\\makepad\\Gen-UI\\examples\\simple1\\ui",
                                    origin_file: "E:/Rust/try/makepad/Gen-UI/examples/simple1/ui/views/root.gen",
                                    compiled_dir: "E:\\Rust\\try\\makepad\\Gen-UI\\examples\\simple1\\src_gen",
                                    compiled_file: "E:\\Rust\\try\\makepad\\Gen-UI\\examples\\simple1\\src_gen\\src\\views\\root.rs",
                                },
                            ),
                            uses: None,
                            props: Some(
                                TokenStream [
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
                                            Ident {
                                                sym: ffffff,
                                            },
                                        ],
                                    },
                                    Punct {
                                        char: ',',
                                        spacing: Alone,
                                    },
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
                                ],
                            ),
                            events: None,
                            prop_ptr: None,
                            event_ptr: None,
                            event_ref: None,
                            event_set: None,
                            children: Some(
                                [
                                    Widget {
                                        is_root: false,
                                        is_prop: false,
                                        is_built_in: true,
                                        is_static: true,
                                        id: None,
                                        name: "label",
                                        source: None,
                                        uses: None,
                                        props: Some(
                                            TokenStream [
                                                Ident {
                                                    sym: draw_text,
                                                },
                                                Punct {
                                                    char: ':',
                                                    spacing: Alone,
                                                },
                                                Group {
                                                    delimiter: Brace,
                                                    stream: TokenStream [
                                                        Ident {
                                                            sym: text_style,
                                                        },
                                                        Punct {
                                                            char: ':',
                                                            spacing: Alone,
                                                        },
                                                        Group {
                                                            delimiter: Brace,
                                                            stream: TokenStream [
                                                                Ident {
                                                                    sym: font_size,
                                                                },
                                                                Punct {
                                                                    char: ':',
                                                                    spacing: Alone,
                                                                },
                                                                Literal {
                                                                    lit: 32,
                                                                },
                                                                Punct {
                                                                    char: ',',
                                                                    spacing: Alone,
                                                                },
                                                            ],
                                                        },
                                                        Punct {
                                                            char: ',',
                                                            spacing: Alone,
                                                        },
                                                    ],
                                                },
                                                Punct {
                                                    char: ',',
                                                    spacing: Alone,
                                                },
                                                Ident {
                                                    sym: text,
                                                },
                                                Punct {
                                                    char: ':',
                                                    spacing: Alone,
                                                },
                                                Literal {
                                                    lit: "Hello",
                                                },
                                                Punct {
                                                    char: ',',
                                                    spacing: Alone,
                                                },
                                            ],
                                        ),
                                        events: None,
                                        prop_ptr: None,
                                        event_ptr: None,
                                        event_ref: None,
                                        event_set: None,
                                        children: None,
                                        inherits: Some(
                                            Label,
                                        ),
                                        traits: Some(
                                            WidgetTrait {
                                                draw_walk: TokenStream [],
                                                handle_event: None,
                                                widget: None,
                                                widgets: None,
                                                widget_id: None,
                                                widget_to_data: None,
                                                data_to_widget: None,
                                                draw: None,
                                                draw_walk_all: None,
                                                is_visible: None,
                                                draw_all: None,
                                                text: None,
                                                set_text: None,
                                                set_text_and_redraw: None,
                                                ref_cast_type_id: None,
                                            },
                                        ),
                                        role: Normal,
                                    },
                                ],
                            ),
                            inherits: Some(
                                Area,
                            ),
                            traits: Some(
                                WidgetTrait {
                                    draw_walk: TokenStream [],
                                    handle_event: None,
                                    widget: None,
                                    widgets: None,
                                    widget_id: None,
                                    widget_to_data: None,
                                    data_to_widget: None,
                                    draw: None,
                                    draw_walk_all: None,
                                    is_visible: None,
                                    draw_all: None,
                                    text: None,
                                    set_text: None,
                                    set_text_and_redraw: None,
                                    ref_cast_type_id: None,
                                },
                            ),
                            role: Normal,
                        },
                    ),
                    children: Some(
                        {
                            ModelTree {
                                node: Widget(
                                    Widget {
                                        is_root: true,
                                        is_prop: false,
                                        is_built_in: true,
                                        is_static: true,
                                        id: Some(
                                            "MyLabel",
                                        ),
                                        name: "Label",
                                        source: Some(
                                            Source {
                                                origin_dir: "E:\\Rust\\try\\makepad\\Gen-UI\\examples\\simple1\\ui",
                                                origin_file: "E:\\Rust\\try\\makepad\\Gen-UI\\examples\\simple1\\ui\\views\\a.gen",
                                                compiled_dir: "E:\\Rust\\try\\makepad\\Gen-UI\\examples\\simple1\\src_gen",       
                                                compiled_file: "E:\\Rust\\try\\makepad\\Gen-UI\\examples\\simple1\\src_gen\\src\\views\\a.rs",
                                            },
                                        ),
                                        uses: None,
                                        props: Some(
                                            TokenStream [
                                                Ident {
                                                    sym: text,
                                                },
                                                Punct {
                                                    char: ':',
                                                    spacing: Alone,
                                                },
                                                Literal {
                                                    lit: "a",
                                                },
                                                Punct {
                                                    char: ',',
                                                    spacing: Alone,
                                                },
                                            ],
                                        ),
                                        events: None,
                                        prop_ptr: None,
                                        event_ptr: None,
                                        event_ref: None,
                                        event_set: None,
                                        children: None,
                                        inherits: Some(
                                            Area,
                                        ),
                                        traits: Some(
                                            WidgetTrait {
                                                draw_walk: TokenStream [],
                                                handle_event: None,
                                                widget: None,
                                                widgets: None,
                                                widget_id: None,
                                                widget_to_data: None,
                                                data_to_widget: None,
                                                draw: None,
                                                draw_walk_all: None,
                                                is_visible: None,
                                                draw_all: None,
                                                text: None,
                                                set_text: None,
                                                set_text_and_redraw: None,
                                                ref_cast_type_id: None,
                                            },
                                        ),
                                        role: Normal,
                                    },
                                ),
                                children: None,
                            },
                        },
                    ),
                },
            ),
            main_rs: RsFile {
                source: Source {
                    origin_dir: "E:\\Rust\\try\\makepad\\Gen-UI\\examples\\simple1\\ui",
                    origin_file: "E:\\Rust\\try\\makepad\\Gen-UI\\examples\\simple1\\ui\\src\\main.rs",
                    compiled_dir: "E:\\Rust\\try\\makepad\\Gen-UI\\examples\\simple1\\src_gen",
                    compiled_file: "E:\\Rust\\try\\makepad\\Gen-UI\\examples\\simple1\\src_gen\\src\\main.rs",
                },
                content: TokenStream [
                    Ident {
                        sym: fn,
                    },
                    Ident {
                        sym: main,
                    },
                    Group {
                        delimiter: Parenthesis,
                        stream: TokenStream [],
                    },
                    Group {
                        delimiter: Brace,
                        stream: TokenStream [
                            Ident {
                                sym: src_gen,
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
                                sym: app,
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
                                sym: app_main,
                            },
                            Group {
                                delimiter: Parenthesis,
                                stream: TokenStream [],
                            },
                        ],
                    },
                ],
            },
        },
    ),
)