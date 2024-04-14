use proc_macro2::TokenStream;


#[derive(Debug,Default,Clone)]
pub struct MatchEvent{
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