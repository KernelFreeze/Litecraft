#version 100

uniform sampler2D tex;

varying vec2 v_tex_coords;

void main() {
    gl_FragColor = texture2D(tex, v_tex_coords);
}