# parse style

## Normal

### rsx

```css
.app {
  .ui_ui {
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
```

### result

```rust
[parser/src/target/style.rs:212] res.1 = [
    Style(
        Style {
            name: "app",
            ty: Class,
            props: None,
            children: Some(
                [
                    Style(
                        Style {
                            name: "ui_ui",
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
                                                    PropsKey {
                                                        name: "align",
                                                        is_style: true,
                                                        ty: Normal,
                                                    }: UnKnown(
                                                        "0.5 0.5",
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
                                                                        name: "width",
                                                                        is_style: true,
                                                                        ty: Normal,
                                                                    }: UnKnown(
                                                                        "100",
                                                                    ),
                                                                    PropsKey {
                                                                        name: "height",
                                                                        is_style: true,
                                                                        ty: Normal,
                                                                    }: UnKnown(
                                                                        "30",
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
                                                        name: "ui_ui",
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
]
```
