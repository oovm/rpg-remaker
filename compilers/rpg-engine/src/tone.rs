#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tone {
    red: i32,
    green: i32,
    blue: i32,
    gray: i32,
}

impl Tone {
    pub fn new(red: i32, green: i32, blue: i32, gray: i32) -> Self {
        Self {
            red: red.clamp(-255, 255),
            green: green.clamp(-255, 255),
            blue: blue.clamp(-255, 255),
            gray: gray.clamp(0, 255),
        }
    }
    
    pub fn red(&self) -> i32 {
        self.red
    }
    
    pub fn set_red(&mut self, red: i32) {
        self.red = red.clamp(-255, 255);
    }
    
    pub fn green(&self) -> i32 {
        self.green
    }
    
    pub fn set_green(&mut self, green: i32) {
        self.green = green.clamp(-255, 255);
    }
    
    pub fn blue(&self) -> i32 {
        self.blue
    }
    
    pub fn set_blue(&mut self, blue: i32) {
        self.blue = blue.clamp(-255, 255);
    }
    
    pub fn gray(&self) -> i32 {
        self.gray
    }
    
    pub fn set_gray(&mut self, gray: i32) {
        self.gray = gray.clamp(0, 255);
    }
    
    pub fn set(&mut self, red: i32, green: i32, blue: i32, gray: i32) {
        self.red = red.clamp(-255, 255);
        self.green = green.clamp(-255, 255);
        self.blue = blue.clamp(-255, 255);
        self.gray = gray.clamp(0, 255);
    }
}

impl Default for Tone {
    fn default() -> Self {
        Self::new(0, 0, 0, 0)
    }
}
