#version 140

in vec2 v_tex_coords;

out vec4 f_color;

uniform sampler2D tex;
uniform float time;

mat2 scale(vec2 _scale) { return mat2(_scale.x, 0.0, 0.0, _scale.y); }

void main() {
  vec2 st = v_tex_coords.xy;
  vec2 translate = vec2(cos(time * 0.05), sin(time * 0.05));

  st += translate * 0.252;

  // move space from the center to the vec2(0.0)
  st -= vec2(0.5);

  st *= scale(vec2(0.5));

  // move it back to the original place
  st += vec2(0.5);

  f_color = texture(tex, st);
}