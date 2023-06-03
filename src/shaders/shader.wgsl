struct VertexInput
{
    @location(0) position : vec4<f32>,
    @location(1) color    : vec4<f32>,
};

struct VertexOut
{
  @builtin(position) position : vec4<f32>,
  @location(0)       color    : vec4<f32>,
}

@vertex fn vertex_main(
  model : VertexInput,
) -> VertexOut {
  var output : VertexOut;

  output.position = model.position;
  output.color    = model.color;

  return output;
}

@fragment fn fragment_main(
  in : VertexOut
) -> @location(0) vec4<f32> {
  return in.color;
}
