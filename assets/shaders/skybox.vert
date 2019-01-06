#version 400 core

layout(location = 0) in vec3 position;

uniform mat4 view;
uniform mat4 perspective;

out vec3 texCoords;

void main() {
    texCoords = position;

    mat4 view_upper3x3 = mat4(mat3(view));
    vec4 pos = perspective * view_upper3x3 * vec4(position, 1.0);
    gl_Position = pos.xyww; // set depth value to maximum after perspective division
}
