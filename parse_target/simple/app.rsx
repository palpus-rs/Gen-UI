<template class="app">
    <window class="ui">
        <view class="body">
            <button value="Hello world" class="button1" @clicked="handle_actions"/>
            <text-input value="Click to count" class="input1"/>
            <label :value="`Counter: ${counter}`" class="label1"/>
        </view>
    </window>
</template>

<script lang="rs" expose="App">
let mut counter:usize = 0

fn handle_actions:FnOnce()->() = || {
    counter += 1;
}
</script>

<style>
.app{
    .ui{
        height : fill;
        width : fill;
        show-bg : true;
        // mix(#7, #3, self.pos.y)
        background-color : linear-gradient(180deg, #7, #3); 
        .body{
            flow : down;
            spacing : 20;
            align : center center;
            .button1{ }
            .input1{
                height : 30;
                width : 100;
            }
            .label1{
                color : #ffffff;
            }
        }
    }
}
</style>
