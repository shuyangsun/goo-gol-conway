#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(push_constant) uniform PushConstants {
    vec3 color;
    vec2 pos;
    vec2 scale;
} push_constants;

layout(location = 0) out vec3 vertex_color;

vec2 positions[3] = vec2[](
    vec2(0.0, 0.0),
    vec2(1.0, 0.0),
    vec2(0.5, 0.86602540378),  // (0.5, sqrt(3) / 2)
);

void main() {
    vec2 pos = positions[gl_VertexIndex] * push_constants.scale;
    vertex_color = push_constants.color;
    gl_Position = vec3((pos + push_constants.pos), 0.0, 1.0);
}
