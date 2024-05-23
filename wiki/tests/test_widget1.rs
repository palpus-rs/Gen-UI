/// 测试直接继承的Component
fn test_widget() {
    let input = r#"
    <template>
        <component inherits="view">
            <label class="t_label" font_size="32" text="label 1"/>
            <label id="second_lb" class="t_label" :font_size="fs"  text="label 2"/>
            <button id="bb" text="text btn" @clicked="btn_click" />
        </component>
    </template>
    
    <script>
    let fs: f64 = 18.0;
    let mut btn_click = ||{
        log!("Button bb Clicked");
    };
    </script>
    
    <style>
    .t_label{
        brightness: 1.1;
        color: #fff;
        wrap: Word;
        font: "crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf";
    }
    </style>
    "#;
    let t = Instant::now();
    let ast = ParseResult::try_from(ParseTarget::try_from(input).unwrap()).unwrap();
    let name = snake_to_camel("easy_widget");
    let result = MakepadConverter::convert(&ast, &name);
    // let result = MakepadConvertResult::new(true, "easy_widget", ast);
    dbg!(t.elapsed());

    // dbg!(result.to_string());
    // E:/Rust/learn/makepad/makepad-rik/examples/simple/src/app.rs
    // /Users/user/Workspace/others/beyond-framework/rsx/converter/wiki/widget.rs
    // E:/Rust/try/makepad/Gen-UI/rsx/converter/wiki/widget.rs
    let mut f = File::create(
        "E:/Rust/try/makepad/Gen-UI/rsx/converter/wiki/widget.rs",
    )
    .unwrap();
    let _ = f.write(result.to_string().as_bytes());
}