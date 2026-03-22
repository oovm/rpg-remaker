use crate::{Result, RgssError};
use sdl2::pixels::Color;

pub struct Graphics {
    width: u32,
    height: u32,
    frame_rate: u32,
    brightness: f32,
}

impl Graphics {
    pub fn new() -> Result<Self> {
        Ok(Self {
            width: 640,
            height: 480,
            frame_rate: 60,
            brightness: 1.0,
        })
    }
    
    pub fn update(&mut self) {
        log::debug!("Graphics update");
    }
    
    pub fn clear(&mut self) {
        log::debug!("Graphics clear");
    }
    
    pub fn width(&self) -> u32 {
        self.width
    }
    
    pub fn height(&self) -> u32 {
        self.height
    }
    
    pub fn set_resolution(&mut self, width: u32, height: u32) -> Result<()> {
        self.width = width;
        self.height = height;
        log::debug!("Graphics resolution set to {}x{}", width, height);
        Ok(())
    }
    
    pub fn brightness(&self) -> f32 {
        self.brightness
    }
    
    pub fn set_brightness(&mut self, brightness: f32) {
        self.brightness = brightness.clamp(0.0, 1.0);
    }
    
    pub fn frame_rate(&self) -> u32 {
        self.frame_rate
    }
    
    pub fn set_frame_rate(&mut self, frame_rate: u32) {
        self.frame_rate = frame_rate.max(1);
    }
    
    pub fn wait(&self, duration: u32) {
        use std::thread;
        use std::time::Duration;
        thread::sleep(Duration::from_millis(duration as u64));
    }
    
    pub fn fadeout(&mut self, duration: u32) {
        let steps = duration as f32 / 16.67;
        for i in 0..steps as i32 {
            self.brightness = 1.0 - (i as f32 / steps);
            self.update();
            self.wait(16);
        }
        self.brightness = 0.0;
    }
    
    pub fn fadein(&mut self, duration: u32) {
        let steps = duration as f32 / 16.67;
        for i in 0..steps as i32 {
            self.brightness = i as f32 / steps;
            self.update();
            self.wait(16);
        }
        self.brightness = 1.0;
    }
    
    pub fn resize_screen(&mut self, width: u32, height: u32) -> Result<()> {
        self.set_resolution(width, height)
    }
}
