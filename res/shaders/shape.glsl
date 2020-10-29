#shader vertex
#version 330 core

layout(location = 0) in vec2 vert_pos;

uniform vec2 u_Trans = vec2(0.0, 0.0);
uniform vec2 u_Scale = vec2(1.0, 1.0);

void main()
{
	gl_Position = vec4(vert_pos * u_Scale + u_Trans, 1.0, 1.0);
}

#shader fragment
#version 330 core

out vec4 FragColor;
uniform vec4 u_Color = vec4(1.0, 1.0, 1.0, 1.0);

void main() {
	FragColor = u_Color;
};