#version 100

varying lowp vec2 texcoords;
uniform sampler2D tex;

void main() {
    gl_FragColor = texture2D(tex, texcoords);
    // gl_FragColor = vec4(0., 1., 1., 0.);
}