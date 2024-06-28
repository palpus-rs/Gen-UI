use gen_parser::common::{Hex, LinearGradient, RadialGradient};
use gen_utils::common::ident;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_str;

/// convert hex to pixel
pub fn hex_to_pixel(value: &str) -> TokenStream {
    let color = parse_str::<TokenStream>(value).unwrap();
    quote! {
        fn pixel(self) -> vec4{
            return #color;
        }
    }
}

pub fn mix_color_to_token(mix_colors: Vec<((String, String), (String, String))>) -> TokenStream {
    fn nested_mix(codes: &Vec<((String, String), (String, String))>, index: usize) -> TokenStream {
        if index >= codes.len() - 1 {
            // 最后一个颜色段，不需要再嵌套
            let ((color, next_color), (stop, next_stop)) = &codes[index];
            return quote! {
                mix(#color, #next_color, smoothstep(#stop, #next_stop, factor))
            };
        } else {
            // 递归生成嵌套的mix调用
            let ((color, next_color), (stop, next_stop)) = &codes[index];
            let next_mix = nested_mix(codes, index + 1);
            return quote! {
                mix(
                    #color,
                    #next_color,
                    smoothstep(#stop, #next_stop, factor),
                    #next_mix
                )
            };
        }
    }

    nested_mix(&mix_colors, 0)
}

pub fn get_color(color: &Hex) -> TokenStream {
    quote! {
        fn get_color(self) -> vec4{
            return #color;
        }
    }
}

pub fn draw_radial_gradient(value: &RadialGradient, fn_name: &str) -> TokenStream {
    let fn_name = ident(fn_name);
    let RadialGradient { colors } = value;

    let mut draw_color_tk = TokenStream::new();

    for (index, (hex, percentage)) in colors.iter().enumerate() {
        let color_ident = format!("color{}", index);
        let percentage_ident = format!("stop{}", index);
        draw_color_tk.extend(quote! {
            let #color_ident = #hex;
            let #percentage_ident = #percentage;
        });
    }

    let mut mix_colors = Vec::new();

    for i in 0..colors.len() - 1 {
        let ident1 = format!("color{}", i);
        let ident2 = format!("color{}", i + 1);

        let stop1 = format!("stop{}", i);
        let stop2 = format!("stop{}", i + 1);

        mix_colors.push(((ident1, ident2), (stop1, stop2)));
    }

    let mix_colors_tk = mix_color_to_token(mix_colors);

    quote! {
        fn #fn_name(self) -> vec4{
            let center = vec2(0.5, 0.5);
            let distance = distance(self.pos, center);
            let factor = clamp(distance, 0.0, 1.0);

            #draw_color_tk

            return #mix_colors_tk;
        }
    }
}

/// draw linear gradient use glsl code
/// - value: &LinearGradient
/// - fn_name: &str (function name)
pub fn draw_linear_gradient(value: &LinearGradient, fn_name: &str) -> TokenStream {
    let fn_name = ident(fn_name);
    let LinearGradient { angle, colors } = value;

    let mut draw_color_tk = TokenStream::new();

    for (index, (hex, percentage)) in colors.iter().enumerate() {
        let color_ident = format!("color{}", index);
        let percentage_ident = format!("stop{}", index);
        draw_color_tk.extend(quote! {
            let #color_ident = #hex;
            let #percentage_ident = #percentage;
        });
    }

    let mut mix_colors = Vec::new();

    for i in 0..colors.len() - 1 {
        let ident1 = format!("color{}", i);
        let ident2 = format!("color{}", i + 1);

        let stop1 = format!("stop{}", i);
        let stop2 = format!("stop{}", i + 1);

        mix_colors.push(((ident1, ident2), (stop1, stop2)));
    }

    let mix_colors_tk = mix_color_to_token(mix_colors);

    quote! {
        fn #fn_name(self) -> vec4{
            let gradient_angle = #angle;
            let direction = vec2(cos(radians(gradient_angle)), sin(radians(gradient_angle)));
            let factor = dot(self.pos, direction);

            #draw_color_tk

            return #mix_colors_tk;
        }
    }
}