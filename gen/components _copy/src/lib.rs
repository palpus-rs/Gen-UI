pub use makepad_draw::makepad_platform;
pub use makepad_draw;
pub use makepad_html;
pub use makepad_derive_widget;
pub use makepad_draw::*;
pub use makepad_derive_widget::*;
pub use makepad_markdown;


pub mod widgets;
pub mod components;

// pub use crate::widgets::view::*;
pub use crate::widgets::widget_match_event::WidgetMatchEvent;
pub use crate::widgets::widget::{
    WidgetSet,
    WidgetSetIterator,
    WidgetUid,
    DrawStep,
    DrawStepApi,
    CreateAt,
    WidgetCache,
    WidgetActionCxExt,
    WidgetActionsApi,
    WidgetActionTrait,
    WidgetAction,
    WidgetActionCast,
    WidgetActionOptionApi,
    WidgetRef,
    Widget,
    WidgetNode,
    WidgetRegistry,
    WidgetFactory,
    DrawStateWrap,
};


pub fn live_design(cx: &mut Cx){
    makepad_draw::live_design(cx);
    
    // widgets
    crate::widgets::root::live_design(cx);
    crate::widgets::window::live_design(cx);
    crate::widgets::nav_control::live_design(cx);
    crate::widgets::scroll_bar::live_design(cx);
    crate::widgets::scroll_bars::live_design(cx);
    crate::widgets::desktop_button::live_design(cx);
    crate::widgets::button::live_design(cx);
    crate::widgets::label::live_design(cx);
    //       widhets - others (use less)
    crate::widgets::performance_view::live_design(cx);
    crate::widgets::debug_view::live_design(cx);
    // components
    crate::components::live_design(cx);
}