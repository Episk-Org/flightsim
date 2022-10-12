use cgmath::{Matrix4, Vector3};

use crate::{
    asset_manager, mesh::Mesh, model::Model, window_handler::GlContext, camera::Camera,
};

pub struct Entity {
    position: Vector3<f32>,
    model: Model,
}

impl Entity {
    pub fn new(
        gl: &GlContext,
        object_file: &str,
        vert_shader: &'static str,
        frag_shader: &'static str,
        texture_files: &[&str],
        position: Vector3<f32>,
    ) -> Self {
        Self {
            position,
            model: asset_manager::load_object(
                gl,
                object_file,
                vert_shader,
                frag_shader,
                texture_files,
            ),
        }
    }

    pub fn new_obj(
        gl: &GlContext,
        object: Mesh,
        vert_shader: &'static str,
        frag_shader: &'static str,
        texture_files: &[&str],
        position: Vector3<f32>,
    ) -> Self {
        Self {
            position,
            model: asset_manager::load_obj_preloaded(
                gl,
                object,
                vert_shader,
                frag_shader,
                texture_files,
            ),
        }
    }

    pub fn render(&self, gl: &GlContext, time: &f32, camera: &mut Camera) {
        let matrix = Matrix4::from_translation(self.position);
        self.model.render(gl, matrix, time, camera);
    }
}
