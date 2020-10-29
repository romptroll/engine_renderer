#shader vertex
#version 330 core

layout(location = 0) in vec2 vert_pos;
layout(location = 1) in int vert_color;
layout(location = 2) in vec2 tex_coord;

out vec4 v_Color;
out vec2 v_TexCoord;


void main() {
	gl_Position = vec4(vert_pos, 1.0, 1.0);
	float a = vert_color & 255;
    float b = (vert_color >> 8) & 255;
    float g = (vert_color >> 16) & 255;
    float r = (vert_color >> 24) & 255;
    v_Color = vec4(r / 255, g / 255, b / 255, a / 255);
	v_TexCoord = tex_coord;
};

#shader fragment
#version 330 core

layout(location = 0) out vec4 color;

in vec2 v_TexCoord;
in vec4 v_Color;

uniform sampler2D u_Texture;

void main() {
	vec4 texColor = texture(u_Texture, v_TexCoord);
	color = texColor * v_Color;
};
