#version 460 core
in vec3 from_color;
out vec4 color;
uniform vec3 tupovec;

void main() {
    vec3 dupa=from_color+tupovec;
    color = vec4(dupa, 1.0);
}