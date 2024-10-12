struct CameraUniform {
    view_pos: vec4<f32>,
    view: mat4x4<f32>,
    view_proj: mat4x4<f32>,
    inv_proj: mat4x4<f32>,
    inv_view: mat4x4<f32>,
}
@group(1) @binding(0)
var<uniform> camera: CameraUniform;
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec3<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) tangent:vec3<f32>,
    @location(4) bitangent:vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec3<f32>,
    @location(1) tan_pos: vec3<f32>,
    @location(2) tan_light_pos: vec3<f32>,
    @location(3) tan_view_pos: vec3<f32>,
};
struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
    @location(9) normal_matrix_0: vec3<f32>,
    @location(10) normal_matrix_1: vec3<f32>,
    @location(11) normal_matrix_2: vec3<f32>,
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
    let normal_matrix = mat3x3<f32>(
        instance.normal_matrix_0,
        instance.normal_matrix_1,
        instance.normal_matrix_2,
    );
    let world_normal = normalize(normal_matrix * model.normal);
    let world_tangent = normalize(normal_matrix * model.tangent);
    let world_bitangent = normalize(normal_matrix * model.bitangent);
    let tangent_matrix = transpose(mat3x3<f32>(
        world_tangent,
        world_bitangent,
        world_normal,
    ));
    let world_position=model_matrix*vec4<f32>(model.position,1.0);

    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position =camera.view_proj * world_position;
    out.tan_pos=tangent_matrix*model.position;
    out.tan_view_pos=tangent_matrix*camera.view_pos.xyz;
    out.tan_light_pos=tangent_matrix*light.position;
    return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;
@group(0) @binding(2)
var t_normal: texture_2d<f32>;
@group(0) @binding(3)
var s_normal: sampler;
 struct Light{
    position: vec3<f32>,
    color: vec3<f32>,
 }
 @group(2) @binding(0)
 var<uniform> light:Light;


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32>{
    let obj_color: vec4<f32>= textureSample(t_diffuse, s_diffuse, in.tex_coords.xy);
    let obj_norm:vec4<f32> =textureSample(t_normal, s_normal, in.tex_coords.xy);
    let amb_str=0.1;
    let amb_color=light.color*amb_str;
    let tan_normal=obj_norm.xyz* 2.0- vec3(1.0,1.0,1.0);
    let light_dir=normalize(in.tan_light_pos-in.tan_pos);
    let view_dir=normalize(in.tan_view_pos.xyz-in.tan_pos);
    let diff_str=max(dot(tan_normal,light_dir),0.0);
    let diff_color=light.color*diff_str;

    let half_dir=normalize(light_dir+view_dir);
    let spec_str=pow(max(dot(tan_normal,half_dir),0.0),64.0);
    let spec_color=spec_str*light.color;
    let result=(amb_color+diff_color+spec_color)*obj_color.xyz;
    return vec4<f32>(result,obj_color.a);
}
 
