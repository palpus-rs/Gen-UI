<template>
    <window class="ui">
        <button class="my_button" text="Hello, World" @clicked="btn_click"></button>
    </window>
</template>

<script>
let mut btn_click = || {
    println!("CLICKED!");
}
</script>