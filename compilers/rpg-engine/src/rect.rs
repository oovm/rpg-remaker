#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width: width.max(0),
            height: height.max(0),
        }
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
    
    pub fn width(&self) -> i32 {
        self.width
    }
    
    pub fn set_width(&mut self, width: i32) {
        self.width = width.max(0);
    }
    
    pub fn height(&self) -> i32 {
        self.height
    }
    
    pub fn set_height(&mut self, height: i32) {
        self.height = height.max(0);
    }
    
    pub fn set(&mut self, x: i32, y: i32, width: i32, height: i32) {
        self.x = x;
        self.y = y;
        self.width = width.max(0);
        self.height = height.max(0);
    }
    
    pub fn empty(&self) -> bool {
        self.width <= 0 || self.height <= 0
    }
    
    pub fn right(&self) -> i32 {
        self.x + self.width
    }
    
    pub fn bottom(&self) -> i32 {
        self.y + self.height
    }
    
    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.x && x < self.right() && y >= self.y && y < self.bottom()
    }
    
    pub fn intersects(&self, other: &Rect) -> bool {
        !(self.right() <= other.x || other.right() <= self.x ||
          self.bottom() <= other.y || other.bottom() <= self.y)
    }
    
    pub fn intersection(&self, other: &Rect) -> Rect {
        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let right = self.right().min(other.right());
        let bottom = self.bottom().min(other.bottom());
        let width = right - x;
        let height = bottom - y;
        Rect::new(x, y, width, height)
    }
    
    pub fn union(&self, other: &Rect) -> Rect {
        let x = self.x.min(other.x);
        let y = self.y.min(other.y);
        let right = self.right().max(other.right());
        let bottom = self.bottom().max(other.bottom());
        let width = right - x;
        let height = bottom - y;
        Rect::new(x, y, width, height)
    }
}

impl Default for Rect {
    fn default() -> Self {
        Self::new(0, 0, 0, 0)
    }
}
