use crate::{Result, RgssError};
use crate::rect::Rect;
use crate::color::Color;
use crate::tone::Tone;

pub struct Viewport {
    id: u32,
    rect: Rect,
    ox: i32,
    oy: i32,
    z: i32,
    color: Color,
    tone: Tone,
    visible: bool,
}

impl Viewport {
    pub fn new(rect: Rect) -> Self {
        Self {
            id: 0,
            rect,
            ox: 0,
            oy: 0,
            z: 0,
            color: Color::new(0, 0, 0, 0),
            tone: Tone::new(0, 0, 0, 0),
            visible: true,
        }
    }
    
    pub fn id(&self) -> u32 {
        self.id
    }
    
    pub fn rect(&self) -> &Rect {
        &self.rect
    }
    
    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }
    
    pub fn ox(&self) -> i32 {
        self.ox
    }
    
    pub fn set_ox(&mut self, ox: i32) {
        self.ox = ox;
    }
    
    pub fn oy(&self) -> i32 {
        self.oy
    }
    
    pub fn set_oy(&mut self, oy: i32) {
        self.oy = oy;
    }
    
    pub fn z(&self) -> i32 {
        self.z
    }
    
    pub fn set_z(&mut self, z: i32) {
        self.z = z;
    }
    
    pub fn color(&self) -> &Color {
        &self.color
    }
    
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
    
    pub fn tone(&self) -> &Tone {
        &self.tone
    }
    
    pub fn set_tone(&mut self, tone: Tone) {
        self.tone = tone;
    }
    
    pub fn visible(&self) -> bool {
        self.visible
    }
    
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
    
    pub fn flash(&mut self, color: Color, duration: i32) {
        log::debug!("Viewport flash: {:?} for {} frames", color, duration);
    }
    
    pub fn update(&mut self) {
    }
    
    pub fn dispose(&mut self) {
    }
    
    pub fn disposed(&self) -> bool {
        false
    }
}
