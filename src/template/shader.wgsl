struct VertexInput{
    @location(0) position: vec3f,
    @location(1) color: vec3f,
}

struct VertexOutput{
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec3f,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.pos = vec4<f32>(in.position, 1.0);
    out.color = in.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 0.0);
}
