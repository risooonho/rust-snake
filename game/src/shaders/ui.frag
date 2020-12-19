#version 100

uniform sampler2D u_sampler;
precision highp float;

varying vec2 v_tc;
varying vec4 v_color;

void main() {
    gl_FragColor = v_color;
    gl_FragColor.a *= texture2D(u_sampler, v_tc).g;
}