# parse template

HTML-LIKE：
与HTML相似，但语法经过处理

所有的被处理的被认为是不必要的，可能使得模版不够专注！

1. 不允许直接书写字符串
2. 禁止使用模版语法，使用Rust `format!`进行值绑定
3. 标签上声明基本属性只能使用基本类型，复杂类型需要绑定
4. 函数体不允许直接书写在属性中，使用函数绑定
5. 属性类型具有强类型指向型

## Normal

### rsx

```html
// this is a window
<window class="ui">
  <view class="body">
    /// button componet
    <button value="Hello world" class="button1" @clicked="handle_actions">
      <div></div>
      <div />
    </button>
    <text-input value="Click to count" class="input1" />
    <label :value="counter" class="label1" />
  </view>
</window>
<text-input value="Click to count" class="input1" />
```

### result

```rust
[
        Comment(
            Normal(
                "this is a window",
            ),
        ),
        Tag(
            Tag {
                name: "window",
                ty: Normal,
                props: Some(
                    {
                        "class": UnKnown(
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
                                        "class": UnKnown(
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
                                                ty: Normal,
                                                props: Some(
                                                    {
                                                        "class": UnKnown(
                                                            "button1",
                                                        ),
                                                        "clicked": Function(
                                                            Function {
                                                                name: "handle_actions",
                                                                params: Some(
                                                                    [
                                                                        "",
                                                                    ],
                                                                ),
                                                            },
                                                        ),
                                                        "value": UnKnown(
                                                            "Hello world",
                                                        ),
                                                    },
                                                ),
                                                children: Some(
                                                    [
                                                        Tag(
                                                            Tag {
                                                                name: "div",
                                                                ty: Normal,
                                                                props: None,
                                                                children: None,
                                                                parent: Some(
                                                                    Tag(
                                                                        Tag {
                                                                            name: "button",
                                                                            ty: Normal,
                                                                            props: Some(
                                                                                {
                                                                                    "class": UnKnown(
                                                                                        "button1",
                                                                                    ),
                                                                                    "clicked": Function(
                                                                                        Function {
                                                                                            name: "handle_actions",
                                                                                            params: Some(
                                                                                                [
                                                                                                    "",
                                                                                                ],
                                                                                            ),
                                                                                        },
                                                                                    ),
                                                                                    "value": UnKnown(
                                                                                        "Hello world",
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
                                                                name: "div",
                                                                ty: SelfClosed,
                                                                props: None,
                                                                children: None,
                                                                parent: Some(
                                                                    Tag(
                                                                        Tag {
                                                                            name: "button",
                                                                            ty: Normal,
                                                                            props: Some(
                                                                                {
                                                                                    "class": UnKnown(
                                                                                        "button1",
                                                                                    ),
                                                                                    "clicked": Function(
                                                                                        Function {
                                                                                            name: "handle_actions",
                                                                                            params: Some(
                                                                                                [
                                                                                                    "",
                                                                                                ],
                                                                                            ),
                                                                                        },
                                                                                    ),
                                                                                    "value": UnKnown(
                                                                                        "Hello world",
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
                                                            name: "view",
                                                            ty: Normal,
                                                            props: Some(
                                                                {
                                                                    "class": UnKnown(
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
                                                        "value": UnKnown(
                                                            "Click to count",
                                                        ),
                                                        "class": UnKnown(
                                                            "input1",
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
                                                                    "class": UnKnown(
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
                                                        "value": Bind(
                                                            "counter",
                                                        ),
                                                        "class": UnKnown(
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
                                                                    "class": UnKnown(
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
                                                    "class": UnKnown(
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
        Tag(
            Tag {
                name: "text-input",
                ty: SelfClosed,
                props: Some(
                    {
                        "value": UnKnown(
                            "Click to count",
                        ),
                        "class": UnKnown(
                            "input1",
                        ),
                    },
                ),
                children: None,
                parent: None,
            },
        ),
    ],
```

## Empty

Parse Error!

```rust
Error(
        ParseError(
            "error parsing template",
        ),
    ),
```

## Bad Template

### not allow value in template

```rust
/// template
<input>xxx</input>

Error(
        TemplateParseRemain(
            "xxx",
        ),
    ),
```
