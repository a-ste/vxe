use crate::context::Context;

/// Trait for user-defined event handler of renderer's events
pub trait Handler {
    fn init(&mut self, _ctx: &mut Context) {}
    fn draw(&mut self, _ctx: &mut Context) {}
}