#version 150

in vec3 position;
in vec3 normal;
in vec2 tex;

out vec2 frag_tex;
out vec3 frag_normal;
out vec3 frag_position;
out float visibility;

uniform float time;
uniform mat4 view;
uniform mat4 projection_view;
uniform mat4 transformation;

const float gradient = 0.1;

void main(void)
{
	frag_tex = tex * 100;
    frag_normal = normal;
	frag_position = position;

	vec4 worldPosition = transformation * vec4(position, 1.0);
	vec4 positionRelativeToCamera = view * worldPosition;
	gl_Position = projection_view * worldPosition;

	float distance = length(positionRelativeToCamera.xyz)/5000;
	visibility = exp(-pow(distance*distance, gradient));
	visibility = clamp(visibility, 0.0, 1.0);
}
