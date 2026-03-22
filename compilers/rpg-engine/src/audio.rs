use crate::{Result, RgssError};
use std::collections::HashMap;
use std::path::Path;

pub struct Audio {
    bgm_volume: f32,
    bgs_volume: f32,
    me_volume: f32,
    se_volume: f32,
    playing_bgm: Option<String>,
    playing_bgs: Option<String>,
    playing_me: Option<String>,
}

impl Audio {
    pub fn new() -> Result<Self> {
        Ok(Self {
            bgm_volume: 1.0,
            bgs_volume: 1.0,
            me_volume: 1.0,
            se_volume: 1.0,
            playing_bgm: None,
            playing_bgs: None,
            playing_me: None,
        })
    }
    
    pub fn bgm_play(&mut self, filename: &Path, volume: i32, pitch: i32, pos: i32) {
        self.playing_bgm = Some(filename.to_string_lossy().to_string());
        log::info!("Playing BGM: {:?} (volume: {}, pitch: {}, pos: {})", filename, volume, pitch, pos);
    }
    
    pub fn bgm_stop(&mut self) {
        self.playing_bgm = None;
        log::info!("Stopping BGM");
    }
    
    pub fn bgm_fade(&mut self, time: i32) {
        log::info!("Fading BGM over {} ms", time);
    }
    
    pub fn bgs_play(&mut self, filename: &Path, volume: i32, pitch: i32, pos: i32) {
        self.playing_bgs = Some(filename.to_string_lossy().to_string());
        log::info!("Playing BGS: {:?} (volume: {}, pitch: {}, pos: {})", filename, volume, pitch, pos);
    }
    
    pub fn bgs_stop(&mut self) {
        self.playing_bgs = None;
        log::info!("Stopping BGS");
    }
    
    pub fn bgs_fade(&mut self, time: i32) {
        log::info!("Fading BGS over {} ms", time);
    }
    
    pub fn me_play(&mut self, filename: &Path, volume: i32, pitch: i32) {
        self.playing_me = Some(filename.to_string_lossy().to_string());
        log::info!("Playing ME: {:?} (volume: {}, pitch: {})", filename, volume, pitch);
    }
    
    pub fn me_stop(&mut self) {
        self.playing_me = None;
        log::info!("Stopping ME");
    }
    
    pub fn me_fade(&mut self, time: i32) {
        log::info!("Fading ME over {} ms", time);
    }
    
    pub fn se_play(&mut self, filename: &Path, volume: i32, pitch: i32) {
        log::info!("Playing SE: {:?} (volume: {}, pitch: {})", filename, volume, pitch);
    }
    
    pub fn se_stop(&mut self) {
        log::info!("Stopping all SE");
    }
    
    pub fn bgm_volume(&self) -> f32 {
        self.bgm_volume
    }
    
    pub fn set_bgm_volume(&mut self, volume: f32) {
        self.bgm_volume = volume.clamp(0.0, 1.0);
    }
    
    pub fn bgs_volume(&self) -> f32 {
        self.bgs_volume
    }
    
    pub fn set_bgs_volume(&mut self, volume: f32) {
        self.bgs_volume = volume.clamp(0.0, 1.0);
    }
    
    pub fn me_volume(&self) -> f32 {
        self.me_volume
    }
    
    pub fn set_me_volume(&mut self, volume: f32) {
        self.me_volume = volume.clamp(0.0, 1.0);
    }
    
    pub fn se_volume(&self) -> f32 {
        self.se_volume
    }
    
    pub fn set_se_volume(&mut self, volume: f32) {
        self.se_volume = volume.clamp(0.0, 1.0);
    }
}
