use crate::{Result, RgssError};

pub struct Scene {
    name: String,
    active: bool,
    visible: bool,
}

impl Scene {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            active: false,
            visible: false,
        }
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn active(&self) -> bool {
        self.active
    }
    
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
    
    pub fn visible(&self) -> bool {
        self.visible
    }
    
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
    
    pub fn start(&mut self) {
        self.active = true;
        self.visible = true;
        log::info!("Scene started: {}", self.name);
    }
    
    pub fn stop(&mut self) {
        self.active = false;
        self.visible = false;
        log::info!("Scene stopped: {}", self.name);
    }
    
    pub fn update(&mut self) {
    }
    
    pub fn draw(&mut self) {
    }
}

pub struct SceneManager {
    scenes: Vec<Scene>,
    current_scene: Option<usize>,
}

impl SceneManager {
    pub fn new() -> Self {
        Self {
            scenes: Vec::new(),
            current_scene: None,
        }
    }
    
    pub fn add_scene(&mut self, scene: Scene) {
        self.scenes.push(scene);
    }
    
    pub fn push_scene(&mut self, scene: Scene) {
        if let Some(index) = self.current_scene {
            if let Some(current) = self.scenes.get_mut(index) {
                current.set_active(false);
            }
        }
        self.scenes.push(scene);
        let new_index = self.scenes.len() - 1;
        self.current_scene = Some(new_index);
        if let Some(new) = self.scenes.get_mut(new_index) {
            new.start();
        }
    }
    
    pub fn pop_scene(&mut self) {
        if let Some(index) = self.current_scene {
            if let Some(current) = self.scenes.get_mut(index) {
                current.stop();
            }
            self.scenes.pop();
            self.current_scene = if self.scenes.is_empty() {
                None
            } else {
                let new_index = self.scenes.len() - 1;
                if let Some(new) = self.scenes.get_mut(new_index) {
                    new.set_active(true);
                }
                Some(new_index)
            };
        }
    }
    
    pub fn switch_scene(&mut self, scene: Scene) {
        if let Some(index) = self.current_scene {
            if let Some(current) = self.scenes.get_mut(index) {
                current.stop();
            }
            self.scenes.remove(index);
        }
        self.scenes.push(scene);
        let new_index = self.scenes.len() - 1;
        self.current_scene = Some(new_index);
        if let Some(new) = self.scenes.get_mut(new_index) {
            new.start();
        }
    }
    
    pub fn current_scene(&self) -> Option<&Scene> {
        self.current_scene.and_then(|index| self.scenes.get(index))
    }
    
    pub fn current_scene_mut(&mut self) -> Option<&mut Scene> {
        self.current_scene.and_then(move |index| self.scenes.get_mut(index))
    }
    
    pub fn update(&mut self) {
        if let Some(scene) = self.current_scene_mut() {
            scene.update();
        }
    }
    
    pub fn draw(&mut self) {
        if let Some(scene) = self.current_scene_mut() {
            scene.draw();
        }
    }
}

impl Default for SceneManager {
    fn default() -> Self {
        Self::new()
    }
}
