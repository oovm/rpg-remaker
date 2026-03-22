#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    red: i32,
    green: i32,
    blue: i32,
    alpha: i32,
}

impl Color {
    pub fn new(red: i32, green: i32, blue: i32, alpha: i32) -> Self {
        Self {
            red: red.clamp(0, 255),
            green: green.clamp(0, 255),
            blue: blue.clamp(0, 255),
            alpha: alpha.clamp(0, 255),
        }
    }
    
    pub fn red(&self) -> i32 {
        self.red
    }
    
    pub fn set_red(&mut self, red: i32) {
        self.red = red.clamp(0, 255);
    }
    
    pub fn green(&self) -> i32 {
        self.green
    }
    
    pub fn set_green(&mut self, green: i32) {
        self.green = green.clamp(0, 255);
    }
    
    pub fn blue(&self) -> i32 {
        self.blue
    }
    
    pub fn set_blue(&mut self, blue: i32) {
        self.blue = blue.clamp(0, 255);
    }
    
    pub fn alpha(&self) -> i32 {
        self.alpha
    }
    
    pub fn set_alpha(&mut self, alpha: i32) {
        self.alpha = alpha.clamp(0, 255);
    }
    
    pub fn set(&mut self, red: i32, green: i32, blue: i32, alpha: i32) {
        self.red = red.clamp(0, 255);
        self.green = green.clamp(0, 255);
        self.blue = blue.clamp(0, 255);
        self.alpha = alpha.clamp(0, 255);
    }
    
    pub fn to_u32(&self) -> u32 {
        ((self.alpha as u32) << 24) |
        ((self.red as u32) << 16) |
        ((self.green as u32) << 8) |
        (self.blue as u32)
    }
    
    pub fn from_u32(value: u32) -> Self {
        let alpha = ((value >> 24) & 0xFF) as i32;
        let red = ((value >> 16) & 0xFF) as i32;
        let green = ((value >> 8) & 0xFF) as i32;
        let blue = (value & 0xFF) as i32;
        Self::new(red, green, blue, alpha)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::new(0, 0, 0, 255)
    }
}
