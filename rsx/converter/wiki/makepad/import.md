# import

## 引入图片

在makepad中可以引入多种图片格式，包括：
- `png` 👍
- `jpg` 👍
- `svg` 👍
- ...

### 目录格式

```                 
   ─┬─── MyProject       
    │                    
    ├─┬──── statics      
    │ │                  
    │ └─────── img1.png  
    │                    
    └────── src           
```

### 图片引入

`self`表示当前项目

```rust
live_design!{
    /// import
    IMG1 = dep("crate://self/statics/img1.png")
}
```

## 引入字体

字体的引入和图片引入基本一样,字体格式包括:
- `ttf` 👍
- `otf`
- ...

### 例子

- 首先对字体进行命名
- 在`font`字段中使用`path`指定字体位置

```rust
live_design!{
    /// import
    TEXT_SUB = {
        font_size: 16.0,
        font: {path: dep("crate://makepad-widgets/resources/GoNotoKurrent-Regular.ttf")}
    }
}
```