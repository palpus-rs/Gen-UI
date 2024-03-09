use makepad_widgets::*;
live_design!{
import makepad_widgets::base::*;
import makepad_widgets::theme_desktop_dark::*;
App = {{App}}{ ui: <Window>{show_bg: false, draw_bg: { color: #000000 }, width: Fill, margin: {top: 3, right: 5, bottom: 7, left: 1}, spacing: 18, align: {x: 16, y: 0},  body = <View>{margin: {top: 3, right: 5, bottom: 7, left: 1}, spacing: 18, } } }
}
app_main!(App);
