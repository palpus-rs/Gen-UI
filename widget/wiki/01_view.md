# View

## emoji

❓：Unknown and need help

❌：mistake

✔️：enum default

## properties

| name                 | type                                                         | default | description                                                  |
| -------------------- | ------------------------------------------------------------ | ------- | ------------------------------------------------------------ |
| `draw_bg`            | `DrawColor`                                                  |         | background for widget                                        |
| `show_bg`            | `bool`                                                       | `false` | show background or not, if this prop is false, draw_bg is useless |
| `layout`             | `Layout`                                                     |         | Layout properties for widget                                 |
| `walk`               | `Walk`                                                       |         | Positioning and size information<br />定位和尺寸信息         |
| `dpi_factor`         | `Option<f64>`                                                |         | ❓                                                            |
| `optimize`           | `ViewOptimize`                                               |         | the types of optimize render                                 |
| `visible`            | `bool`                                                       | true    | Is it visible or not                                         |
| `debug`              | `ViewDebug`                                                  |         | properties that can be debugged                              |
| `event_order`        | `EventOrder`                                                 |         | the order of handling events                                 |
| `grab_key_focus`     | `bool`                                                       | `true`  | should actively grab the keyboard focus<br />是否应该主动抓取键盘焦点 |
| `block_signal_event` | `bool`                                                       | `false` | whether to prevent the propagation of signal events, by default, events will not be passed to any sub views or subsequent event handlers, which can prevent event flooding or restrict event processing under specific conditions.<br />是否阻止信号事件的传播，默认不会将事件传递给任何子视图或后续的事件处理器，这样可以防止事件泛滥或者在特定条件下限制事件的处理。 |
| `cursor`             | `Option<MouseCursor>`                                        | `None`  | mouse cursor<br />鼠标样式                                   |
| `scroll_bars`        | ptr:`Option<LivePtr>`<br />real:`ScrollBars`<br />❓：can you give me an easy example to show how to use scroll in view？ | `None`  | The behavior or appearance of a scrollbar<br />滚动条的行为或外观 |
| `design_mode`        | `bool`                                                       | `false` | Is it in design mode? Design mode is usually used during the development phase, allowing developers to adjust and preview the layout and style of components at runtime without the need to recompile code.<br />是否处于设计模式，设计模式通常用于开发阶段，让开发者可以在运行时调整和预览组件的布局和样式，而不需要重新编译代码。 |
| `animator`           | `Animator`                                                   |         |                                                              |

## property type

### DrawColor

| field | type | description      |
| ----- | ---- | ---------------- |
| color | Vec4 | background color |

#### impl

```rust
fn pixel(self) -> vec4
```

#### example

##### normal

```rust
draw_bg: {
	fn pixel(self) -> vec4 {
		return #000;
	}
}
```

##### Linear gradient

```rust
draw_bg: {
	fn pixel(self) -> vec4 {
		// linear gradient from top to bottom
		return mix(#f05d5d, #5f5df0, self.pos.y);
	}
}
```

**❓ Is here has any other examples?**

### Layout

| field        | type    | default | description                                                  |
| ------------ | ------- | ------- | ------------------------------------------------------------ |
| scroll       | DVec2   |         | The offset of scroll scrolling<br />scroll 滚动的偏移量      |
| clip_x       | bool    | true    | Crop horizontally, which can be used to limit the visible area of UI elements within their containers and avoid drawing content beyond the specified area<br />水平方向上进行裁剪，这可以用于限制UI元素在其容器内的可视区域，避免绘制超出指定区域的内容 |
| clip_y       | bool    | true    | Crop vertically, which can be used to limit the visible area of UI elements within their containers and avoid drawing content beyond the specified area<br />垂直方向上进行裁剪，这可以用于限制UI元素在其容器内的可视区域，避免绘制超出指定区域的内容 |
| padding      | Padding |         | padding refers to the space between the boundaries of UI elements and their content<br />内边距，指UI元素的边界与其内容之间的空间 |
| align        | Align   |         | **❓Alignment of sub elements**<br />子元素的对齐方式         |
| flow         | Flow    |         | The flow direction of the layout determines how the sub elements are arranged inside the container<br />布局的流动方向，决定了子元素是如何在容器内部排列的 |
| spacing      | f64     |         | The spacing between sub elements<br />子元素之间的间距       |
| line_spacing | f64     |         | line spacing<br />行间距                                     |

### Walk

| field   | type            | description                                                  |
| ------- | --------------- | ------------------------------------------------------------ |
| abs_pos | `Option<DVec2>` | 绝对位置 <br />absoulte position                             |
| margin  | Margin          | 外边距，围绕在元素边框的空白区域<br />Surround the blank area around the element border |
| width   | Size            | the width of the widget                                      |
| height  | Size            | the height of the widget                                     |

### ViewOptimize

优化渲染的类型

the types of optimize render

| field    | description                                                  |
| -------- | ------------------------------------------------------------ |
| None ✔️   | There is no specific optimization strategy                   |
| DrawList | collecting rendering commands into a drawing list for batch processing, which can reduce the number of drawing calls and improve rendering efficiency |
| Texture  | This may mean caching the view content as a texture, allowing you to directly draw the texture instead of redrawing all the content |

### EventOrder

用于表示事件处理的顺序

Used to indicate the order of event processing

| field               | description                                                  |
| ------------------- | ------------------------------------------------------------ |
| Down                | Indicates that events will be passed in order from top to bottom. Usually, this means that the event is first received by the outermost (or parent) component, and then sequentially passed to the inner (or child) component until a suitable component is found to handle the event<br />表示事件将按照从顶层向下层的顺序传递。通常，这意味着事件首先被最外层（或父级）组件接收，然后依次向内层（或子级）组件传递，直到找到适合处理该事件的组件为止 |
| Up ✔️                | Indicates that events will be passed in the order from lower to top, which is opposite to Down. In this case, the event is first received by the innermost component and then passed on to the outer layer in sequence.<br />表示事件将按照从下层向顶层的顺序传递，即与Down相反。这种情况下，事件首先被最内层的组件接收，然后依次向外层传递。 |
| `List(Vec<LiveId>)` | Provides a more flexible way of event delivery, allowing developers to specify a specific list of event delivery orders<br />提供了一种更灵活的事件传递方式，允许开发者指定一个具体的事件传递顺序列表 |

### MouseCursor

cursor styles of mouse

| Enum Variant | Description                                                  |
| ------------ | ------------------------------------------------------------ |
| Hidden       | Don't show the cursor.                                       |
| Default      | The default cursor, often an arrow.                          |
| Crosshair    | A crosshair cursor.                                          |
| Hand         | A hand cursor, typically used for links.                     |
| Arrow        | An arrow cursor.                                             |
| Move         | A move cursor, indicating movement.                          |
| Text         | A text cursor, indicating text interaction.                  |
| Wait         | A wait cursor, indicating a process is ongoing.              |
| Help         | A help cursor, often represented with a question mark.       |
| NotAllowed   | A not allowed cursor, indicating an action cannot be performed. |
| NResize      | North resize, indicating resizing from the top.              |
| NeResize     | North-East resize, diagonal resizing from the top-right corner. |
| EResize      | East resize, indicating resizing from the right side.        |
| SeResize     | South-East resize, diagonal resizing from the bottom-right corner. |
| SResize      | South resize, indicating resizing from the bottom.           |
| SwResize     | South-West resize, diagonal resizing from the bottom-left corner. |
| WResize      | West resize, indicating resizing from the left side.         |
| NwResize     | North-West resize, diagonal resizing from the top-left corner. |
| NsResize     | North-South resize, indicating vertical resizing.            |
| NeswResize   | North-East to South-West resize, indicating diagonal resizing. |
| EwResize     | East-West resize, indicating horizontal resizing.            |
| NwseResize   | North-West to South-East resize, indicating diagonal resizing. |
| ColResize    | Column resize, indicating horizontal resizing, often used in tables. |
| RowResize    | Row resize, indicating vertical resizing, often used in tables. |

## Makepad Type

### DVec2

二维平面向量，常用于表示位移，含有x，y属性

Two dimensional plane vector, commonly used to represent displacement, containing x and y attributes

| field | type |
| ----- | ---- |
| x     | f64  |
| y     | f64  |

### Padding

| field  | type |
| ------ | ---- |
| left   | f64  |
| right  | f64  |
| top    | f64  |
| bottom | f64  |

### Margin

| field  | type |
| ------ | ---- |
| left   | f64  |
| right  | f64  |
| top    | f64  |
| bottom | f64  |

### Align

| field | type |
| ----- | ---- |
| x     | f64  |
| y     | f64  |

### Flow (enum)

The flow direction of the layout determines how the sub elements are arranged inside the container

| field     | description                                   |
| --------- | --------------------------------------------- |
| Right     | order from right to left                      |
| Down✔️     | order from top to bottom                      |
| Overlay   | All elements overlap in the upper left corner |
| RightWrap | order from right to left, wrap if exceeded    |

### Size

| field  | default | description                                         |
| ------ | ------- | --------------------------------------------------- |
| Fill ✔️ |         | inherits from parent<br />继承父级                  |
| Down   | 200.0   | Fixed size<br />固定的尺寸                          |
| Fit    |         | depend on children (fit content)<br />根据内容确定  |
| All    |         | Adapt to all available spaces<br />适应所有可用空间 |

## mistakes and helps

dpi_factor you can ignore in the docs

scrollbars have an example in theme_desktop_dark ScrollXYView

align is x/y factor so x:1.0 = align right, x:0.5 = align center

as for shader examples, the entire 'theme' is in widgets/src/theme_deskop_darkj

you will find most useful things DSL related there

and for more advanced uses of the DSL you can check the SDXL example or ironfish
