#include <common.glsl>

uniform int u_fill;
uniform int u_outline;
uniform vec2 u_offset;
uniform vec2 u_index_offset;
uniform float u_outline_thickness;
uniform int u_count;
uniform float u_size;

#ifdef VERTEX_SHADER
out vec2 v_quad_pos;
attribute vec2 a_pos;
uniform mat3 u_projection_matrix;
uniform mat3 u_view_matrix;
void main() {
    v_quad_pos = a_pos * (1. + u_padding);
    float size = u_unit_radius;
    vec2 pos = v_quad_pos * size + u_unit_position;
    vec3 p_pos = u_projection_matrix * u_view_matrix * vec3(pos, 1.);
    gl_Position = vec4(p_pos.xy, 0., p_pos.z);
}
#endif

#ifdef FRAGMENT_SHADER
uniform sampler2D u_previous_texture;
in vec2 v_quad_pos;

float shapeDistance(vec2 uv, int index, float size) {
    return length(uv - u_offset - float(index) * u_index_offset) - size;
}

#include <shapes.glsl>

void main() {
    commonInit();
    vec2 uv = v_quad_pos;
    vec4 previous_color = texture(u_previous_texture, gl_FragCoord.xy / vec2(textureSize(u_previous_texture, 0)));
    gl_FragColor = previous_color;
    // gl_FragColor = vec4(1);

    for(int i = 0; i < u_count; i++) {
        gl_FragColor = alphaBlend(gl_FragColor, shapeRender(uv, i));
    }

    // if(abs(length(uv) - 1.0) < 0.02) {
    //     gl_FragColor = vec4(0);
    // }
}
#endif