#version 460

layout(location = 0) in vec2 a_position;
layout(location = 1) in vec2 a_texture_coord;

layout(set = 0, binding = 0) uniform Scene {
    mat4 view;
};

layout(set = 1, binding = 0) uniform Sprite {
    mat4 model;
    vec3 color;
    float visible;
};

layout(location = 0) out vec2 texture_coord;

void main() {
    gl_Position = view * model * vec4(a_position, 0.0, visible);
    texture_coord = a_texture_coord;
}

