#ifdef GL_ES
precision mediump float;
#endif

uniform vec2 u_resolution;

void main() {

    vec2 pos = gl_FragCoord.xy/u_resolution;

    vec3 color = vec3(0.023529412,0.68235296,0.83137256);

    gl_FragColor = vec4(color, 1.0);
}