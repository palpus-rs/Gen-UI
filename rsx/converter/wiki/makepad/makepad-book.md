### 覆盖样式

```rust
let view  = self.ui.view(id!(body));
view.apply_over_and_redraw(cx, live!{
    draw_bg: {color: #ddd},
    height: 100,
});
```


## 注册组件

### 创建组件

```rust
#[derive(Live, Widget)]
pub struct MyWidget {
    
}
```