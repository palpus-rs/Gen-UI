# Import

This section details how to incorporate external resources such as images and fonts into your Makepad projects. Makepad supports various formats, allowing you to enhance the visual aspect of your applications.

## Importing Images

Makepad allows the importation of various image formats to enrich your user interface. Supported formats include:

- `png` 👍
- `jpg` 👍
- `svg` 👍
- and more...

### Directory Structure

To organize your project and its resources effectively, follow a structured directory format. For example:

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

To import an image into your project, refer to it using a path that starts with `self`, indicating the current project. This makes your project references clear and organized.

```rust
live_design!{
    /// Import statement
    IMG1 = dep("crate://self/statics/img1.png") // Using dep() function to import an image from the statics directory
}
```

## Importing Fonts

Importing fonts in Makepad is very similar to importing images. Makepad supports various font formats to allow you to customize the typography of your application. Supported font formats include:

- `ttf` 👍
- `otf`
- and more...

### Example

To use a font in your project, you first name the font and then specify its path in the `font` field of your live design. This example demonstrates how to define and use a custom font style:

- First, assign a name to your font for reference.
- Use the `path` field within the `font` property to specify the font's location.

```rust
live_design!{
    /// Import statement
    TEXT_SUB = {
        font_size: 16.0, // Define the font size
        font: {path: dep("crate://makepad-widgets/resources/GoNotoKurrent-Regular.ttf")} // Specify the font's path
    }
}
```

By following these guidelines, you can effectively manage and utilize external resources such as images and fonts in your Makepad projects, enhancing the visual appeal and user experience of your applications.