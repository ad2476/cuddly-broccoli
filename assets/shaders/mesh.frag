#version 400 core

in vec3 WS_position; // world-space position
in vec3 WS_normal;   // world-space normal

out vec3 fragColor;

uniform vec3 cDiffuse;

const float u_time = 0.0;
const float light_dist = 10.0;
const float PI = 3.1415926535897932384626433832795;
const float light_theta = 0.0;

const float ambientStrength = 0.3;
const vec3 lightColor = vec3(1.0);

void main() {
    /* light position as function of time */
    float phi = abs(fract(u_time/1000.0) - 0.5)*2*PI - PI/2.0;
    float l_x = light_dist * cos(light_theta) * sin(phi);
    float l_y = light_dist * cos(phi);
    float l_z = light_dist * sin(light_theta) * sin(phi);

    vec3 WS_toLight = normalize(vec3(l_x, l_y, l_z) - WS_position);

    /* lighting model */
    vec3 ambient = ambientStrength * lightColor;

    float diff = max(0.0, dot(normalize(WS_normal), WS_toLight));
    vec3 diffuse = diff * lightColor;
    fragColor = (ambient + diffuse) * cDiffuse;
}
