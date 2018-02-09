#version 120

// Input vertex data, different for all executions of this shader.
attribute vec2 position;
attribute vec2 vertexUV;

// Output data ; will be interpolated for each fragment.
varying vec2 UV;

void main() {
	gl_Position = vec4(position, 0, 1);
	
	// UV of the vertex. No special space for this one.
	UV = vertexUV;
}