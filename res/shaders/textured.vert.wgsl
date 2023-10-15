struct VertexOutput {
    @location(0) vColor: vec4<f32>,
    @builtin(position) member: vec4<f32>,
}

var<private> aPosition_1: vec4<f32>;
var<private> aColor_1: vec4<f32>;
var<private> vColor: vec4<f32>;
var<private> gl_Position: vec4<f32>;

fn main_1() {
    let _e4: vec4<f32> = aPosition_1;
    gl_Position = _e4;
    let _e5: vec4<f32> = aColor_1;
    vColor = _e5;
    return;
}

@vertex 
fn main(@location(0) aPosition: vec4<f32>, @location(1) aColor: vec4<f32>) -> VertexOutput {
    aPosition_1 = aPosition;
    aColor_1 = aColor;
    main_1();
    let _e11: vec4<f32> = vColor;
    let _e13: vec4<f32> = gl_Position;
    return VertexOutput(_e11, _e13);
}
