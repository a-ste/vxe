use crate::context::Context;

/// Trait for user-defined event handler of renderer's events
pub trait Handler {
    /// Function that will be called at initialization of Renderer's loop
    fn init(&mut self, _ctx: &mut Context) {}

    /// Function that will be called before drawing frame
    fn update(&mut self, _ctx: &mut Context) {}

    /// Function that will be called for each requested frame
    fn draw(&mut self, _ctx: &mut Context) {}
}