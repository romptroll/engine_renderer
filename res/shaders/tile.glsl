#shader vertex
#version 430 core

layout(location = 0) in float tileID;

out int v_TileID;

uniform ivec2 u_MapSize = ivec2(0, 0);
uniform vec2 u_Size = vec2(1.0, 1.0);
uniform vec2 u_Scale = vec2(1.0, 1.0);
uniform vec2 u_Trans = vec2(0.0, 0.0);

void main()
{
	v_TileID = int(tileID);

	int width = int(round(u_MapSize.x));
	int height = u_MapSize.y;

	int x = int(mod(float(gl_InstanceID), float(width)));
	if (x >= width)
		x = 0;
	int y = (gl_InstanceID - x) / width;

	vec2 pos;
	pos.x = float(x) / float(width) * 2.0;
	pos.y = float(y) / float(height) * 2.0;

	gl_Position = vec4((pos * u_Scale * ((u_Size * u_MapSize) / 2.0)) + u_Trans, 1.0, 1.0);
};


#shader geometry
#version 430 core

layout(points) in;
layout(triangle_strip, max_vertices = 4) out;

in int v_TileID[1];

out vec2 v_TexCoord;
out vec4 v_Color;

struct textureRegion
{
	float x1;
	float y1;
	float x2;
	float y2;
	vec4 color;
};

layout(std430, binding = 0) buffer layoutName
{
	textureRegion regions[];
};

uniform vec2 u_Size = vec2(1.0, 1.0);
uniform vec2 u_Scale = vec2(1.0, 1.0);

void main()
{
	vec4 pos;

	if (v_TileID[0] >= 0)
	{

		pos = gl_in[0].gl_Position;
		gl_Position = pos;
		v_TexCoord.x = regions[v_TileID[0]].x1;
		v_TexCoord.y = regions[v_TileID[0]].y2;
		//v_TexCoord.x = 0.0;
		//v_TexCoord.y = 1.0;
		v_Color = regions[v_TileID[0]].color;
		EmitVertex();

		pos = gl_in[0].gl_Position;
		pos.x += u_Size.x * u_Scale.x;
		gl_Position = pos;
		v_TexCoord.x = regions[v_TileID[0]].x2;
		v_TexCoord.y = regions[v_TileID[0]].y2;
		//v_TexCoord.x = 1.0;
		//v_TexCoord.y = 1.0;
		v_Color = regions[v_TileID[0]].color;
		EmitVertex();

		pos = gl_in[0].gl_Position;
		pos.y += u_Size.y * u_Scale.y;
		gl_Position = pos;
		v_TexCoord.x = regions[v_TileID[0]].x1;
		v_TexCoord.y = regions[v_TileID[0]].y1;
		//v_TexCoord.x = 0.0;
		//v_TexCoord.y = 0.0;
		v_Color = regions[v_TileID[0]].color;
		EmitVertex();

		pos = gl_in[0].gl_Position;
		pos.x += u_Size.x * u_Scale.x;
		pos.y += u_Size.y * u_Scale.y;
		gl_Position = pos;
		v_TexCoord.x = regions[v_TileID[0]].x2;
		v_TexCoord.y = regions[v_TileID[0]].y1;
		//v_TexCoord.x = 1.0;
		//v_TexCoord.y = 0.0;
		v_Color = regions[v_TileID[0]].color;
		EmitVertex();

		EndPrimitive();
	}
}

#shader fragment
#version 430 core

layout(location = 0) out vec4 color;

in vec2 v_TexCoord;
in vec4 v_Color;

uniform sampler2D u_Texture;

void main()
{
	//vec2 test = vec2(1, 0);
	vec4 texColor = texture(u_Texture, v_TexCoord);
	color = v_Color * texColor;
	//color = v_Color;
};