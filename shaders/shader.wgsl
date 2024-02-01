struct CameraUniform{
    view_proj:mat4x4<f32>,
}
@group(1) @binding(0)
var<uniform> camera: CameraUniform;
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec3<f32>,
    @location(2) normal: vec3<f32>
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) world_position: vec3<f32>,
};
struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
       let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.world_normal = model.normal;
    var world_position:vec4<f32>=model_matrix*vec4<f32>(model.position,1.0);
    out.world_position=world_position.xyz;
    out.clip_position =camera.view_proj * model_matrix * vec4<f32>(model.position, 1.0);
    return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;
 struct Light{
    position: vec3<f32>,
    color: vec3<f32>,
 }
 @group(2) @binding(0)
 var<uniform> light:Light;


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let obj_color:vec4<f32>= textureSample(t_diffuse, s_diffuse, in.tex_coords.xy);
    let amb_str=0.1;
    let amb_color=light.color*amb_str;
    let light_dir=normalize(light.position-in.world_position);
    let diff_str=max(dot(in.world_normal,light_dir),0.0);
    let diff_color=light.color*diff_str;
    let result=(amb_color+diff_color)*obj_color.xyz;
    return vec4<f32>(result,obj_color.a);
}
 
