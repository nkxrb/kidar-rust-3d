struct VertexInput{
    @location(0) position: vec3f,
    @location(1) color: vec3f,
}

struct VertexOutput{
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec3f,
}

struct UniformBufferObject {
    proj: mat4x4<f32>,
    view: mat4x4<f32>,
    model: mat4x4<f32>,
}

@group(0) @binding(0) 
var<uniform> ubo: UniformBufferObject;

fn is_nan(val: f32) -> bool {
    return val != val;
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    // 将位置信息从裁剪空间转换到屏幕空间
    //  * ubo.view  * ubo.view * ubo.model
    let mvp = ubo.proj;
    let pos = mvp * vec4<f32>(in.position, 1.0);
    out.pos = vec4<f32>(pos.xyz/pos.w, 1.0);
    if(is_nan(out.pos.x) || is_nan(out.pos.y) || is_nan(out.pos.z)){
        out.pos = vec4<f32>(0.0);
    }
    out.color = in.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 0.0);
}
