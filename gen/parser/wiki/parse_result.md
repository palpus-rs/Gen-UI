# Parse Result

## test machine 1

cpu: 2.2 GHz 四核 Intel Core i7

rsx:

- 1.332564ms
- 1.203039ms
- 1.496007ms
- 1.229173ms
- 1.207143ms
- 1.125941ms

vue:

- 6.839ms
- 7.19ms
- 6.804ms
- 7.978ms
- 6.977ms
- 7.046ms

## test machine 2

cpu: Intel(R) Core(TM) i5-10200H CPU @ 2.40GHz 2.40 GHz

rsx:

- 1.4491ms
- 1.2271ms
- 1.6149ms
- 1.6475ms
- 1.4695ms

vue:

- 6.196ms
- 6.012ms
- 6.209ms
- 6.319ms
- 6.447ms

## rsx

```vue
<template>
  <window class="ui">
    <view class="body">
      /// button componet
      <button value="Hello world" class="button1" @clicked="handle_click" />
      <text-input value="Click to count" class="input1" />
      <label :value="counter" class="label1" />
    </view>
  </window>
</template>

<script>
let mut counter:usize = 0_usize;

let mut handle_click = ||{
    counter += 1;
};
</script>

<style>
.app {
  .ui {
    height: fill;
    width: fill;
    show_bg: true;
    background_color: linear_gradient(180deg, #7, #3);
    .body {
      flow: down;
      spacing: 20;
      align: 0.5 0.5;
      .button1 {
      }
      .input1 {
        height: 30;
        width: 100;
      }
      .label1 {
        color: #ffffff;
      }
    }
  }
}
</style>
```

## result

```rust
[parser/src/ast/result.rs:260] result = ParseResult {
    template: Some(
        [
            Tag(
                Tag {
                    name: "window",
                    ty: Normal,
                    props: Some(
                        {
                            PropsKey {
                                name: "class",
                                is_style: false,
                                ty: Normal,
                            }: UnKnown(
                                "ui",
                            ),
                        },
                    ),
                    children: Some(
                        [
                            Tag(
                                Tag {
                                    name: "view",
                                    ty: Normal,
                                    props: Some(
                                        {
                                            PropsKey {
                                                name: "class",
                                                is_style: false,
                                                ty: Normal,
                                            }: UnKnown(
                                                "body",
                                            ),
                                        },
                                    ),
                                    children: Some(
                                        [
                                            Comment(
                                                Document(
                                                    "button componet",
                                                ),
                                            ),
                                            Tag(
                                                Tag {
                                                    name: "button",
                                                    ty: SelfClosed,
                                                    props: Some(
                                                        {
                                                            PropsKey {
                                                                name: "class",
                                                                is_style: false,
                                                                ty: Normal,
                                                            }: UnKnown(
                                                                "button1",
                                                            ),
                                                            PropsKey {
                                                                name: "clicked",
                                                                is_style: false,
                                                                ty: Function,
                                                            }: Function(
                                                                Function {
                                                                    name: "handle_click",
                                                                    params: None,
                                                                    is_style: false,
                                                                },
                                                            ),
                                                            PropsKey {
                                                                name: "value",
                                                                is_style: false,
                                                                ty: Normal,
                                                            }: UnKnown(
                                                                "Hello world",
                                                            ),
                                                        },
                                                    ),
                                                    children: None,
                                                    parent: Some(
                                                        Tag(
                                                            Tag {
                                                                name: "view",
                                                                ty: Normal,
                                                                props: Some(
                                                                    {
                                                                        PropsKey {
                                                                            name: "class",
                                                                            is_style: false,
                                                                            ty: Normal,
                                                                        }: UnKnown(
                                                                            "body",
                                                                        ),
                                                                    },
                                                                ),
                                                                children: None,
                                                                parent: None,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                            Tag(
                                                Tag {
                                                    name: "text-input",
                                                    ty: SelfClosed,
                                                    props: Some(
                                                        {
                                                            PropsKey {
                                                                name: "class",
                                                                is_style: false,
                                                                ty: Normal,
                                                            }: UnKnown(
                                                                "input1",
                                                            ),
                                                            PropsKey {
                                                                name: "value",
                                                                is_style: false,
                                                                ty: Normal,
                                                            }: UnKnown(
                                                                "Click to count",
                                                            ),
                                                        },
                                                    ),
                                                    children: None,
                                                    parent: Some(
                                                        Tag(
                                                            Tag {
                                                                name: "view",
                                                                ty: Normal,
                                                                props: Some(
                                                                    {
                                                                        PropsKey {
                                                                            name: "class",
                                                                            is_style: false,
                                                                            ty: Normal,
                                                                        }: UnKnown(
                                                                            "body",
                                                                        ),
                                                                    },
                                                                ),
                                                                children: None,
                                                                parent: None,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                            Tag(
                                                Tag {
                                                    name: "label",
                                                    ty: SelfClosed,
                                                    props: Some(
                                                        {
                                                            PropsKey {
                                                                name: "value",
                                                                is_style: false,
                                                                ty: Bind,
                                                            }: Bind(
                                                                "counter",
                                                            ),
                                                            PropsKey {
                                                                name: "class",
                                                                is_style: false,
                                                                ty: Normal,
                                                            }: UnKnown(
                                                                "label1",
                                                            ),
                                                        },
                                                    ),
                                                    children: None,
                                                    parent: Some(
                                                        Tag(
                                                            Tag {
                                                                name: "view",
                                                                ty: Normal,
                                                                props: Some(
                                                                    {
                                                                        PropsKey {
                                                                            name: "class",
                                                                            is_style: false,
                                                                            ty: Normal,
                                                                        }: UnKnown(
                                                                            "body",
                                                                        ),
                                                                    },
                                                                ),
                                                                children: None,
                                                                parent: None,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ],
                                    ),
                                    parent: Some(
                                        Tag(
                                            Tag {
                                                name: "window",
                                                ty: Normal,
                                                props: Some(
                                                    {
                                                        PropsKey {
                                                            name: "class",
                                                            is_style: false,
                                                            ty: Normal,
                                                        }: UnKnown(
                                                            "ui",
                                                        ),
                                                    },
                                                ),
                                                children: None,
                                                parent: None,
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ],
                    ),
                    parent: None,
                },
            ),
        ],
    ),
    style: Some(
        [
            Style(
                Style {
                    name: "app",
                    ty: Class,
                    props: None,
                    children: Some(
                        [
                            Style(
                                Style {
                                    name: "ui",
                                    ty: Class,
                                    props: Some(
                                        {
                                            PropsKey {
                                                name: "height",
                                                is_style: true,
                                                ty: Normal,
                                            }: UnKnown(
                                                "fill",
                                            ),
                                            PropsKey {
                                                name: "background_color",
                                                is_style: true,
                                                ty: Function,
                                            }: Function(
                                                Function {
                                                    name: "linear_gradient",
                                                    params: Some(
                                                        [
                                                            "180deg",
                                                            "#7",
                                                            "#3",
                                                        ],
                                                    ),
                                                    is_style: true,
                                                },
                                            ),
                                            PropsKey {
                                                name: "show_bg",
                                                is_style: true,
                                                ty: Normal,
                                            }: UnKnown(
                                                "true",
                                            ),
                                            PropsKey {
                                                name: "width",
                                                is_style: true,
                                                ty: Normal,
                                            }: UnKnown(
                                                "fill",
                                            ),
                                        },
                                    ),
                                    children: Some(
                                        [
                                            Style(
                                                Style {
                                                    name: "body",
                                                    ty: Class,
                                                    props: Some(
                                                        {
                                                            PropsKey {
                                                                name: "align",
                                                                is_style: true,
                                                                ty: Normal,
                                                            }: UnKnown(
                                                                "0.5 0.5",
                                                            ),
                                                            PropsKey {
                                                                name: "flow",
                                                                is_style: true,
                                                                ty: Normal,
                                                            }: UnKnown(
                                                                "down",
                                                            ),
                                                            PropsKey {
                                                                name: "spacing",
                                                                is_style: true,
                                                                ty: Normal,
                                                            }: UnKnown(
                                                                "20",
                                                            ),
                                                        },
                                                    ),
                                                    children: Some(
                                                        [
                                                            Style(
                                                                Style {
                                                                    name: "button1",
                                                                    ty: Class,
                                                                    props: None,
                                                                    children: None,
                                                                    parent: Some(
                                                                        Style(
                                                                            Style {
                                                                                name: "body",
                                                                                ty: Class,
                                                                                props: None,
                                                                                children: None,
                                                                                parent: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                            Style(
                                                                Style {
                                                                    name: "input1",
                                                                    ty: Class,
                                                                    props: Some(
                                                                        {
                                                                            PropsKey {
                                                                                name: "height",
                                                                                is_style: true,
                                                                                ty: Normal,
                                                                            }: UnKnown(
                                                                                "30",
                                                                            ),
                                                                            PropsKey {
                                                                                name: "width",
                                                                                is_style: true,
                                                                                ty: Normal,
                                                                            }: UnKnown(
                                                                                "100",
                                                                            ),
                                                                        },
                                                                    ),
                                                                    children: Some(
                                                                        [],
                                                                    ),
                                                                    parent: Some(
                                                                        Style(
                                                                            Style {
                                                                                name: "body",
                                                                                ty: Class,
                                                                                props: None,
                                                                                children: None,
                                                                                parent: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                            Style(
                                                                Style {
                                                                    name: "label1",
                                                                    ty: Class,
                                                                    props: Some(
                                                                        {
                                                                            PropsKey {
                                                                                name: "color",
                                                                                is_style: true,
                                                                                ty: Normal,
                                                                            }: UnKnown(
                                                                                "#ffffff",
                                                                            ),
                                                                        },
                                                                    ),
                                                                    children: Some(
                                                                        [],
                                                                    ),
                                                                    parent: Some(
                                                                        Style(
                                                                            Style {
                                                                                name: "body",
                                                                                ty: Class,
                                                                                props: None,
                                                                                children: None,
                                                                                parent: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        ],
                                                    ),
                                                    parent: Some(
                                                        Style(
                                                            Style {
                                                                name: "ui",
                                                                ty: Class,
                                                                props: None,
                                                                children: None,
                                                                parent: None,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ],
                                    ),
                                    parent: Some(
                                        Style(
                                            Style {
                                                name: "app",
                                                ty: Class,
                                                props: None,
                                                children: None,
                                                parent: None,
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ],
                    ),
                    parent: None,
                },
            ),
        ],
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
                                mutability: Some(
                                    Mut,
                                ),
                                ident: Ident(
                                    counter,
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
                                                usize,
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
                                        token: 0_usize,
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
                                handle_click,
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
                                                    Expr::Binary {
                                                        attrs: [],
                                                        left: Expr::Path {
                                                            attrs: [],
                                                            qself: None,
                                                            path: Path {
                                                                leading_colon: None,
                                                                segments: [
                                                                    PathSegment {
                                                                        ident: Ident(
                                                                            counter,
                                                                        ),
                                                                        arguments: PathArguments::None,
                                                                    },
                                                                ],
                                                            },
                                                        },
                                                        op: BinOp::AddAssign(
                                                            PlusEq,
                                                        ),
                                                        right: Expr::Lit {
                                                            attrs: [],
                                                            lit: Lit::Int {
                                                                token: 1,
                                                            },
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
}
```
