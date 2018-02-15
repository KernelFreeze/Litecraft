#version 330 core

layout(location = 0) in vec2 aPos;
layout(location = 1) in vec2 aTexCoord;
layout(location = 2) in float aTexture;

uniform mat4 uTransform;
uniform mat4 uProjection;

out vec2 vTexCoord;
out float vTexture;

void main() {
  gl_Position = uTransform * uProjection * vec4(aPos, 0.0, 1.0);

  vTexCoord = vec2(aTexCoord.x, aTexCoord.y);
  vTexture = aTexture;
}