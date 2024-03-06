# Parse Target

## rsx

```vue
<template>
  <window class="ui">
    <view class="body">
      /// button componet
      <button value="Hello world" class="button1" @clicked="handle_actions" />
      <text-input value="Click to count" class="input1" />
      <label :value="counter" class="label1" />
    </view>
  </window>
</template>

<script>
let mut counter:usize = 0_usize;

let mut click = ||{
    counter += 1;
};
</script>

<style>
.app {
  .ui {
    height: fill;
    width: fill;
    show_bg: true;
    background_color: linear_gradient(180deg, #7, #3);
    .body {
      flow: down;
      spacing: 20;
      align: 0.5 0.5;
      .button1 {
      }
      .input1 {
        height: 30;
        width: 100;
      }
      .label1 {
        color: #ffffff;
      }
    }
  }
}
</style>
```

## result

```rust
[parser/src/ast/result.rs:258] target = ParseTarget {
    core: ParseCore {
        template: Some(
            "<window class=\"ui\">\n                <view class=\"body\">\n                    /// button componet\n                    <button value=\"Hello world\" class=\"button1\" @clicked=\"handle_actions\" />\n                    <text-input value=\"Click to count\" class=\"input1\" />\n                    <label :value=\"counter\" class=\"label1\" />\n                </view>\n            </window>\n        ",
        ),
        script: Some(
            "let mut counter:usize = 0_usize;\n\n        let mut click = ||{\n            counter += 1;\n        };\n        ",
        ),
        style: Some(
            ".app {\n            .ui {\n              height: fill;\n              width: fill;\n              show_bg: true;\n              background_color: linear_gradient(180deg, #7, #3);\n              .body {\n                flow: down;\n                spacing: 20;\n                align: 0.5 0.5;\n                .button1 {\n                }\n                .input1 {\n                  height: 30;\n                  width: 100;\n                }\n                .label1 {\n                  color: #ffffff;\n                }\n              }\n            }\n          }\n        ",
        ),
    },
    comment: None,
}
```

