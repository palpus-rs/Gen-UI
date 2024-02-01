# HTML Parser

HTML-LIKE：
与HTML相似，但语法经过处理

所有的被处理的被认为是不必要的，可能使得模版不够专注！

1. 不允许直接书写字符串
2. 禁止使用模版语法，使用Rust `format!`进行值绑定
3. 标签上声明基本属性只能使用基本类型，复杂类型需要绑定
4. 函数体不允许直接书写在属性中，使用函数绑定
5. 属性类型具有强类型指向型

<img src="../README/imgs/template_design.png" />

## origin
``` html
        //! app.rsx
        <template class="app">
            // this is a window
            <window class="ui">
                <view class="body">
                    /// button componet
                    <button value="Hello world" class="button1" @clicked="handle_actions"/>
                    <text-input value="Click to count" class="input1"/>
                    <label :value="counter" class="label1"/>
                </view>
            </window>
        </template>
```
## AST
``` rust
[src/lib/template/parser.rs:273] res = (
    "",
    [
        TemplateASTNode {
            node_type: Comment(
                File,
            ),
            tag_name: None,
            comment: Some(
                " app.rsx",
            ),
            properties: None,
            children: None,
            parent: None,
        },
        TemplateASTNode {
            node_type: Tag,
            tag_name: Some(
                "template",
            ),
            comment: None,
            properties: Some(
                {
                    "class": String(
                        "app",
                    ),
                },
            ),
            children: Some(
                [
                    TemplateASTNode {
                        node_type: Comment(
                            Normal,
                        ),
                        tag_name: None,
                        comment: Some(
                            " this is a window",
                        ),
                        properties: None,
                        children: None,
                        parent: Some(
                            TemplateASTNode {
                                node_type: Tag,
                                tag_name: Some(
                                    "template",
                                ),
                                comment: None,
                                properties: Some(
                                    {
                                        "class": String(
                                            "app",
                                        ),
                                    },
                                ),
                                children: None,
                                parent: None,
                            },
                        ),
                    },
                    TemplateASTNode {
                        node_type: Tag,
                        tag_name: Some(
                            "window",
                        ),
                        comment: None,
                        properties: Some(
                            {
                                "class": String(
                                    "ui",
                                ),
                            },
                        ),
                        children: Some(
                            [
                                TemplateASTNode {
                                    node_type: Tag,
                                    tag_name: Some(
                                        "view",
                                    ),
                                    comment: None,
                                    properties: Some(
                                        {
                                            "class": String(
                                                "body",
                                            ),
                                        },
                                    ),
                                    children: Some(
                                        [
                                            TemplateASTNode {
                                                node_type: Comment(
                                                    Document,
                                                ),
                                                tag_name: None,
                                                comment: Some(
                                                    " button componet",
                                                ),
                                                properties: None,
                                                children: None,
                                                parent: Some(
                                                    TemplateASTNode {
                                                        node_type: Tag,
                                                        tag_name: Some(
                                                            "view",
                                                        ),
                                                        comment: None,
                                                        properties: Some(
                                                            {
                                                                "class": String(
                                                                    "body",
                                                                ),
                                                            },
                                                        ),
                                                        children: None,
                                                        parent: None,
                                                    },
                                                ),
                                            },
                                            TemplateASTNode {
                                                node_type: Tag,
                                                tag_name: Some(
                                                    "button",
                                                ),
                                                comment: None,
                                                properties: Some(
                                                    {
                                                        "clicked": Function(
                                                            "handle_actions",
                                                        ),
                                                        "value": String(
                                                            "Hello world",
                                                        ),
                                                        "class": String(
                                                            "button1",
                                                        ),
                                                    },
                                                ),
                                                children: None,
                                                parent: Some(
                                                    TemplateASTNode {
                                                        node_type: Tag,
                                                        tag_name: Some(
                                                            "view",
                                                        ),
                                                        comment: None,
                                                        properties: Some(
                                                            {
                                                                "class": String(
                                                                    "body",
                                                                ),
                                                            },
                                                        ),
                                                        children: None,
                                                        parent: None,
                                                    },
                                                ),
                                            },
                                            TemplateASTNode {
                                                node_type: Tag,
                                                tag_name: Some(
                                                    "text-input",
                                                ),
                                                comment: None,
                                                properties: Some(
                                                    {
                                                        "value": String(
                                                            "Click to count",
                                                        ),
                                                        "class": String(
                                                            "input1",
                                                        ),
                                                    },
                                                ),
                                                children: None,
                                                parent: Some(
                                                    TemplateASTNode {
                                                        node_type: Tag,
                                                        tag_name: Some(
                                                            "view",
                                                        ),
                                                        comment: None,
                                                        properties: Some(
                                                            {
                                                                "class": String(
                                                                    "body",
                                                                ),
                                                            },
                                                        ),
                                                        children: None,
                                                        parent: None,
                                                    },
                                                ),
                                            },
                                            TemplateASTNode {
                                                node_type: Tag,
                                                tag_name: Some(
                                                    "label",
                                                ),
                                                comment: None,
                                                properties: Some(
                                                    {
                                                        "class": String(
                                                            "label1",
                                                        ),
                                                        "value": Bind(
                                                            "counter",
                                                        ),
                                                    },
                                                ),
                                                children: None,
                                                parent: Some(
                                                    TemplateASTNode {
                                                        node_type: Tag,
                                                        tag_name: Some(
                                                            "view",
                                                        ),
                                                        comment: None,
                                                        properties: Some(
                                                            {
                                                                "class": String(
                                                                    "body",
                                                                ),
                                                            },
                                                        ),
                                                        children: None,
                                                        parent: None,
                                                    },
                                                ),
                                            },
                                        ],
                                    ),
                                    parent: Some(
                                        TemplateASTNode {
                                            node_type: Tag,
                                            tag_name: Some(
                                                "window",
                                            ),
                                            comment: None,
                                            properties: Some(
                                                {
                                                    "class": String(
                                                        "ui",
                                                    ),
                                                },
                                            ),
                                            children: None,
                                            parent: None,
                                        },
                                    ),
                                },
                            ],
                        ),
                        parent: Some(
                            TemplateASTNode {
                                node_type: Tag,
                                tag_name: Some(
                                    "template",
                                ),
                                comment: None,
                                properties: Some(
                                    {
                                        "class": String(
                                            "app",
                                        ),
                                    },
                                ),
                                children: None,
                                parent: None,
                            },
                        ),
                    },
                ],
            ),
            parent: None,
        },
    ],
) 
```
