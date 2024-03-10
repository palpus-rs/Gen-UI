use makepad_widgets::*;
live_design!{
import makepad_widgets::base::*;
import makepad_widgets::theme_desktop_dark::*;
App = {{App}}{ ui: <Window>{show_bg: false, width: Fill, draw_bg: { color: #000000 }, spacing: 18, margin: {top: 3, right: 5, bottom: 7, left: 1}, align: {x: 32, y: 0},  body = <View>{spacing: 18, margin: {top: 3, right: 5, bottom: 7, left: 1}, } } }
}
app_main!(App);
