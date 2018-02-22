#version 110

uniform sampler2D tex;
uniform mat4 palette;

varying vec2 v_tex_coords;

void main() {
  float color = texture2D(tex, v_tex_coords).x;
  gl_FragColor = palette[int(color * 255.0 + 0.5)];
}
