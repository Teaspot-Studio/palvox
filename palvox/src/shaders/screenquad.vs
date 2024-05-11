out vec3 v_color;

const vec2[6] QUAD_POS = vec2[](
  vec2(-1.0, -1.0),
  vec2( 1.0,  1.0),
  vec2(-1.0,  1.0),
  vec2(-1.0, -1.0),
  vec2( 1.0, -1.0),
  vec2( 1.0,  1.0)
);

const vec3[6] QUAD_COL = vec3[](
    vec3(1., 0., 0.),
    vec3(0., 1., 0.),
    vec3(0., 0., 1.),
    vec3(1., 0., 0.),
    vec3(0., 0., 1.),
    vec3(0., 1., 0.)
);

void main() {
  gl_Position = vec4(QUAD_POS[gl_VertexID], 0., 1.);
  v_color = QUAD_COL[gl_VertexID];
}