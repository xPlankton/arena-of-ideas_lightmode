#include <common.glsl>

#ifdef VERTEX_SHADER
out vec2 uv;
attribute vec2 a_pos;

void main() {
    init_fields();
    uv = get_uv(a_pos);
    gl_Position = get_gl_position(uv);
}
#endif

#ifdef FRAGMENT_SHADER
in vec2 uv;

void main() {
    init_fields();
    float t = get_field_value(uv);
    vec4 color = mix(u_background_dark, u_background_light, t);
    gl_FragColor = color;
}
#endif
