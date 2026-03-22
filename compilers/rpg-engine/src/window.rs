use crate::{Result, RgssError};
use crate::bitmap::Bitmap;
use crate::viewport::Viewport;
use crate::color::Color;
use crate::tone::Tone;

pub struct Window {
    id: u32,
    viewport: Option<Viewport>,
    x: i32,
    y: i32,
    z: i32,
    width: i32,
    height: i32,
    ox: i32,
    oy: i32,
    opacity: i32,
    back_opacity: i32,
    contents_opacity: i32,
    openness: i32,
    active: bool,
    pause: bool,
    windowskin: Option<Bitmap>,
    contents: Option<Bitmap>,
    cursor_rect: (i32, i32, i32, i32),
    visible: bool,
}

impl Window {
    pub fn new(viewport: Option<Viewport>) -> Self {
        Self {
            id: 0,
            viewport,
            x: 0,
            y: 0,
            z: 0,
            width: 0,
            height: 0,
            ox: 0,
            oy: 0,
            opacity: 255,
            back_opacity: 255,
            contents_opacity: 255,
            openness: 255,
            active: true,
            pause: false,
            windowskin: None,
            contents: None,
            cursor_rect: (0, 0, 0, 0),
            visible: true,
        }
    }
    
    pub fn id(&self) -> u32 {
        self.id
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
    
    pub fn width(&self) -> i32 {
        self.width
    }
    
    pub fn set_width(&mut self, width: i32) {
        self.width = width;
    }
    
    pub fn height(&self) -> i32 {
        self.height
    }
    
    pub fn set_height(&mut self, height: i32) {
        self.height = height;
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
    
    pub fn opacity(&self) -> i32 {
        self.opacity
    }
    
    pub fn set_opacity(&mut self, opacity: i32) {
        self.opacity = opacity.clamp(0, 255);
    }
    
    pub fn back_opacity(&self) -> i32 {
        self.back_opacity
    }
    
    pub fn set_back_opacity(&mut self, back_opacity: i32) {
        self.back_opacity = back_opacity.clamp(0, 255);
    }
    
    pub fn contents_opacity(&self) -> i32 {
        self.contents_opacity
    }
    
    pub fn set_contents_opacity(&mut self, contents_opacity: i32) {
        self.contents_opacity = contents_opacity.clamp(0, 255);
    }
    
    pub fn openness(&self) -> i32 {
        self.openness
    }
    
    pub fn set_openness(&mut self, openness: i32) {
        self.openness = openness.clamp(0, 255);
    }
    
    pub fn active(&self) -> bool {
        self.active
    }
    
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
    
    pub fn pause(&self) -> bool {
        self.pause
    }
    
    pub fn set_pause(&mut self, pause: bool) {
        self.pause = pause;
    }
    
    pub fn windowskin(&self) -> Option<&Bitmap> {
        self.windowskin.as_ref()
    }
    
    pub fn set_windowskin(&mut self, windowskin: Option<Bitmap>) {
        self.windowskin = windowskin;
    }
    
    pub fn contents(&self) -> Option<&Bitmap> {
        self.contents.as_ref()
    }
    
    pub fn set_contents(&mut self, contents: Option<Bitmap>) {
        self.contents = contents;
    }
    
    pub fn cursor_rect(&self) -> (i32, i32, i32, i32) {
        self.cursor_rect
    }
    
    pub fn set_cursor_rect(&mut self, x: i32, y: i32, width: i32, height: i32) {
        self.cursor_rect = (x, y, width, height);
    }
    
    pub fn visible(&self) -> bool {
        self.visible
    }
    
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
    
    pub fn update(&mut self) {
    }
    
    pub fn dispose(&mut self) {
        self.windowskin = None;
        self.contents = None;
    }
    
    pub fn disposed(&self) -> bool {
        self.windowskin.is_none() && self.contents.is_none()
    }
}
