# Var 
 - [x] `let win_pad = String::from("89 16");`
 - [x] `let win_pad:String = String::from("32 24");`
 - [x] `let win_pad:&str = "10 16"; 👍`
 - [x] `let win_pad = "10 16"; 👍`

```rust
#[test]
    fn test_simple() {
        let input = r#"
        <template>
            <window id="ui" :padding="win_pad">
                <view id="body" />
            </window>
        </template>
        
        <script>
        let win_pad = "10 16";
        // let win_pad = 17; 
        // let win_pad = String::from("89 16"); ✅
        // let win_pad:String = String::from("32 24"); ✅
        // let win_pad:&str = "10 16"; ✅ 👍
        // let win_pad = "10 16"; ✅ 👍
        // let win_pad:String = String::from("10 16");
        let view_margin = "32";
        </script>
        
        <style>
        #ui{
            background_visible: true;
            width: Fill;
            height: Fill;
            background_color: #7733ff;
            #body{
                flow: Down;
                spacing: 20;
                align: 0.5 0.5;
            }
        }
        </style>
        "#;
        let t = Instant::now();
        let ast = ParseResult::try_from(ParseTarget::try_from(input).unwrap()).unwrap();
        let result = MakepadConvertResult::new(true, "App", ast);
        dbg!(t.elapsed());
        // dbg!(result);
        //"/Users/user/Downloads/beyond-framework-main/rsx/converter/wiki/convert.rs"
        //
        // E:/Rust/try/makepad/rsx/converter/wiki/convert.rs
        let mut f =
            File::create("/Users/user/Downloads/makepad-rik/examples/single/window_s/src/app.rs")
                .unwrap();
        let _ = f.write(result.to_string().as_bytes());
    }
```