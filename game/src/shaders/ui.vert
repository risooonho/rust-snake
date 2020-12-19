#version 100

attribute vec2 pos;
attribute vec2 uv;
attribute vec4 color_in;


varying lowp vec2 texcoords;
varying lowp vec4 color;


uniform mat4 model;
uniform mat4 proj_view;


void main() {
    texcoords = uv;
    color = color_in;
    gl_Position = proj_view * model * vec4(pos, 0.0, 1.0);
}