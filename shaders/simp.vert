#version 130

in vec2 pos;
in vec2 uv;

out vec2 fragUV;

void main() {
    gl_Position = vec4(pos, 1, 1);
	fragUV = uv;
}
