use drm::{control::{dumbbuffer::DumbBuffer, Device}};
use image::{DynamicImage, Rgba, RgbaImage};

use crate::{card::*, types::{ColorFormat, Vec2}};

pub struct ToucanAPI {
    pub(crate) card: Card,
    buffer: Option<DumbBuffer>,
}



impl ToucanAPI {
    pub fn init(card: Card) -> Self {
        Self { card, buffer: None }
    }
    
    pub fn create_buffer(&mut self, size: Vec2<u32>, format: ColorFormat, bpp: u32) {
        self.buffer = Some(self.card.create_dumb_buffer((size.x, size.y), format.to_drm(), bpp)
            .expect("Failed to create buffer!"));
    }
    
    pub fn create_image(&mut self, out: &str) {
        let mut buffer = self.card.map_dumb_buffer(self.buffer.as_mut().unwrap());
        for x in 0..800 {
            for y in 0..600 {
                buffer.as_mut().unwrap()[(((y * 800 + x) * 4) as usize) + 0] = 233;
                buffer.as_mut().unwrap()[(((y * 800 + x) * 4) as usize) + 1] = 63;
                buffer.as_mut().unwrap()[(((y * 800 + x) * 4) as usize) + 2] = 125;
                buffer.as_mut().unwrap()[(((y * 800 + x) * 4) as usize) + 3] = 255;
            }
        }
        let image = DynamicImage::ImageRgba8(RgbaImage::from_vec(800, 600, buffer.unwrap().to_vec()).unwrap());
        image.save_with_format(out, image::ImageFormat::Png);
    }
    
    pub fn get_card(&self) -> &Card { &self.card }
}