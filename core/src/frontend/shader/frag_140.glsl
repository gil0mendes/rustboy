#version 140

uniform sampler2D tex;
uniform mat4 palette;

in vec2 v_tex_coords;
out vec4 f_color;

void main() {
    float color = texture(tex, v_tex_coords).x;
    f_color = palette[uint(color * 255.0 + 0.5)];
}