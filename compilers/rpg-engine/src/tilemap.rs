use crate::{Result, RgssError};
use crate::viewport::Viewport;
use crate::bitmap::Bitmap;
use crate::table::Table;

pub struct Tilemap {
    id: u32,
    viewport: Option<Viewport>,
    map_data: Option<Table>,
    flash_data: Option<Table>,
    tileset: Option<Bitmap>,
    ox: i32,
    oy: i32,
    visible: bool,
}

impl Tilemap {
    pub fn new(viewport: Option<Viewport>) -> Self {
        Self {
            id: 0,
            viewport,
            map_data: None,
            flash_data: None,
            tileset: None,
            ox: 0,
            oy: 0,
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
    
    pub fn map_data(&self) -> Option<&Table> {
        self.map_data.as_ref()
    }
    
    pub fn set_map_data(&mut self, map_data: Option<Table>) {
        self.map_data = map_data;
    }
    
    pub fn flash_data(&self) -> Option<&Table> {
        self.flash_data.as_ref()
    }
    
    pub fn set_flash_data(&mut self, flash_data: Option<Table>) {
        self.flash_data = flash_data;
    }
    
    pub fn tileset(&self) -> Option<&Bitmap> {
        self.tileset.as_ref()
    }
    
    pub fn set_tileset(&mut self, tileset: Option<Bitmap>) {
        self.tileset = tileset;
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
    
    pub fn visible(&self) -> bool {
        self.visible
    }
    
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
    
    pub fn update(&mut self) {
    }
    
    pub fn dispose(&mut self) {
        self.map_data = None;
        self.flash_data = None;
        self.tileset = None;
        self.viewport = None;
    }
    
    pub fn disposed(&self) -> bool {
        self.tileset.is_none()
    }
}
