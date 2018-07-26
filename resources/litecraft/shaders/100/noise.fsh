#version 100

float random(in vec2 st) {
  return fract(sin(dot(st.xy, vec2(12.9898, 78.233))) * 43758.5453123);
}

void main() {
  vec2 st = gl_FragCoord.xy;
  vec3 color = vec3(random(st * 1.0)) / 200;
  gl_FragColor = vec4(color, 1.0);
}