use cgmath::{InnerSpace, Vector3};
use image::GenericImageView;

use crate::mesh::Mesh;

pub fn generate_surface(heightmap: &str, scale: f32, height_exaggeration: f32) -> Mesh {
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut texies: Vec<[f32; 2]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    let image = image::open(heightmap).expect("Image not found");
    let image_width = image.width();
    let image_height = image.height();

    for y in 0..image_height {
        for x in 0..image_width {
            let image::Rgba(data) = image.get_pixel(x, y);
            
            vertices.push([
                x as f32 * scale,
                data[0] as f32 * height_exaggeration,
                y as f32 * scale,
            ]);
            texies.push([
                x as f32 / image_width as f32,
                y as f32 / image_height as f32,
            ]);
        }
    }

    for y in 0..image_height as i32 {
        for x in 0..image_width as i32 {
            let index = (x + y * image_width as i32) as usize;

            let vertex: Vector3<f32> = vertices[index].into();
            let up: Vector3<f32> = vertices
                [(x + (y + 1).clamp(0, (image_height - 1) as i32) * image_width as i32) as usize]
                .into();
            let down: Vector3<f32> = vertices
                [(x + (y - 1).clamp(0, (image_height - 1) as i32) * image_width as i32) as usize]
                .into();
            let left: Vector3<f32> = vertices
                [((x - 1).clamp(0, (image_width - 1) as i32) + y * image_width as i32) as usize]
                .into();
            let right: Vector3<f32> = vertices
                [((x + 1).clamp(0, (image_width - 1) as i32) + y * image_width as i32) as usize]
                .into();

            let c_1 = Vector3::cross(left - vertex, up - vertex);
            let c_2 = Vector3::cross(up - vertex, right - vertex);
            let c_3 = Vector3::cross(right - vertex, down - vertex);
            let c_4 = Vector3::cross(down - vertex, left - vertex);

            normals.push((c_1 + c_2 + c_3 + c_4).normalize().into());
        }
    }

    for y in 0..image_height - 1 {
        for x in 0..image_width - 1 {
            indices.extend_from_slice(&[
                x + y * image_width,
                x + (y + 1) * image_width,
                (x + 1) + y * image_width,
            ]);
            indices.extend_from_slice(&[
                (x + 1) + y * image_width,
                x + (y + 1) * image_width,
                (x + 1) + (y + 1) * image_width,
            ]);
        }
    }

    Mesh::new(vertices, texies, normals, indices)
}