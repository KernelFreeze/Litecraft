#version 140

#define PI 3.14159265359

in vec2 v_tex_coords;

out vec4 f_color;

uniform sampler2D tex;
uniform float time;

mat2 rotate2d(float _angle) {
  return mat2(cos(_angle), -sin(_angle), sin(_angle), cos(_angle));
}

mat2 scale(vec2 _scale) { return mat2(_scale.x, 0.0, 0.0, _scale.y); }

void main() {
  vec2 st = v_tex_coords.xy;

  // move space from the center to the vec2(0.0)
  st -= vec2(0.5);

  // rotate the space
  st *= rotate2d(sin(time) * PI);
  st *= scale(vec2(sin(time) + 1.5) * 3);

  // move it back to the original place
  st += vec2(0.5);

  f_color = texture(tex, st);
}