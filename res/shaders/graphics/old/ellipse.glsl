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
layout(triangle_strip, max_vertices = 256) out;

in int tight_color[];

out vec4 color;

uniform int u_ellipse_detail = 100;

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
    /*pos.z /= 2.0;
    pos.w /= 2.0;
    pos.x += pos.z;
    pos.y += pos.w;*/

    color = col;

    ///////////////////////////////////////////////////////
    for(int i = 0; i < u_ellipse_detail+1; i++) {
        float x = cos(float(i) / float(u_ellipse_detail) * 2.0 * 3.14) * pos.z / 2.0;
        float y = sin(float(i) / float(u_ellipse_detail) * 2.0 * 3.14) * pos.w / 2.0;
        x = pos.x+pos.z/2 + x;
        y = pos.y+pos.w/2 + y;

        gl_Position = vec4(x, y, 1.0, 1.0);
        EmitVertex();

        gl_Position = vec4(pos.x+pos.z/2, pos.y+pos.w/2, 1.0, 1.0);
        EmitVertex();
    }
}

#shader fragment
#version 330 core

in vec4 color;

layout(location = 0) out vec4 out_color;

void main() {
	out_color = color;
}