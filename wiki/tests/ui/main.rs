//! 假如这是src里的main.rs文件
use gen_macros::{app, target::Target};

fn main(){
    /// 相当于makepad_example_simple::app::app_main()
    app!{
        Target::Makepad,
        "../app.gen"
    }
}