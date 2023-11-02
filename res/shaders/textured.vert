#version 450
layout(set=0, binding=0)
uniform Camera {
    mat4 u_view_proj;
};

layout(location=0) in vec4 aPosition;
layout(location=1) in vec4 aColor;

layout(location=0) out vec4 vColor;

void main() {
    gl_Position = u_view_proj * aPosition;
    vColor = aColor;
    // vTexCoord = aTexCoord;
}
