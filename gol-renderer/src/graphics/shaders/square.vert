#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(push_constant) uniform PushConstants {
    vec4 color;
    mat4 transform;
} push_constants;

layout(location = 0) out vec4 vertex_color;

vec2 positions[4] = vec2[](
    vec2(0.0, 1.0),
    vec2(1.0, 1.0),
    vec2(0.0, 0.0),
    vec2(1.0, 0.0)
);

void main() {
    vec2 pos = positions[gl_VertexIndex];
    vertex_color = push_constants.color;
    gl_Position = push_constants.transform * vec4(pos, 0.0, 1.0);
}
