use crate::{Result, RgssError};
use crate::bitmap::Bitmap;
use crate::viewport::Viewport;
use crate::color::Color;
use crate::tone::Tone;
use std::path::Path;

pub struct Sprite {
    id: u32,
    bitmap: Option<Bitmap>,
    viewport: Option<Viewport>,
    x: i32,
    y: i32,
    z: i32,
    ox: i32,
    oy: i32,
    zoom_x: f32,
    zoom_y: f32,
    angle: f32,
    flip_h: bool,
    flip_v: bool,
    bush_depth: i32,
    opacity: i32,
    blend_type: i32,
    color: Color,
    tone: Tone,
    src_rect: (i32, i32, i32, i32),
    visible: bool,
    mirror: bool,
}

impl Sprite {
    pub fn new(viewport: Option<Viewport>) -> Self {
        Self {
            id: 0,
            bitmap: None,
            viewport,
            x: 0,
            y: 0,
            z: 0,
            ox: 0,
            oy: 0,
            zoom_x: 1.0,
            zoom_y: 1.0,
            angle: 0.0,
            flip_h: false,
            flip_v: false,
            bush_depth: 0,
            opacity: 255,
            blend_type: 0,
            color: Color::new(0, 0, 0, 0),
            tone: Tone::new(0, 0, 0, 0),
            src_rect: (0, 0, 0, 0),
            visible: true,
            mirror: false,
        }
    }
    
    pub fn id(&self) -> u32 {
        self.id
    }
    
    pub fn bitmap(&self) -> Option<&Bitmap> {
        self.bitmap.as_ref()
    }
    
    pub fn set_bitmap(&mut self, bitmap: Option<Bitmap>) {
        self.bitmap = bitmap;
    }
    
    pub fn viewport(&self) -> Option<&Viewport> {
        self.viewport.as_ref()
    }
    
    pub fn set_viewport(&mut self, viewport: Option<Viewport>) {
        self.viewport = viewport;
    }
    
    pub fn x(&self) -> i32 {
        self.x
    }
    
    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }
    
    pub fn y(&self) -> i32 {
        self.y
    }
    
    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }
    
    pub fn z(&self) -> i32 {
        self.z
    }
    
    pub fn set_z(&mut self, z: i32) {
        self.z = z;
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
    
    pub fn zoom_x(&self) -> f32 {
        self.zoom_x
    }
    
    pub fn set_zoom_x(&mut self, zoom_x: f32) {
        self.zoom_x = zoom_x;
    }
    
    pub fn zoom_y(&self) -> f32 {
        self.zoom_y
    }
    
    pub fn set_zoom_y(&mut self, zoom_y: f32) {
        self.zoom_y = zoom_y;
    }
    
    pub fn angle(&self) -> f32 {
        self.angle
    }
    
    pub fn set_angle(&mut self, angle: f32) {
        self.angle = angle;
    }
    
    pub fn flip_h(&self) -> bool {
        self.flip_h
    }
    
    pub fn set_flip_h(&mut self, flip_h: bool) {
        self.flip_h = flip_h;
    }
    
    pub fn flip_v(&self) -> bool {
        self.flip_v
    }
    
    pub fn set_flip_v(&mut self, flip_v: bool) {
        self.flip_v = flip_v;
    }
    
    pub fn bush_depth(&self) -> i32 {
        self.bush_depth
    }
    
    pub fn set_bush_depth(&mut self, bush_depth: i32) {
        self.bush_depth = bush_depth;
    }
    
    pub fn opacity(&self) -> i32 {
        self.opacity
    }
    
    pub fn set_opacity(&mut self, opacity: i32) {
        self.opacity = opacity.clamp(0, 255);
    }
    
    pub fn blend_type(&self) -> i32 {
        self.blend_type
    }
    
    pub fn set_blend_type(&mut self, blend_type: i32) {
        self.blend_type = blend_type;
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
    
    pub fn src_rect(&self) -> (i32, i32, i32, i32) {
        self.src_rect
    }
    
    pub fn set_src_rect(&mut self, x: i32, y: i32, width: i32, height: i32) {
        self.src_rect = (x, y, width, height);
    }
    
    pub fn visible(&self) -> bool {
        self.visible
    }
    
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
    
    pub fn mirror(&self) -> bool {
        self.mirror
    }
    
    pub fn set_mirror(&mut self, mirror: bool) {
        self.mirror = mirror;
    }
    
    pub fn flash(&mut self, color: Color, duration: i32) {
        log::debug!("Sprite flash: {:?} for {} frames", color, duration);
    }
    
    pub fn update(&mut self) {
    }
    
    pub fn dispose(&mut self) {
        self.bitmap = None;
        self.viewport = None;
    }
    
    pub fn disposed(&self) -> bool {
        self.bitmap.is_none()
    }
}
