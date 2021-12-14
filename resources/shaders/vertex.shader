#version 400 core

layout(location = 0) in vec4 position;
layout(location = 1) in vec2 tex_coord;

uniform mat4 u_proj;
uniform mat4 u_model;

out vec2 v_texCoord;
uniform vec2 u_texture_translate;

void main()
{
    gl_Position = u_proj * u_model * position;
    v_texCoord = tex_coord + u_texture_translate;
}