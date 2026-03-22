use crate::{Result, RgssError};
use crate::color::Color;
use crate::rect::Rect;
use std::path::Path;

pub struct Bitmap {
    width: i32,
    height: i32,
    pixels: Vec<u32>,
}

impl Bitmap {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width: width.max(1),
            height: height.max(1),
            pixels: vec![0; (width.max(1) * height.max(1)) as usize],
        }
    }
    
    pub fn from_file(filename: &Path) -> Result<Self> {
        log::info!("Loading bitmap from: {:?}", filename);
        Ok(Self::new(640, 480))
    }
    
    pub fn width(&self) -> i32 {
        self.width
    }
    
    pub fn height(&self) -> i32 {
        self.height
    }
    
    pub fn rect(&self) -> Rect {
        Rect::new(0, 0, self.width, self.height)
    }
    
    pub fn get_pixel(&self, x: i32, y: i32) -> Color {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return Color::new(0, 0, 0, 0);
        }
        let index = (y * self.width + x) as usize;
        let pixel = self.pixels[index];
        Color::from_u32(pixel)
    }
    
    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return;
        }
        let index = (y * self.width + x) as usize;
        self.pixels[index] = color.to_u32();
    }
    
    pub fn fill_rect(&mut self, rect: Rect, color: Color) {
        let x0 = rect.x().max(0);
        let y0 = rect.y().max(0);
        let x1 = (rect.x() + rect.width()).min(self.width);
        let y1 = (rect.y() + rect.height()).min(self.height);
        
        for y in y0..y1 {
            for x in x0..x1 {
                self.set_pixel(x, y, color);
            }
        }
    }
    
    pub fn clear(&mut self) {
        self.pixels.fill(0);
    }
    
    pub fn blt(&mut self, dest_x: i32, dest_y: i32, src_bitmap: &Bitmap, src_rect: Rect, opacity: i32) {
        self.stretch_blt(
            Rect::new(dest_x, dest_y, src_rect.width(), src_rect.height()),
            src_bitmap,
            src_rect,
            opacity
        );
    }
    
    pub fn stretch_blt(&mut self, dest_rect: Rect, src_bitmap: &Bitmap, src_rect: Rect, opacity: i32) {
        log::debug!("Stretch blt: dest={:?}, src={:?}, opacity={}", dest_rect, src_rect, opacity);
    }
    
    pub fn draw_text(&mut self, rect: Rect, text: &str, align: i32) {
        log::debug!("Draw text: '{}' at {:?}, align={}", text, rect, align);
    }
    
    pub fn text_size(&self, text: &str) -> Rect {
        Rect::new(0, 0, text.len() as i32 * 8, 16)
    }
    
    pub fn hue_change(&mut self, hue: i32) {
        log::debug!("Hue change: {}", hue);
    }
    
    pub fn blur(&mut self) {
        log::debug!("Blur");
    }
    
    pub fn radial_blur(&mut self, angle: i32, division: i32) {
        log::debug!("Radial blur: angle={}, division={}", angle, division);
    }
    
    pub fn dispose(&mut self) {
        self.pixels.clear();
    }
    
    pub fn disposed(&self) -> bool {
        self.pixels.is_empty()
    }
}

impl Clone for Bitmap {
    fn clone(&self) -> Self {
        Self {
            width: self.width,
            height: self.height,
            pixels: self.pixels.clone(),
        }
    }
}
