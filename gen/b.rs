Block {
    brace_token: Brace,
    stmts: [
        Stmt::Item(
            Item::Enum {
                attrs: [
                    Attribute {
                        pound_token: Pound,
                        style: AttrStyle::Outer,
                        bracket_token: Bracket,
                        meta: Meta::List {
                            path: Path {
                                leading_colon: None,
                                segments: [
                                    PathSegment {
                                        ident: Ident(
                                            derive,
                                        ),
                                        arguments: PathArguments::None,
                                    },
                                ],
                            },
                            delimiter: MacroDelimiter::Paren(
                                Paren,
                            ),
                            tokens: TokenStream [
                                Ident {
                                    sym: Event,
                                },
                                Punct {
                                    char: ',',
                                    spacing: Alone,
                                },
                                Ident {
                                    sym: Debug,
                                },
                                Punct {
                                    char: ',',
                                    spacing: Alone,
                                },
                                Ident {
                                    sym: Clone,
                                },
                                Punct {
                                    char: ',',
                                    spacing: Alone,
                                },
                                Ident {
                                    sym: Default,
                                },
                            ],
                        },
                    },
                ],
                vis: Visibility::Public(
                    Pub,
                ),
                enum_token: Enum,
                ident: Ident(
                    Events,
                ),
                generics: Generics {
                    lt_token: None,
                    params: [],
                    gt_token: None,
                    where_clause: None,
                },
                brace_token: Brace,
                variants: [
                    Variant {
                        attrs: [
                            Attribute {
                                pound_token: Pound,
                                style: AttrStyle::Outer,
                                bracket_token: Bracket,
                                meta: Meta::List {
                                    path: Path {
                                        leading_colon: None,
                                        segments: [
                                            PathSegment {
                                                ident: Ident(
                                                    name,
                                                ),
                                                arguments: PathArguments::None,
                                            },
                                        ],
                                    },
                                    delimiter: MacroDelimiter::Paren(
                                        Paren,
                                    ),
                                    tokens: TokenStream [
                                        Literal {
                                            lit: "click",
                                        },
                                    ],
                                },
                            },
                        ],
                        ident: Ident(
                            Clicked,
                        ),
                        fields: Fields::Unnamed {
                            paren_token: Paren,
                            unnamed: [
                                Field {
                                    attrs: [],
                                    vis: Visibility::Inherited,
                                    mutability: FieldMutability::None,
                                    ident: None,
                                    colon_token: None,
                                    ty: Type::Path {
                                        qself: None,
                                        path: Path {
                                            leading_colon: None,
                                            segments: [
                                                PathSegment {
                                                    ident: Ident(
                                                        String,
                                                    ),
                                                    arguments: PathArguments::None,
                                                },
                                            ],
                                        },
                                    },
                                },
                            ],
                        },
                        discriminant: None,
                    },
                    Comma,
                    Variant {
                        attrs: [],
                        ident: Ident(
                            None,
                        ),
                        fields: Fields::Unit,
                        discriminant: None,
                    },
                    Comma,
                ],
            },
        ),
        Stmt::Item(
            Item::Struct {
                attrs: [
                    Attribute {
                        pound_token: Pound,
                        style: AttrStyle::Outer,
                        bracket_token: Bracket,
                        meta: Meta::List {
                            path: Path {
                                leading_colon: None,
                                segments: [
                                    PathSegment {
                                        ident: Ident(
                                            derive,
                                        ),
                                        arguments: PathArguments::None,
                                    },
                                ],
                            },
                            delimiter: MacroDelimiter::Paren(
                                Paren,
                            ),
                            tokens: TokenStream [
                                Ident {
                                    sym: Debug,
                                },
                                Punct {
                                    char: ',',
                                    spacing: Alone,
                                },
                                Ident {
                                    sym: Prop,
                                },
                                Punct {
                                    char: ',',
                                    spacing: Alone,
                                },
                                Ident {
                                    sym: Default,
                                },
                                Punct {
                                    char: ',',
                                    spacing: Alone,
                                },
                                Ident {
                                    sym: Clone,
                                },
                            ],
                        },
                    },
                ],
                vis: Visibility::Public(
                    Pub,
                ),
                struct_token: Struct,
                ident: Ident(
                    MyProps,
                ),
                generics: Generics {
                    lt_token: None,
                    params: [],
                    gt_token: None,
                    where_clause: None,
                },
                fields: Fields::Named {
                    brace_token: Brace,
                    named: [
                        Field {
                            attrs: [],
                            vis: Visibility::Public(
                                Pub,
                            ),
                            mutability: FieldMutability::None,
                            ident: Some(
                                Ident(
                                    label1,
                                ),
                            ),
                            colon_token: Some(
                                Colon,
                            ),
                            ty: Type::Path {
                                qself: None,
                                path: Path {
                                    leading_colon: None,
                                    segments: [
                                        PathSegment {
                                            ident: Ident(
                                                String,
                                            ),
                                            arguments: PathArguments::None,
                                        },
                                    ],
                                },
                            },
                        },
                    ],
                },
                semi_token: None,
            },
        ),
        Stmt::Local {
            attrs: [],
            let_token: Let,
            pat: Pat::Type {
                attrs: [],
                pat: Pat::Ident {
                    attrs: [],
                    by_ref: None,
                    mutability: None,
                    ident: Ident(
                        fs,
                    ),
                    subpat: None,
                },
                colon_token: Colon,
                ty: Type::Path {
                    qself: None,
                    path: Path {
                        leading_colon: None,
                        segments: [
                            PathSegment {
                                ident: Ident(
                                    f64,
                                ),
                                arguments: PathArguments::None,
                            },
                        ],
                    },
                },
            },
            init: Some(
                LocalInit {
                    eq_token: Eq,
                    expr: Expr::Lit {
                        attrs: [],
                        lit: Lit::Float {
                            token: 18.0,
                        },
                    },
                    diverge: None,
                },
            ),
            semi_token: Semi,
        },
        Stmt::Local {
            attrs: [],
            let_token: Let,
            pat: Pat::Ident {
                attrs: [],
                by_ref: None,
                mutability: Some(
                    Mut,
                ),
                ident: Ident(
                    btn_click,
                ),
                subpat: None,
            },
            init: Some(
                LocalInit {
                    eq_token: Eq,
                    expr: Expr::Closure {
                        attrs: [],
                        lifetimes: None,
                        constness: None,
                        movability: None,
                        asyncness: None,
                        capture: None,
                        or1_token: Or,
                        inputs: [],
                        or2_token: Or,
                        output: ReturnType::Default,
                        body: Expr::Block {
                            attrs: [],
                            label: None,
                            block: Block {
                                brace_token: Brace,
                                stmts: [
                                    Stmt::Macro {
                                        attrs: [],
                                        mac: Macro {
                                            path: Path {
                                                leading_colon: None,
                                                segments: [
                                                    PathSegment {
                                                        ident: Ident(
                                                            println,
                                                        ),
                                                        arguments: PathArguments::None,
                                                    },
                                                ],
                                            },
                                            bang_token: Not,
                                            delimiter: MacroDelimiter::Paren(
                                                Paren,
                                            ),
                                            tokens: TokenStream [
                                                Literal {
                                                    lit: "Button bb Clicked",
                                                },
                                            ],
                                        },
                                        semi_token: Some(
                                            Semi,
                                        ),
                                    },
                                    Stmt::Macro {
                                        attrs: [],
                                        mac: Macro {
                                            path: Path {
                                                leading_colon: None,
                                                segments: [
                                                    PathSegment {
                                                        ident: Ident(
                                                            active,
                                                        ),
                                                        arguments: PathArguments::None,
                                                    },
                                                ],
                                            },
                                            bang_token: Not,
                                            delimiter: MacroDelimiter::Paren(
                                                Paren,
                                            ),
                                            tokens: TokenStream [
                                                Ident {
                                                    sym: Events,
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
                                                    sym: Clicked,
                                                },
                                                Punct {
                                                    char: ',',
                                                    spacing: Alone,
                                                },
                                                Literal {
                                                    lit: "Hello",
                                                },
                                                Punct {
                                                    char: '.',
                                                    spacing: Alone,
                                                },
                                                Ident {
                                                    sym: to_string,
                                                },
                                                Group {
                                                    delimiter: Parenthesis,
                                                    stream: TokenStream [],
                                                },
                                            ],
                                        },
                                        semi_token: Some(
                                            Semi,
                                        ),
                                    },
                                ],
                            },
                        },
                    },
                    diverge: None,
                },
            ),
            semi_token: Semi,
        },
        Stmt::Item(
            Item::Fn {
                attrs: [],
                vis: Visibility::Inherited,
                sig: Signature {
                    constness: None,
                    asyncness: None,
                    unsafety: None,
                    abi: None,
                    fn_token: Fn,
                    ident: Ident(
                        hello,
                    ),
                    generics: Generics {
                        lt_token: None,
                        params: [],
                        gt_token: None,
                        where_clause: None,
                    },
                    paren_token: Paren,
                    inputs: [],
                    variadic: None,
                    output: ReturnType::Default,
                },
                block: Block {
                    brace_token: Brace,
                    stmts: [
                        Stmt::Macro {
                            attrs: [],
                            mac: Macro {
                                path: Path {
                                    leading_colon: None,
                                    segments: [
                                        PathSegment {
                                            ident: Ident(
                                                println,
                                            ),
                                            arguments: PathArguments::None,
                                        },
                                    ],
                                },
                                bang_token: Not,
                                delimiter: MacroDelimiter::Paren(
                                    Paren,
                                ),
                                tokens: TokenStream [
                                    Literal {
                                        lit: "{}",
                                    },
                                    Punct {
                                        char: ',',
                                        spacing: Alone,
                                    },
                                    Ident {
                                        sym: hello,
                                    },
                                ],
                            },
                            semi_token: Some(
                                Semi,
                            ),
                        },
                    ],
                },
            },
        ),
        Stmt::Macro {
            attrs: [],
            mac: Macro {
                path: Path {
                    leading_colon: None,
                    segments: [
                        PathSegment {
                            ident: Ident(
                                on_shutdown,
                            ),
                            arguments: PathArguments::None,
                        },
                    ],
                },
                bang_token: Not,
                delimiter: MacroDelimiter::Brace(
                    Brace,
                ),
                tokens: TokenStream [
                    Ident {
                        sym: println,
                    },
                    Punct {
                        char: '!',
                        spacing: Alone,
                    },
                    Group {
                        delimiter: Parenthesis,
                        stream: TokenStream [
                            Literal {
                                lit: "{}",
                            },
                            Punct {
                                char: ',',
                                spacing: Alone,
                            },
                            Literal {
                                lit: "on shutdown!",
                            },
                        ],
                    },
                    Punct {
                        char: ';',
                        spacing: Alone,
                    },
                    Ident {
                        sym: println,
                    },
                    Punct {
                        char: '!',
                        spacing: Alone,
                    },
                    Group {
                        delimiter: Parenthesis,
                        stream: TokenStream [
                            Literal {
                                lit: "GenUI: {}",
                            },
                            Punct {
                                char: ',',
                                spacing: Alone,
                            },
                            Literal {
                                lit: "good bye!",
                            },
                        ],
                    },
                    Punct {
                        char: ';',
                        spacing: Alone,
                    },
                ],
            },
            semi_token: None,
        },
    ],
},