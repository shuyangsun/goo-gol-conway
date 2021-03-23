#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec3 vertex_color;

layout(location = 0) out vec3 fragment_color;

void main() {
    fragment_color = vertex_color;
}
