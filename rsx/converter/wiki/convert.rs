use makepad_widgets::*;
live_design!{
import makepad_widgets::base::*;
import makepad_widgets::theme_desktop_dark::*;
App = {{App}}{ ui: <Window>{draw_bg: { color: #000000 }, show_bg: false, width: Fill, spacing: 18, margin: {top: 3, right: 5, bottom: 7, left: 1}, clip_y: false, height: 178.9, padding: {top: 10, right: 16, bottom: 10, left: 16}, line_spacing: 32.9, clip_x: true,  body = <View>{spacing: 18, margin: {top: 3, right: 5, bottom: 7, left: 1}, } } }
}
app_main!(App);
