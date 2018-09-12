#version 410 core

layout(location = 0) in vec3 Position;
//layout(location = 2) in vec3 Color;
layout(location = 5) in vec2 texCoord;

// transformation matrices:
//uniform mat4 m; // model

out vec2 uv;

void main()
{
    gl_Position = vec4(Position, 1.0);
    uv = texCoord;
}
