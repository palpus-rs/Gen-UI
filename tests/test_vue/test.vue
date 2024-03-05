<template>
    <div class="ui">
        <div class="body">
            /// button componet
            <button value="Hello world" class="button1" @click="handle_actions" />
            <text-input class="input1" value="Click to count" />
            <label :value="counter" class="label1" />
        </div>
    </div>
</template>


<style lang="css">
.app {}

.ui {
    box-sizing: border-box;
    background-color: linear-gradient(180deg, #777 , #333);
    height: inherit;
    width: inherit;
}

.body {
    display: flex;
    padding: 20px;
    text-align: center;


}

.button1 {}

.input1 {
    width: 100px;
    height: 30px;
}

.label1 {
    color: #ffffff;
}
</style>
