#version 460
layout(location=0) in vec3 pos;
layout(location=1) in vec3 rgb;

layout(location=0) out vec3 color;

void main() {
	gl_Position = vec4(pos, 1.0);
	color = rgb;
}
