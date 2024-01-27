use app_style::app_style;

let mut counter:usize = 0

let handle_actions:FnOnce()->() = || {
    counter += 1;
}

let app = instance!{
    name: "App",
    template: r#"
    <template class="app">
        <window class="ui">
            <view class="body">
                <button value="Hello world" class="button1" @clicked="handle_actions"/>
                <text-input value="Click to count" class="input1"/>
                <label :value="`Counter: ${counter}`" class="label1"/>
            </view>
        </window>
    </template>"#
}