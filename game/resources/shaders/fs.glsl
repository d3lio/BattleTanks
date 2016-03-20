#version 330 core

uniform sampler2D tex;

in vec2 fs_tex_coord;

out vec3 color;

void main() {
   color = texture(tex, fs_tex_coord).rgb;
}
