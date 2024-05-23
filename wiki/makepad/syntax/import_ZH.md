# Import

本节详细介绍了如何将图像和字体等外部资源整合到Makepad项目中。Makepad支持各种格式，使您能够增强应用程序的视觉效果。

## Importing Images

Makepad允许导入各种图像格式，以丰富您的用户界面。支持的格式包括：

- `png` 👍
- `jpg` 👍
- `svg` 👍
- and more...

### Directory Structure

要有效地组织项目及其资源，请遵循结构化目录格式。例如:

```                 
   ─┬─── MyProject       # Root directory of your project
    │                    
    ├─┬──── statics      # A directory for static resources like images and fonts
    │ │                  
    │ └─────── img1.png  # An example image file within the statics directory
    │                    
    └────── src          # The source directory where your Rust code lives
```

### Image Importation

要将图像导入到项目中，请使用以`self`开头的路径引用该图像，表示当前项目。这使您的项目参考资料清晰有序。

```rust
live_design!{
    /// Import statement
    IMG1 = dep("crate://self/statics/img1.png") // Using dep() function to import an image from the statics directory
}
```

## Importing Fonts

在Makepad中导入字体与导入图像非常相似。Makepad支持各种字体格式，允许您自定义应用程序的排版。支持的字体格式包括：

- `ttf` 👍
- `otf`
- and more...

### Example

要在项目中使用字体，首先命名字体，然后在live design的`font`字段中指定其路径。此示例演示如何定义和使用自定义字体样式：

- 首先，为字体指定一个名称以供参考。
- 使用`font`属性中的`path`字段指定字体的位置。

```rust
live_design!{
    /// Import statement
    TEXT_SUB = {
        font_size: 16.0, // Define the font size
        font: {path: dep("crate://makepad-widgets/resources/GoNotoKurrent-Regular.ttf")} // Specify the font's path
    }
}
```

通过遵循这些准则，您可以有效地管理和利用Makepad项目中的图像和字体等外部资源，增强应用程序的视觉吸引力和用户体验。