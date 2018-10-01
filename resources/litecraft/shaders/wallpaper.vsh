#version 140

uniform mat4 persp_matrix;
uniform mat4 view_matrix;
uniform mat4 transform;

in vec2 position;
in vec2 tex_coords;

out vec2 v_tex_coords;

void main() {
    v_tex_coords = tex_coords;
    gl_Position = persp_matrix * view_matrix * transform * vec4(position, 0.0, 1.0);
}