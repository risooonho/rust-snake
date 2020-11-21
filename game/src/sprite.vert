#version 100

attribute vec2 pos;
attribute vec2 uv;


varying lowp vec2 texcoords;


uniform mat4 model;
uniform mat4 projection;


void main() {
    texcoords = uv;
    gl_Position = projection * model *  vec4(pos, 0.0, 1.0);
}