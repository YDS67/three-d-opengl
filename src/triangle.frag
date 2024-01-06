in vec4 v_color;
in vec2 v_uv;
out vec4 fragColor;

uniform sampler2D tex;

vec3 srgb_from_linear_srgb(vec3 rgb) {
    vec3 a = vec3(0.055, 0.055, 0.055);
    vec3 ap1 = vec3(1.0, 1.0, 1.0) + a;
    vec3 g = vec3(2.4, 2.4, 2.4);
    vec3 ginv = 1.0 / g;
    vec3 select = step(vec3(0.0031308, 0.0031308, 0.0031308), rgb);
    vec3 lo = rgb * 12.92;
    vec3 hi = ap1 * pow(rgb, ginv) - a;
    return mix(lo, hi, select);
}

vec4 tex_col;

void main() {
    tex_col = texture(tex, v_uv);
    fragColor = vec4(0.5*srgb_from_linear_srgb(v_color.rgb) + 0.5*srgb_from_linear_srgb(tex_col.rgb),1.0);
}