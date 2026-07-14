use crate::gui::bitmap::Bitmap;

pub mod text;
pub mod flex;

pub trait Control {
    fn render(&self, pixel_ratio: f32) -> Bitmap;
}
