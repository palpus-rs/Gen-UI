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
  text: 'Hello, World!';
  padding: 16;
  align: 0.5;
  /* all walk is supported */
  /* draw_text */
  font_size: 20;
  brightness: 1.1;
  wrap: Word;
  curve: 0.5;
  height_factor: 1.3;
  line_spacing: 1.5;
  top_drop: 0.5;
  font: 'crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf';
  color: #fff;
}
```

```
        <template>
            <window id="ui" >
                <view id="body" >
                    <button id="btn1"  @clicked="change_text"></button>
                    <label id="t_label" :text="label_text"/>
                </view>
            </window>
        </template>

        <script>
        let mut label_text = String::from("this is a Hello, World!! emoji failed");

        let mut change_text = || {
            label_text = String::from("I have been clicked!");
        };
        </script>

        <style>
        #ui{
            #body{
               #t_label{
                    font: "crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf";
               }
            }
        }
        </style>
```
