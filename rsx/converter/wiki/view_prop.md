# View widget Props

## rsx

```css
<style>
#ui{
    /* padding: 16; */
    /* padding 1 4 6 8 */
    padding: 10 16;
    /* same as padding */
    margin: 1 3 5 7;
    /* Fit | All | Fill | f64 */
    height: 178.9;
    width: Fill;
    line_spacing: 32.9;
    clip_x: true;
    clip_y: false;
    /* makepad: draw_bg */
    background_color: #000;
    /* makepad: show_bg */
    background_visible: false;
    spacing: 16;
    /* align: 16 32; */
    align: 16;
    /* Right | Down | Overlay | RightWrap */
    flow: Down;
    /* makepad: abs_pos */
    /* absolute_position: 16.5; */
    absolute_position: 16.5 23;
    /* same as absolute_position */
    scroll: 12.6 8.0;
    /* view optimize */
    /* None | Texture | DrawList */
    optimize: Texture;
    /* Up | Down (List wait todo!) */
    event_order: Up;
    visible: false;
    block_signal_event:true;
    grab_key_focus: false;
    /* Hidden | Default | Hand | Move | ... see Makepad::MouseCursor  */
    cursor:Hidden;
}
</style>
```

# label Props

```css
.ui {
}
```
