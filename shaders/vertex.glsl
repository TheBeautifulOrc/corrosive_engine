#version 450

in vec2 position;

uniform vec2 tri_pos;

void main() {
	vec2 pos = vec2(position.x + tri_pos.x, position.y + tri_pos.y);
	gl_Position = vec4(pos, 0.0, 1.0);
}