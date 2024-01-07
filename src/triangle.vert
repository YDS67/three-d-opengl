in vec3 position;
in vec4 color;
in vec2 uv;

out vec4 v_color;
out vec2 v_uv;
out vec2 v_pos;

uniform mat4 model;
uniform mat4 viewProjection;

void main() {
    v_pos = position.xy;
    gl_Position = viewProjection * model * vec4(position, 1.0);
    v_color = color;
    v_uv = uv;
}