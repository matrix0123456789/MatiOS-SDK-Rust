use crate::gui::bitmap::Bitmap;
use crate::gui::controls::text::TextControl;
use crate::gui::controls::Control;
use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec;
use alloc::vec::Vec;
use core::arch::asm;
use core::cell::RefCell;

pub struct FlexControl {
    pub children: Vec<Box<dyn Control>>,
    pub horizontal: bool,
}
impl Control for FlexControl {
    fn render(&self, pixel_ratio: f32) -> Bitmap {
        let mut rendered_children = vec![];
        let max_width = 1000;
        let sum_height = 1000;
        for child in self.children.iter() {
            for i in 0..1000000000 {
                unsafe {
                    asm! {
                    "nop",
                    "nop",
                    "nop"
                    }
                }
            }
            unsafe {
                asm! {
                "nop",
                "nop",
                "nop"
                }
            }
            let render_result = child.render(pixel_ratio);
            rendered_children.push(render_result);
        }
        let mut ret = Bitmap::new(max_width, sum_height, pixel_ratio);
        let mut pos = 0;
        for child in rendered_children {
            ret.compose(&child, 0, pos);
            pos += child.height;
        }

        return ret;
    }
}
