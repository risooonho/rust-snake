#version 100
uniform vec2 u_screen_size;
uniform vec2 u_tex_size;

attribute vec2 a_pos;
attribute vec2 a_tc;
attribute vec4 a_color;

varying vec2 v_tc;
varying vec4 v_color;

void main() {
    gl_Position = vec4(
        2.0 * a_pos.x / u_screen_size.x - 1.0,
        1.0 - 2.0 * a_pos.y / u_screen_size.y,
        0.0,
        1.0);
    
    v_tc = a_tc / u_tex_size;
    v_color = a_color / 255.0;
}