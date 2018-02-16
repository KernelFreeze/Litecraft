#version 330 core

layout(location = 0) in vec3 aPos;
layout(location = 1) in vec2 aTexCoord;
layout(location = 2) in float aTexture;
layout(location = 3) in float aCullface;
layout(location = 4) in float aTint;

uniform mat4 uTransform;
uniform mat4 uProjection;
uniform mat4 uView;

out vec2 vTexCoord;
out float vTexture;
out float vCullface;
out float vTint;

void main() {
  gl_Position = uTransform * uProjection * uView * vec4(aPos, 1.0);

  vTexCoord = vec2(aTexCoord.x, aTexCoord.y);
  vTexture = aTexture;
  vCullface = aCullface;
  vTint = aTint;
}