#version 100

uniform mat4 persp_matrix;
uniform mat4 view_matrix;

attribute vec2 position;
attribute vec2 tex_coords;

varying vec2 v_tex_coords;

void main() {
    v_tex_coords = tex_coords;
    gl_Position = persp_matrix * view_matrix * vec4(position, 0.0, 1.0);
}