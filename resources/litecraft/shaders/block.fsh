#version 330 core

out vec4 FragColor;

in vec2 vTexCoord;
in float vTexture;

uniform sampler2D uTextures[10];

// Get appropiate texture requested by OpenGL
vec4 getSampleFromArray(int ndx) {
  vec4 color;

  if (ndx == 0) {
    color = texture(uTextures[0], vTexCoord);
  } else if (ndx == 1) {
    color = texture(uTextures[1], vTexCoord);
  } else if (ndx == 2) {
    color = texture(uTextures[2], vTexCoord);
  } else if (ndx == 3) {
    color = texture(uTextures[3], vTexCoord);
  } else if (ndx == 4) {
    color = texture(uTextures[4], vTexCoord);
  } else if (ndx == 5) {
    color = texture(uTextures[5], vTexCoord);
  } else if (ndx == 6) {
    color = texture(uTextures[6], vTexCoord);
  } else if (ndx == 7) {
    color = texture(uTextures[7], vTexCoord);
  } else if (ndx == 8) {
    color = texture(uTextures[8], vTexCoord);
  } else /*if (ndx == 9)*/ {
    color = texture(uTextures[9], vTexCoord);
  }
  return color;
}

void main() { FragColor = getSampleFromArray(int(vTexture)); }