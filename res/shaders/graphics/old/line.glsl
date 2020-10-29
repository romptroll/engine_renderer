#shader vertex
#version 330 core

layout(location = 0) in vec4 v_bounds;
layout(location = 1) in int v_color;

out int tight_color;

void main() {
    gl_Position = v_bounds;
    tight_color = v_color;
}

#shader geometry
#version 330 core

layout(points) in;
layout(triangle_strip, max_vertices = 6) out;

in int tight_color[];

out vec4 color;

uniform float u_line_width;

vec4 normal_color(int tight_color) {
    float a = tight_color & 255;
    float b = (tight_color >> 8) & 255;
    float g = (tight_color >> 16) & 255;
    float r = (tight_color >> 24) & 255;
    return vec4(r / 255, g / 255, b / 255, a / 255);
}

void main() {
    vec4 col = normal_color(tight_color[0]);
    vec4 pos = gl_in[0].gl_Position;

    color = col;

    float x_len = pos.x - pos.z;
    float y_len = pos.y - pos.w;
    
    float a = atan(y_len / x_len);
    float pi = 3.14 / 2.0;

    vec2 pos1 = vec2(pos.x + cos(a + pi) * u_line_width, pos.y + sin(a + pi) * u_line_width); //(0, 0)
    vec2 pos2 = vec2(pos.x + cos(a - pi) * u_line_width, pos.y + sin(a - pi) * u_line_width); //(1, 0)
    vec2 pos3 = vec2(pos.z + cos(a + pi) * u_line_width, pos.w + sin(a + pi) * u_line_width); //(0, 1)
    vec2 pos4 = vec2(pos.z + cos(a - pi) * u_line_width, pos.w + sin(a - pi) * u_line_width); //(1, 1)

    ///////////////////////////////////////////////////////

    //(0, 0)
    gl_Position = vec4(pos1.x, pos1.y, 1.0, 1.0);
    EmitVertex();
    
    //(1, 0)
    gl_Position = vec4(pos2.x, pos2.y, 1.0, 1.0);
    EmitVertex();

    //(0, 1)
    gl_Position = vec4(pos3.x, pos3.y, 1.0, 1.0);
    EmitVertex();

    //(1, 1)
    gl_Position = vec4(pos4.x, pos4.y, 1.0, 1.0);
    EmitVertex();
}


#shader fragment
#version 330 core

in vec4 color;

layout(location = 0) out vec4 out_color;

void main() {
	out_color = color;
}