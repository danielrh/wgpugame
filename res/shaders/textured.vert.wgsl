struct Camera {
    u_view_proj: mat4x4<f32>,
}

struct VertexOutput {
    @location(0) vColor: vec4<f32>,
    @builtin(position) member: vec4<f32>,
}

@group(0) @binding(0) 
var<uniform> global: Camera;
var<private> aPosition_1: vec4<f32>;
var<private> aColor_1: vec4<f32>;
var<private> vColor: vec4<f32>;
var<private> gl_Position: vec4<f32>;

fn main_1() {
    let _e6: mat4x4<f32> = global.u_view_proj;
    let _e7: vec4<f32> = aPosition_1;
    gl_Position = (_e6 * _e7);
    let _e9: vec4<f32> = aColor_1;
    vColor = _e9;
    return;
}

@vertex 
fn main(@location(0) aPosition: vec4<f32>, @location(1) aColor: vec4<f32>) -> VertexOutput {
    aPosition_1 = aPosition;
    aColor_1 = aColor;
    _ = (&global.u_view_proj);
    main_1();
    let _e13: vec4<f32> = vColor;
    let _e15: vec4<f32> = gl_Position;
    return VertexOutput(_e13, _e15);
}
