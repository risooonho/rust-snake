#version 100

attribute vec2 uv;
attribute vec2 pos;

varying lowp vec2 texcoords;


void main() {
    texcoords = uv;
    gl_Position = vec4(pos, 0.0, 1.0);
}