#shader vertex
#version 330 core

layout(location = 0) in vec2 vert_pos;
layout(location = 1) in vec2 tex_coord;

out vec2 v_TexCoord;

uniform vec2 u_Trans = vec2(0.0, 0.0);
uniform vec2 u_Scale = vec2(1.0, 1.0);

void main() {
	gl_Position = vec4(vert_pos * u_Scale + u_Trans, 1.0, 1.0);
	v_TexCoord = tex_coord;
};

#shader fragment
#version 330 core

layout(location = 0) out vec4 color;

in vec2 v_TexCoord;

uniform sampler2D u_Texture;
uniform vec4 u_Color = vec4(1.0, 1.0, 1.0, 1.0);

void main() {
	vec4 texColor = texture(u_Texture, v_TexCoord);
	color = texColor * u_Color;
};