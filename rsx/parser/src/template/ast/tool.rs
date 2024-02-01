use crate::TEMPLATE;

use super::TemplateASTNode;

/// check ast template node has or not
pub fn ast_check_template(ast: Vec<TemplateASTNode>) -> bool {
    for item in ast {
        // check is tag?
        if item.is_tag() {
            // check template is super node?
            let tag_name = item.get_tag_name().unwrap();
            return tag_name.eq(TEMPLATE);
        } else {
            continue;
        }
    }
    false
}

pub fn check_parse_template(input: &str) -> bool {
    input.is_empty()
}

#[cfg(test)]
mod test_tool {
    use crate::template::parse_template;

    use super::ast_check_template;

    #[test]
    fn check_template() {
        let template = r#"
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
        "#;
        let no_template = r#"
        //! app.rsx
        <label :value="counter" class="label1"/>
        "#;
        let res1 = parse_template(template).unwrap();
        let res2 = parse_template(no_template).unwrap();
        assert!(ast_check_template(res1.1));
        assert!(!ast_check_template(res2.1));
    }
}
