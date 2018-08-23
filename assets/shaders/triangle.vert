#version 410 core

layout(location = 0) in vec3 Position;
layout(location = 2) in vec3 Color;

out vec4 color;

void main()
{
    gl_Position = vec4(Position, 1.0);
    color = vec4(Color, 1.0);
}
