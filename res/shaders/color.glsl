#shader vertex
#version 330 core

layout(location = 0) in vec2 vert_pos;

uniform vec2 u_Trans = vec2(0.0, 0.0);
uniform vec2 u_Scale = vec2(1.0, 1.0);
out vec2 pos;

void main()
{
	gl_Position = vec4(vert_pos * u_Scale + u_Trans, 1.0, 1.0);
	pos = vert_pos;
}

#shader fragment
#version 330 core

out vec4 FragColor;
uniform vec4 u_Color = vec4(0.1, 1.0, 1.0, 1.0);
in vec2 pos;

void main() {
	vec4 color = u_Color;
	color.b *= (pos.x + 1.0) / 2.0;
	color.g *= (pos.y + 1.0) / 2.0;
	FragColor = color;
}