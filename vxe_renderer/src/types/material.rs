use std::collections::HashMap;
use crate::types::parameter::Parameter;
use crate::context::{Context, PipelineState};
use crate::data::{Vertex, LumTess};
use crate::types::DeferredFrameBuffer;
use std::sync::RwLock;
use std::rc::Rc;

pub trait Material {
    fn new(ctx: &mut Context) -> Rc<RwLock<dyn Material>> where Self: Sized;

    fn render(&mut self,
              ctx: &mut Context, frame: &DeferredFrameBuffer, tess: &LumTess<Vertex, u32>,
              trs: [[f32; 4]; 4], projection: [[f32; 4]; 4], view: [[f32; 4]; 4]);

    fn name(&self) -> String;

    fn set_parameter(&mut self, name: String, parameter: Parameter);
    fn parameters(&self) -> HashMap<String, Parameter>;
}

/// Defines default pipeline to be used by deferred rendering
pub fn default_pipeline() -> PipelineState {
    PipelineState::default()
        .enable_clear_color(false)
        .enable_clear_depth(false)
}