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
[parser\src\target\style.rs:207] res.1 = [
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
                                    "background_color": Function(
                                        Function {
                                            name: "linear_gradient",
                                            params: Some(
                                                [
                                                    "180deg",
                                                    "#7",
                                                    "#3",
                                                ],
                                            ),
                                        },
                                    ),
                                    "width": UnKnown(
                                        "fill",
                                    ),
                                    "show_bg": UnKnown(
                                        "true",
                                    ),
                                    "height": UnKnown(
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
                                                    "align": UnKnown(
                                                        "0.5 0.5",
                                                    ),
                                                    "spacing": UnKnown(
                                                        "20",
                                                    ),
                                                    "flow": UnKnown(
                                                        "down",
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
                                                                    "height": UnKnown(
                                                                        "30",
                                                                    ),
                                                                    "width": UnKnown(
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
                                                                    "color": UnKnown(
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
