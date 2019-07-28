#version 140

uniform sampler2D tex_front;
uniform sampler2D tex_back;
uniform mat4 palette;

in vec2 v_tex_coords;
out vec4 f_color;

void main() {
    float color_front = texture(tex_front, v_tex_coords).x;
    float color_back = texture(tex_back, v_tex_coords).x;
    float color = mix(color_front, color_back, 0.5);
    f_color = palette[uint(color * 255.0 + 0.5)];
}
