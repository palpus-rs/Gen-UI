use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*; 
    import makepad_draw::shader::std::*;
    import gen_components::components::*;

    GShaderExample = <ScrollYView>{
        height: 200.0,
        width: Fill,
        spacing: 10.0,
        flow: Down,
        <GLabel>{
            text: "GShader"
        }
        <GShader>{
            height: 200.0,
            width: 200.0,
            draw_shader:{
                fn pixel(self) -> vec4 {
                                
                    let loading_size = vec2(64.0);
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let loading_dot_size = vec2(loading_size.x * 0.2 * 0.96);
                    let rotate_time = self.time * 0.05;
                    let counter = 0.0;
                    // draw 16 dots around as a loading animation
                    for i in 0..16{
                        // each dot is a circle and we place it around the circle, with a bit of spacing
                        // there are 16 dots so angle is 0.125PI
                        let angle = 0.125 * 3.1415926;
                        // now we use time to rotate the dots
                        // let dot_pos = vec2(
                        //     self.rect_size.x * 0.5 - cos(angle * counter) * loading_size.x * 0.5,
                        //     self.rect_size.y * 0.5 - sin(angle * counter) * loading_size.y * 0.5
                        // );
                        let dot_pos = vec2(
                            self.rect_size.x * 0.5 - cos(angle * counter + rotate_time) * loading_size.x * 0.5,
                            self.rect_size.y * 0.5 - sin(angle * counter + rotate_time) * loading_size.y * 0.5
                        );
                        
                        sdf.circle(dot_pos.x, dot_pos.y, loading_dot_size.x * 0.5);
                        sdf.fill(#FF0000 + vec4(counter* 0.05, counter* 0.05, counter* 0.05, -0.0525 * counter));
                        counter += 1.0;
                    }
                    
                    return sdf.result;
                }
            }
        }
        <GShader>{
            height: 200.0,
            width: 200.0,
            draw_shader:{
                fn pixel(self) -> vec4 {
                                
                    let uv = self.pos - 0.5;
                    uv.x *= self.rect_size.x / self.rect_size.y;

                    let radius = length(uv);
                    let wave = sin(radius * 10.0 - self.time * 2.0);
                    let intensity = wave * 0.5 + 0.5;
                    let col = vec3(intensity);

                    return vec4(col, 1.0);
                }
            }
        }
        <GShader>{
            height: 200.0,
            width: 200.0,
            draw_shader:{
                fn pixel(self) -> vec4 {
                                
                    let uv = self.pos - 0.5;
                    
                    let time = self.time * 0.5;

                    let col = vec3(0.0);
                    let noise = fract(sin(dot(uv, vec2(12.9898, 78.233))) * 43758.5453);
                    
                    col += 0.1 * vec3(noise, noise* 0.5, noise * 0.2);

                    let r = length(uv);
                    let a = atan(uv.y, uv.x);
                    let f = 0.5 + 0.5 * sin(6.0 * (a + time) + r * 10.0);
                    col += vec3(f* 0.3, f* 0.2, f* 0.5);
                    let i = 0;
                    for _i in 0..10 {
                        // let x = sin(float(i)) * 0.1 + time;
                        // let star_posi = vec2(fract( * 0.1 + time), fract(sin(float(i) * 23421.6313) * 0.1 + time));
                        let star_pos = vec2(fract(sin(float(i) * 43758.5453) * 0.1 + time), fract(sin(float(i) * 23421.6313) * 0.1 + time));
                        star_pos = star_pos * 2.0 - 1.0;
                        star_pos.x *= uv.x / uv.y;
                        let d = uv - star_pos;
                        let star = 1.0 / length(d) * 0.05;
                        col += vec3(star);
                        i = i + 1;
                    }
                    let flicker = fract(sin(dot(uv.xy + time * 20.0, vec2(12.9898,78.233))) * 43758.5453);
                    col += 0.1 * vec3(flicker, flicker * 0.5, flicker * 0.2);

                    let plasma = sin(uv.x * 10.0 + time * 2.0) * cos(uv.y * 10.0 + time * 2.0); 
                    col += vec3(0.2, 0.1, 0.3) * plasma;
                    
                    let morph = sin(time + r * 10.0) * 0.5 + 0.5;
                    col *= vec3(morph, morph * 0.8, morph * 1.2);

                    return vec4(col, 1.0);
                }
            }
        }
    }
}