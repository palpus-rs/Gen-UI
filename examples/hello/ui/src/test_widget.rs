// use gen_macros::Prop;

// pub fn test_widget1() {
//     #[derive(Prop)]
//     struct TestWidget1 {
//         text: String,
//         num: u32,
//     }
    
//     let mut prop = TestWidget1::default();

    
//     let mut btn_click = || {
//         prop.text = String::from("I have been clicked");
//         println!("Button bb Clicked");
//         // active!(Events::Clicked("Hello".to_string()));
//     };

//     before_render!{
//         println!("Before Render");
//     }

//     after_render!{
//         println!("After Render");
//     }
// }
