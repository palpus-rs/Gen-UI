## Window结构体属性

```rust
#[derive(Live, Widget)]
pub struct Window {
    //#[rust] caption_size: DVec2,
    #[live] last_mouse_pos: DVec2,
    #[live] mouse_cursor_size: DVec2,
    #[live] demo: bool,
    #[rust] demo_next_frame: NextFrame,
    #[live] cursor_draw_list: DrawList2d,
    #[live] draw_cursor: DrawQuad,
    #[live] debug_view: DebugView,
    #[live] performance_view: PerformanceView,
    #[live] nav_control: NavControl,
    #[live] window: WindowHandle,
    #[live] stdin_size: DrawColor,
    #[live] overlay: Overlay,
    #[live] main_draw_list: DrawList2d,
    #[live] pass: Pass,
    #[rust(Texture::new(cx))] depth_texture: Texture,
    #[live] hide_caption_on_fullscreen: bool, 
    #[live] show_performance_view: bool,
    #[deref] view: View,
    #[rust] draw_state: DrawStateWrap<DrawState>,
}
```

- `last_mouse_pos`和`mouse_cursor_size`: 这两个属性分别存储了最后一次鼠标位置和鼠标光标的大小，使用`DVec2`数据类型（可能是一个包含两个维度的向量，用于描述二维空间中的点或向量）。

- `demo`: 一个布尔值，可能用于指示是否处于演示模式。

- `demo_next_frame`: `NextFrame`类型，可能用于控制演示模式下的下一帧渲染。

- `cursor_draw_list`: `DrawList2d`类型，可能用于定义在窗口中绘制光标时所需的绘制指令列表。

- `draw_cursor`: `DrawQuad`类型，可能是用于绘制光标的矩形区域。

- `debug_view`和`performance_view`: 分别为`DebugView`和`PerformanceView`类型，用于显示调试信息和性能指标。

- `nav_control`: `NavControl`类型，可能用于处理导航相关的控制逻辑。

- `window`: `WindowHandle`类型，代表窗口的句柄或引用。

- `overlay`: `Overlay`类型，可能用于在窗口上层显示额外的内容或界面元素。

- `main_draw_list`: 类似于`cursor_draw_list`，为`DrawList2d`类型，用于存储主要内容的绘制指令。

- `pass`: `Pass`类型，可能用于描述渲染过程中的一个阶段或步骤。

- `depth_texture`: `Texture`类型，用于存储深度纹理，这在3D渲染或需要深度测试的场景中非常重要。

- `hide_caption_on_fullscreen`和`show_performance_view`: 布尔值，分别控制全屏时是否隐藏标题栏和是否显示性能视图。

- `view`: `View`类型，通过`deref`属性，表示`Window`内部包含一个`View`结构体，可能用于处理窗口内部的视图渲染和布局。

## View结构体属性

```rust
#[derive(Live, LiveRegisterWidget, WidgetRef, WidgetSet)]
pub struct View {
    // draw info per UI element
    #[live]
    pub draw_bg: DrawColor,

    #[live(false)]
    show_bg: bool,

    #[layout]
    layout: Layout,

    #[walk]
    walk: Walk,

    //#[live] use_cache: bool,
    #[live]
    dpi_factor: Option<f64>,

    #[live]
    optimize: ViewOptimize,
    #[live]
    debug: ViewDebug,
    #[live]
    event_order: EventOrder,

    #[live(true)]
    visible: bool,

    #[live(true)]
    grab_key_focus: bool,
    #[live(false)]
    block_signal_event: bool,
    #[live]
    cursor: Option<MouseCursor>,
    #[live]
    scroll_bars: Option<LivePtr>,
    #[live(false)]
    design_mode: bool,

    #[rust]
    find_cache: HashMap<u64, WidgetSet>,

    #[rust]
    scroll_bars_obj: Option<Box<ScrollBars>>,
    #[rust]
    view_size: Option<DVec2>,

    #[rust]
    area: Area,
    #[rust]
    draw_list: Option<DrawList2d>,

    #[rust]
    texture_cache: Option<ViewTextureCache>,
    #[rust]
    defer_walks: Vec<(LiveId, DeferWalk)>,
    #[rust]
    draw_state: DrawStateWrap<DrawState>,
    #[rust]
    children: ComponentMap<LiveId, WidgetRef>,
    #[rust]
    draw_order: Vec<LiveId>,

    #[animator]
    animator: Animator,
}
```


- `draw_bg`: `DrawColor`类型，用于定义背景的绘制颜色。

- `show_bg`: 布尔值，控制是否显示背景。

- `layout`和`walk`: 分别控制布局和遍历行为。

- `dpi_factor`: 可选的`f64`类型，用于定义DPI因子，可能影响渲染的缩放级别。

- `optimize`, `debug`, `event_order`: 控制优化、调试和事件顺序的相关设置。

- `visible`, `grab_key_focus`, `block_signal_event`: 控制可见性、键盘焦点抓取和事件信号阻塞的布尔值。

- `cursor`和`scroll_bars`: 分别用于定义鼠标光标样式和滚动条的设置。

- `find_cache`, `scroll_bars_obj`, `view_size`等: 这些以`rust`标记的属性用于内部逻辑，包括缓存、对象引用和尺寸等。

- `animator`: `Animator`类型，用于动画处理。
