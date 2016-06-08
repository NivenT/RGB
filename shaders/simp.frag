#version 130

uniform sampler2D sampler;

in vec2 fragUV;

out vec4 finalColor;

void main()
{
    finalColor = texture(sampler, fragUV);
}
