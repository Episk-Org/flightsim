#version 150

in float visibility;
in vec2 frag_tex;
in vec3 frag_normal;
in vec3 frag_position;
out vec4 out_color;

uniform sampler2D texture0;

uniform float time;
uniform mat4 view;
uniform mat4 transformation;

void main(void)
{
	vec3 water_color = vec3(1, 1, 1);
	float specular_exponent = 50.0;

	vec3 light_dir = normalize(vec3(0.5, 0, 1.0));
	vec3 normal = normalize(texture(texture0, frag_tex).xyz);

	vec3 v_normal = mat3(view * transformation) * normal;
	vec3 v_position = vec3(view * transformation * vec4(frag_position, 1.0));

	// light calc
	vec3 light = vec3(0.2, 0.803, 1);

	vec3 light_source = mat3(view) * light_dir;
	float diffuse = clamp(dot(v_normal, light_source), 0.0, 1);

	vec3 r = reflect(-light_source, v_normal);
	vec3 v = normalize(-v_position); // View direction

	float specular = dot(r, v);
	if (specular > 0.0) {
		specular = 1.0 * pow(specular, specular_exponent);
	}

	specular = max(specular, 0.0);
	float shade = diffuse + specular;
	light += shade * water_color;

	out_color = vec4(light/2, 0.8);

	out_color = mix(vec4(0.5,0.5,0.5, 0.5), out_color, visibility);
}
