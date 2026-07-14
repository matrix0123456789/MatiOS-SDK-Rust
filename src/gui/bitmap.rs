use alloc::vec;
use alloc::vec::Vec;

#[repr(C)]
pub struct Bitmap {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
    pub pixel_ratio: f32,
}
impl Bitmap {
    pub fn new(width: usize, height: usize, pixel_ratio: f32) -> Self {
        return Self {
            width: width,
            height: height,
            pixels: vec![
                Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.0
                };
                width * height
            ],
            pixel_ratio,
        };
    }
    pub fn compose(&mut self, other: &Bitmap, x: usize, y: usize) {
        for i in 0..(if (other.width < self.width - x) {
            other.width
        } else {
            self.width - x
        }) {
            for j in 0..(if (other.height < self.height - y) {
                other.height
            } else {
                self.height - y
            }) {
                self.pixels[(x + i) + (y + j) * self.width] = other.pixels[i + j * other.width];
            }
        }
    }
}
#[repr(C)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl Color {}
