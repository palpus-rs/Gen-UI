## easy.gen

```rust
[tests/src/main.rs:20] model_view.unwrap() = Model {
    special: "/Users/user/Workspace/others/beyond-framework/gen/tests/ui/view/easy.gen",
    template: Some(
        TemplateModel {
            special: Some(
                "01HTC6V3PWH02EK235N9ACXM8A",
            ),
            class: None,
            id: Some(
                "ui",
            ),
            name: "window",
            props: Some(
                {
                    PropsKey {
                        name: "background_visible",
                        is_style: false,
                        ty: Normal,
                    }: UnKnown(
                        "true",
                    ),
                },
            ),
            prop_ptr: NoProps,
            callbacks: None,
            event_ptr: None,
            inherits: None,
            root: true,
            children: Some(
                [
                    TemplateModel {
                        special: Some(
                            "01HTC6V3PWH02EK235N9ACXM8A",
                        ),
                        class: None,
                        id: Some(
                            "body",
                        ),
                        name: "view",
                        props: Some(
                            {
                                PropsKey {
                                    name: "spacing",
                                    is_style: false,
                                    ty: Bind,
                                }: Bind(
                                    "view_space",
                                ),
                            },
                        ),
                        prop_ptr: NoProps,
                        callbacks: None,
                        event_ptr: None,
                        inherits: None,
                        root: false,
                        children: Some(
                            [
                                TemplateModel {
                                    special: Some(
                                        "01HTC6V3PW4DCRD3F0H5QZK8SP",
                                    ),
                                    class: None,
                                    id: Some(
                                        "t_label",
                                    ),
                                    name: "label",
                                    props: Some(
                                        {
                                            PropsKey {
                                                name: "text",
                                                is_style: false,
                                                ty: Bind,
                                            }: Bind(
                                                "label_text",
                                            ),
                                        },
                                    ),
                                    prop_ptr: NoProps,
                                    callbacks: None,
                                    event_ptr: None,
                                    inherits: None,
                                    root: false,
                                    children: None,
                                    parent: None,
                                },
                            ],
                        ),
                        parent: None,
                    },
                ],
            ),
            parent: None,
        },
    ),
    script: Some(
        Script(
            Block {
                brace_token: Brace,
                stmts: [
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
                                    view_space,
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
                                    lit: Lit::Int {
                                        token: 20,
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
                                label_text,
                            ),
                            subpat: None,
                        },
                        init: Some(
                            LocalInit {
                                eq_token: Eq,
                                expr: Expr::Call {
                                    attrs: [],
                                    func: Expr::Path {
                                        attrs: [],
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
                                                PathSep,
                                                PathSegment {
                                                    ident: Ident(
                                                        from,
                                                    ),
                                                    arguments: PathArguments::None,
                                                },
                                            ],
                                        },
                                    },
                                    paren_token: Paren,
                                    args: [
                                        Expr::Lit {
                                            attrs: [],
                                            lit: Lit::Str {
                                                token: "this is a Hello, World!! emoji failed",
                                            },
                                        },
                                    ],
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
                                change_text,
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
                                                Stmt::Expr(
                                                    Expr::Assign {
                                                        attrs: [],
                                                        left: Expr::Path {
                                                            attrs: [],
                                                            qself: None,
                                                            path: Path {
                                                                leading_colon: None,
                                                                segments: [
                                                                    PathSegment {
                                                                        ident: Ident(
                                                                            label_text,
                                                                        ),
                                                                        arguments: PathArguments::None,
                                                                    },
                                                                ],
                                                            },
                                                        },
                                                        eq_token: Eq,
                                                        right: Expr::Call {
                                                            attrs: [],
                                                            func: Expr::Path {
                                                                attrs: [],
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
                                                                        PathSep,
                                                                        PathSegment {
                                                                            ident: Ident(
                                                                                from,
                                                                            ),
                                                                            arguments: PathArguments::None,
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                            paren_token: Paren,
                                                            args: [
                                                                Expr::Lit {
                                                                    attrs: [],
                                                                    lit: Lit::Str {
                                                                        token: "I have been clicked!",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                    },
                                                    Some(
                                                        Semi,
                                                    ),
                                                ),
                                            ],
                                        },
                                    },
                                },
                                diverge: None,
                            },
                        ),
                        semi_token: Semi,
                    },
                ],
            },
        ),
    ),
    style: Some(
        {
            "body": {
                PropsKey {
                    name: "align",
                    is_style: true,
                    ty: Normal,
                }: UnKnown(
                    "0.5",
                ),
                PropsKey {
                    name: "flow",
                    is_style: true,
                    ty: Normal,
                }: UnKnown(
                    "Down",
                ),
            },
            "ui": {
                PropsKey {
                    name: "height",
                    is_style: true,
                    ty: Normal,
                }: UnKnown(
                    "Fill",
                ),
                PropsKey {
                    name: "width",
                    is_style: true,
                    ty: Normal,
                }: UnKnown(
                    "Fill",
                ),
                PropsKey {
                    name: "background_color",
                    is_style: true,
                    ty: Normal,
                }: UnKnown(
                    "#96CEF8",
                ),
            },
            "t_label": {
                PropsKey {
                    name: "font",
                    is_style: true,
                    ty: Normal,
                }: UnKnown(
                    "\"crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf\"",
                ),
                PropsKey {
                    name: "font_size",
                    is_style: true,
                    ty: Normal,
                }: UnKnown(
                    "32.0",
                ),
                PropsKey {
                    name: "brightness",
                    is_style: true,
                    ty: Normal,
                }: UnKnown(
                    "1.1",
                ),
                PropsKey {
                    name: "color",
                    is_style: true,
                    ty: Normal,
                }: UnKnown(
                    "#fff",
                ),
                PropsKey {
                    name: "wrap",
                    is_style: true,
                    ty: Normal,
                }: UnKnown(
                    "Word",
                ),
            },
        },
    ),
    compile: false,
}
```