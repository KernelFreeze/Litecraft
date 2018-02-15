#version 330 core

layout(location = 0) in vec2 aPos;
layout(location = 1) in vec2 aTexCoord;

uniform mat4 uTransform;
uniform mat4 uProjection;

out vec2 vTexCoord;

void main() {
  vTexCoord = aTexCoord.xy;
  gl_Position = uProjection * uTransform * vec4(aPos.xy, 0.0, 1.0);
}