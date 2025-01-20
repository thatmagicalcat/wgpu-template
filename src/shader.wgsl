// Vertex

struct VertexInput {
    @location(0) position: vec4<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.position = model.position;
    out.color = vec4(model.color, 1.0);

    return out;
}

// Fragment

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
