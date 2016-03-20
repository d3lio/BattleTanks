#version 330 core

uniform mat4 mvp;

layout (location = 0) in vec3 vs_position;
layout (location = 1) in vec2 vs_tex_coord;

out vec2 fs_tex_coord;

void main() {
   gl_Position = mvp * vec4(vs_position, 1.0);
   fs_tex_coord = vs_tex_coord;
}
