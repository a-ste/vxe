use crate::data::{Vertex, LumTess};
use crate::context::Context;
use crate::types::material::Material;
use crate::types::{Transform, DeferredFrameBuffer};
use std::rc::Rc;
use std::sync::RwLock;

/// Structure for containing renderable meshes with assigned materials
pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    material: Option<Rc<RwLock<dyn Material>>>,
    tess: Option<LumTess<Vertex, u32>>
}

impl Mesh
{
    pub fn new<V, I>(vertices: V, indices: I) -> Mesh
        where
            V: Into<Vec<Vertex>>,
            I: Into<Vec<u32>>,
    {
        Mesh {
            vertices: vertices.into(),
            indices: indices.into(),
            material: None,
            tess: None,
        }
    }

    /// Should be called before mesh gets drawn, it creates internal data for rendering
    pub fn build(&mut self, ctx: &mut Context) {
        if self.tess.is_none() {
            self.tess = Some(ctx.new_tess(&self.vertices, &self.indices));
        }
    }

    /// Sets material that the mesh will be using when being drawn
    pub fn set_material(&mut self, material: Rc<RwLock<dyn Material>>) {
        self.material = Some(material);
    }

    /// Draws the mesh with its specified material
    pub fn draw(&self, ctx: &mut Context, frame: &DeferredFrameBuffer, trs: Transform, projection: [[f32; 4]; 4], view: [[f32; 4]; 4]) {
        if let Some(material) = &self.material {
            let tess = self.tess.as_ref().expect("Mesh wasn't built before rendering");

            // TODO: Don't use unwrap here
            material.write().unwrap().render(ctx, frame, tess, trs.raw_matrix(), projection, view);
        }
    }
}