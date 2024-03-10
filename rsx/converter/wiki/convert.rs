use makepad_widgets::*;
live_design!{
import makepad_widgets::base::*;
import makepad_widgets::theme_desktop_dark::*;
App = {{App}}{ ui: <Window>{block_signal_event: true, align: RightWrap, visible: false, grab_key_focus: false, } }
}
app_main!(App);
