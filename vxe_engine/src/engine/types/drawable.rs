pub trait Drawable {
    fn draw(&mut self, _projection: [[f32; 4]; 4], _view: [[f32; 4]; 4]);
}