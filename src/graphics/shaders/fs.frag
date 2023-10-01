#version 460

layout(location = 0) in vec2 texture_coord;

layout(location = 0) out vec4 o_color;

layout(set = 2, binding = 0) uniform sampler u_sampler;
layout(set = 2, binding = 1) uniform texture2D u_textures;

layout(set = 1, binding = 0) uniform Sprite {
    mat4 model;
    vec3 color;
    float visible;
};

void main() {
    o_color = texture(sampler2D(u_textures, u_sampler), texture_coord) * vec4(color, 1.0);
}

