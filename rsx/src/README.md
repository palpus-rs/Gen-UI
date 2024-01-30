# HTML Parser
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
                    <label :value="`Counter: ${counter}`" class="label1"/>
                </view>
            </window>
        </template>
```
## AST
``` rust
[src/lib/template/parser.rs:237] res = (
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
                    PropertyKey {
                        key: "class",
                        key_type: Normal,
                    }: String(
                        "app",
                    ),
                },
            ),
            children: Some(
                [],
            ),
            parent: None,
        },
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
            parent: None,
        },
        TemplateASTNode {
            node_type: Tag,
            tag_name: Some(
                "window",
            ),
            comment: None,
            properties: Some(
                {
                    PropertyKey {
                        key: "class",
                        key_type: Normal,
                    }: String(
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
                                PropertyKey {
                                    key: "class",
                                    key_type: Normal,
                                }: String(
                                    "body",
                                ),
                            },
                        ),
                        children: Some(
                            [],
                        ),
                        parent: None,
                    },
                ],
            ),
            parent: None,
        },
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
            parent: None,
        },
        TemplateASTNode {
            node_type: Tag,
            tag_name: Some(
                "button",
            ),
            comment: None,
            properties: Some(
                {
                    PropertyKey {
                        key: "class",
                        key_type: Normal,
                    }: String(
                        "button1",
                    ),
                    PropertyKey {
                        key: "clicked",
                        key_type: Function,
                    }: Function(
                        "handle_actions",
                    ),
                    PropertyKey {
                        key: "value",
                        key_type: Normal,
                    }: String(
                        "Hello world",
                    ),
                },
            ),
            children: None,
            parent: None,
        },
    ],
)
```
