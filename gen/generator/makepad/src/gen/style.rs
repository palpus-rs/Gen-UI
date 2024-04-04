use gen_converter::model::prop::ConvertStyle;

/// 在Makepad中没有像Vue的style部分，所以这里的style策略器只是一个空实现
pub fn style()->impl FnMut(&mut ConvertStyle) -> (){
    return |_style| {};
}