use std::{
    env,
    io::{BufReader, Cursor, Read},
    path,
};

use wgpu::util::DeviceExt;

use crate::{
    model::{self, Material, Mesh, ModelVertex},
    textures,
};

pub fn load_string(path: &str) -> Result<String, std::io::Error> {
    use std::fs::File;
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
pub fn load_binary(path: &str) -> Result<Vec<u8>, std::io::Error> {
    let path = std::path::Path::new(path);
    let data = std::fs::read(path)?;
    Ok(data)
}

pub fn load_texture(
    file_name: &str,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    is_normal_map: bool,
) -> Result<textures::Texture, anyhow::Error> {
    use std::fs;
    let f = fs::File::open(get_location_of_assets()).unwrap();
    let mut archive = tar::Archive::new(f);
    //look for the file in the archive
    let mut bytes: Vec<u8> = Vec::new();
    for file in archive.entries().unwrap() {
        let file = file?;
        //only use files in the textures folder
        if file.path().unwrap().parent().unwrap() != std::path::Path::new("textures") {
            continue;
        }
        if file.path().unwrap().file_name().unwrap() == std::path::Path::new(file_name) {
            file.bytes().for_each(|b| bytes.push(b.unwrap()));
            break;
        }
    }
    let texture = textures::Texture::from_bytes(device, &bytes, file_name, queue, is_normal_map)?;
    Ok(texture)
}
pub fn read_game_assets(file_name: &str) -> Result<Vec<u8>, anyhow::Error> {
    use std::fs;

    let f = fs::File::open(get_location_of_assets())?;
    let mut archive = tar::Archive::new(f);
    //look for the file in the archive
    let mut bytes: Vec<u8> = Vec::new();
    for file in archive.entries()? {
        let file = file?;
        if file.path()? == std::path::Path::new(file_name) {
            file.bytes().for_each(|b| bytes.push(b.unwrap()));
            break;
        }
    }
    Ok(bytes)
}
fn get_location_of_assets() -> String {
    let mut path = env::current_exe().unwrap();
    path.pop();
    path.join("game.assets").to_str().unwrap().to_string()
}
pub fn load_model(
    file_name: &str,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    layout: &wgpu::BindGroupLayout,
) -> Result<crate::model::Model, anyhow::Error> {
    use std::fs;

    let f = fs::File::open(get_location_of_assets()).unwrap();
    let mut archive = tar::Archive::new(f);
    //look for the file in the archive
    let mut obj_text: String = String::new();
    for file in archive.entries().unwrap() {
        let mut file = file.unwrap();
        //only use files in the textures folder
        if file.path().unwrap().parent().unwrap() != std::path::Path::new("res") {
            continue;
        }
        if file.path().unwrap().file_name().unwrap() == std::path::Path::new(file_name) {
            file.read_to_string(&mut obj_text)?;
            break;
        }
    }
    if obj_text == "" {
        panic!("Could not find model {:?}", file_name);
    }
    let obj_cursor = std::io::Cursor::new(obj_text);
    let mut obj_reader = BufReader::new(obj_cursor);
    let (models, obj_materials) = tobj::load_obj_buf(
        &mut obj_reader,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
        |p| {
            //set parent of path to textures folder
            let mut path = path::PathBuf::from("res");
            path.push(p);
            let mat_text = read_game_assets(path.to_str().unwrap_or("")).unwrap();
            tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
        },
    )?;

    let mut materials: Vec<Material> = Vec::new();
    for mat in obj_materials? {
        let diffuse_texture = load_texture(&mat.diffuse_texture.unwrap(), device, queue, false)?;
        let normal_texture = load_texture(&mat.normal_texture.unwrap(), device, queue, true)?;
        materials.push(Material::new(
            device,
            &mat.name,
            diffuse_texture,
            normal_texture,
            layout,
        ));
    }
    let meshes = models
        .into_iter()
        .map(|m| {
            let mut vertices = (0..m.mesh.positions.len() / 3)
                .map(|i| model::ModelVertex {
                    position: [
                        m.mesh.positions[i * 3],
                        m.mesh.positions[i * 3 + 1],
                        m.mesh.positions[i * 3 + 2],
                    ],
                    tex_coords: [m.mesh.texcoords[i * 2], m.mesh.texcoords[i * 2 + 1]],
                    normal: [
                        m.mesh.normals[i * 3],
                        m.mesh.normals[i * 3 + 1],
                        m.mesh.normals[i * 3 + 2],
                    ],
                    tangent: [0.0; 3],
                    bitangent: [0.0; 3],
                })
                .collect::<Vec<ModelVertex>>();
            let indices = &m.mesh.indices;
            let mut triangles_included = vec![0; vertices.len()];
            for c in indices.chunks(3) {
                let v0 = vertices[c[0] as usize];
                let v1 = vertices[c[1] as usize];
                let v2 = vertices[c[2] as usize];
                let pos0: cgmath::Vector3<_> = v0.position.into();
                let pos1: cgmath::Vector3<_> = v1.position.into();
                let pos2: cgmath::Vector3<_> = v2.position.into();
                let uv0: cgmath::Vector2<_> = v0.tex_coords.into();
                let uv1: cgmath::Vector2<_> = v1.tex_coords.into();
                let uv2: cgmath::Vector2<_> = v2.tex_coords.into();
                let delta_pos1 = pos1 - pos0;
                let delta_pos2 = pos2 - pos0;
                let delta_uv1 = uv1 - uv0;
                let delta_uv2 = uv2 - uv0;
                let r = 1.0 / (delta_uv1.x * delta_uv2.y - delta_uv1.y * delta_uv2.x);
                let tangent = (delta_pos1 * delta_uv2.y - delta_pos2 * delta_uv1.y) * r;
                let bitangent = (delta_pos2 * delta_uv1.x - delta_pos1 * delta_uv2.x) * -r;
                vertices[c[0] as usize].tangent =
                    (tangent + cgmath::Vector3::from(vertices[c[0] as usize].tangent)).into();
                vertices[c[1] as usize].tangent =
                    (tangent + cgmath::Vector3::from(vertices[c[1] as usize].tangent)).into();
                vertices[c[2] as usize].tangent =
                    (tangent + cgmath::Vector3::from(vertices[c[2] as usize].tangent)).into();
                vertices[c[0] as usize].bitangent =
                    (bitangent + cgmath::Vector3::from(vertices[c[0] as usize].bitangent)).into();
                vertices[c[1] as usize].bitangent =
                    (bitangent + cgmath::Vector3::from(vertices[c[1] as usize].bitangent)).into();
                vertices[c[2] as usize].bitangent =
                    (bitangent + cgmath::Vector3::from(vertices[c[2] as usize].bitangent)).into();
                triangles_included[c[0] as usize] += 1;
                triangles_included[c[1] as usize] += 1;
                triangles_included[c[2] as usize] += 1;
            }
            for (i, n) in triangles_included.into_iter().enumerate() {
                let denom = 1.0 / n as f32;
                let v = &mut vertices[i];
                v.tangent = (cgmath::Vector3::from(v.tangent) * denom).into();
                v.bitangent = (cgmath::Vector3::from(v.bitangent) * denom).into();
            }
            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} Vertex buffer", file_name)),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} Index buffer", file_name)),
                contents: bytemuck::cast_slice(&m.mesh.indices),
                usage: wgpu::BufferUsages::INDEX,
            });
            Mesh {
                name: file_name.to_string(),
                vertex_buffer,
                index_buffer,
                num_elements: m.mesh.indices.len() as u32,
                material: m.mesh.material_id.unwrap_or(0),
            }
        })
        .collect::<Vec<Mesh>>();
    Ok(model::Model { meshes, materials })
}
