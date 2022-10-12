#version 150

in float visibility;
in vec2 frag_tex;
in vec3 frag_normal;
out vec4 out_color;

uniform mat4 view;
uniform sampler2D texture0;

void main(void)
{
    vec3 light = vec3(-1, 1.0, -1);

	vec3 t_normal = mat3(view) * frag_normal;
	float shade = dot(normalize(t_normal), mat3(view) * normalize(light) );
    shade = clamp(shade, 0.1, 1);

	out_color = vec4((texture(texture0, frag_tex) * shade).xyz, 1.0);

	out_color = mix(vec4(0.5,0.5,0.5, 0.2), out_color, visibility);
}