#version 400 core

in vec3 texCoords;
out vec4 fragColor;

uniform samplerCube tex;

void main() {
	fragColor = texture(tex, texCoords);
}
