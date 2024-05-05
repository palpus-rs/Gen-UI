use gen_converter::model::script::{LifeTime, PropFn};
use gen_utils::common::token_stream_to_tree;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::StmtMacro;

use crate::utils::apply_over_and_redraw;

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct MatchEvent {
    /// the bind props which can be changed in the app main
    /// this will be set on handle startup
    global: Option<Vec<PropFn>>,
    startup: Option<TokenStream>,
    shutdown: Option<TokenStream>,
    foreground: Option<TokenStream>,
    background: Option<TokenStream>,
    pause: Option<TokenStream>,
    resume: Option<TokenStream>,
    app_got_focus: Option<TokenStream>,
    app_lost_focus: Option<TokenStream>,
    next_frame: Option<TokenStream>,
    action: Option<TokenStream>,
    actions: Option<TokenStream>,
    signal: Option<TokenStream>,
    audio_devices: Option<TokenStream>,
    midi_ports: Option<TokenStream>,
    video_inputs: Option<TokenStream>,
    http_response: Option<TokenStream>,
    http_request_error: Option<TokenStream>,
    http_progress: Option<TokenStream>,
    network_responses: Option<TokenStream>,
    draw: Option<TokenStream>,
    timer: Option<TokenStream>,
    draw_2d: Option<TokenStream>,
    key_down: Option<TokenStream>,
    key_up: Option<TokenStream>,
    back_pressed: Option<TokenStream>,
    match_event: Option<TokenStream>,
    match_event_with_draw_2d: Option<TokenStream>,
}

impl MatchEvent {
    pub fn handle_actions(&mut self, root_id: &str, actions: Vec<PropFn>) -> &mut Self {
        let mut tk = TokenStream::new();
        for item in actions {
            let PropFn {
                widget, id, code, ..
            } = item;

            tk.extend(apply_over_and_redraw(
                Some(root_id.to_string()),
                &widget,
                &id,
                token_stream_to_tree(code.to_token_stream()),
            ));
        }

        self.actions.replace(quote! {
            fn handle_actions(&mut self, cx: &mut Cx, actions:&Actions){

            }
        });
        self
    }

    pub fn handle_lifetime(
        &mut self,
        root_id: &str,
        binds: Option<Vec<PropFn>>,
        lifetimes: Option<LifeTime>,
    ) -> &mut Self {
        if let Some(lifetimes) = lifetimes {
            let LifeTime { startup, shutdown } = lifetimes;

            self.handle_startup(root_id, binds, startup)
                .handle_shutdown(shutdown);
        }
        self
    }
    pub fn handle_startup(
        &mut self,
        root_id: &str,
        binds: Option<Vec<PropFn>>,
        startup: Option<StmtMacro>,
    ) -> &mut Self {
        if let Some(startup) = startup {
            let mut tk = startup.mac.tokens;
            if let Some(bind_tks) = &binds {
                for item in bind_tks {
                    let PropFn {
                        widget, id, code, ..
                    } = item;

                    tk.extend(apply_over_and_redraw(
                        Some(root_id.to_string()),
                        widget,
                        id,
                        token_stream_to_tree(code.to_token_stream()),
                    ));
                }
            }

            self.startup.replace(quote! {
                fn handle_startup(&mut self, cx: &mut Cx) {
                    #tk
                }
            });
        }
        self.global = binds;
        self
    }
    pub fn handle_shutdown(&mut self, shutdown: Option<StmtMacro>) -> &mut Self {
        if let Some(shutdown) = shutdown {
            self.shutdown.replace(quote! {
                fn handle_shutdown(&mut self, _cx: &mut Cx){
                    #shutdown.mac.tokens
                }
            });
        }
        self
    }
}
