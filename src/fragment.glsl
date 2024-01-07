in vec4 v_color;
in vec2 v_uv;
in vec2 v_pos;

out vec4 fragColor;

uniform sampler2D tex;

vec4 tex_col;

void main() {
    if (length(v_pos) > 0.5) {
        discard;
    } else {
        tex_col = texture(tex, v_uv);
        fragColor = 0.5*v_color + 0.5*tex_col;
    }
}