use makepad_widgets :: * ; live_design ! { import makepad_widgets :: base ::*; import makepad_widgets :: theme_desktop_dark ::*; root , App = { { App } } { root : < root > { } } } pub struct App { #[live] pub ui : WidgetRef , }