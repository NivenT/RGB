#version 130

uniform mat3 camera;

in vec3 pos;
in vec4 color;
in vec2 uv;

out vec4 fragColor;
out vec2 fragUV;

void main() {
	//pos.z determines depth and is independent of perspective
	vec3 position = camera * vec3(pos.xy, 1);
    gl_Position = vec4(position.xy, pos.z, 1);

	fragColor = color;
	fragUV = uv;
}
