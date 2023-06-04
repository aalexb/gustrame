#version 460 core
layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 col;
uniform mat4 projection;
uniform mat4 model;

out vec3 from_color;
void main() {
    from_color=col;
    gl_Position =projection *model*  vec4(pos.xy, 0.0, 1.0);
}