#version 130

uniform sampler2D sampler;

in vec4 fragColor;
in vec2 fragUV;

out vec4 finalColor;

void main()
{
    finalColor = fragColor * texture(sampler, fragUV);
}
