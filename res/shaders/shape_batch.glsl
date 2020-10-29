#shader vertex
#version 330 core

layout(location = 0) in vec2 vert_pos;
layout(location = 1) in int vert_color;

out vec4 color;

void main()
{
	gl_Position = vec4(vert_pos, 1.0, 1.0);
    float a = vert_color & 255;
    float b = (vert_color >> 8) & 255;
    float g = (vert_color >> 16) & 255;
    float r = (vert_color >> 24) & 255;
    color = vec4(r / 255, g / 255, b / 255, a / 255);
    //color = vec4(1.0, 1.0, 1.0, 1.0);
}

#shader fragment
#version 330 core

out vec4 FragColor;
in vec4 color;

void main() {
	FragColor = color;
};