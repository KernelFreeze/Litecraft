#version 330 core

out vec4 FragColor;

uniform vec2 uResolution;

float random (in vec2 st) {
    return fract(sin(dot(st.xy,
                         vec2(12.9898,78.233)))
                * 43758.5453123);
}

void main() {
    vec2 st = gl_FragCoord.xy / uResolution.xy;
    st.x *= uResolution.x / uResolution.y;
    float r = random(st * 1.0);

    gl_FragColor = vec4(vec3(r) * vec3(0.05), 1.0);
}