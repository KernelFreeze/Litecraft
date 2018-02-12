#version 330 core

#define PI 3.14159265359

out vec4 FragColor;

in vec2 vTexCoord;

uniform sampler2D uTexture;
uniform float uTime;
uniform vec2 uResolution;

mat2 rotate2d(float _angle){
    return mat2(cos(_angle),-sin(_angle),
                sin(_angle),cos(_angle));
}

mat2 scale(vec2 _scale) {
    return mat2(_scale.x, 0.0, 0.0, _scale.y);
}

void main(){
    vec2 st = vTexCoord;

    // move space from the center to the vec2(0.0)
    st -= vec2(0.5);
    // rotate the space
    st = rotate2d(sin(uTime) * PI) * scale(vec2(sin(uTime) + 1.5)) * st * vec2(3.0);
    // move it back to the original place
    st += vec2(0.5);

    FragColor = texture(uTexture, st);
}