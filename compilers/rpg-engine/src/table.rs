#[derive(Debug, Clone)]
pub struct Table {
    data: Vec<i32>,
    x_size: i32,
    y_size: i32,
    z_size: i32,
}

impl Table {
    pub fn new_1d(x_size: i32) -> Self {
        Self {
            data: vec![0; x_size.max(1) as usize],
            x_size: x_size.max(1),
            y_size: 1,
            z_size: 1,
        }
    }
    
    pub fn new_2d(x_size: i32, y_size: i32) -> Self {
        Self {
            data: vec![0; (x_size.max(1) * y_size.max(1)) as usize],
            x_size: x_size.max(1),
            y_size: y_size.max(1),
            z_size: 1,
        }
    }
    
    pub fn new_3d(x_size: i32, y_size: i32, z_size: i32) -> Self {
        Self {
            data: vec![0; (x_size.max(1) * y_size.max(1) * z_size.max(1)) as usize],
            x_size: x_size.max(1),
            y_size: y_size.max(1),
            z_size: z_size.max(1),
        }
    }
    
    pub fn x_size(&self) -> i32 {
        self.x_size
    }
    
    pub fn y_size(&self) -> i32 {
        self.y_size
    }
    
    pub fn z_size(&self) -> i32 {
        self.z_size
    }
    
    pub fn get_1d(&self, x: i32) -> i32 {
        if x < 0 || x >= self.x_size {
            return 0;
        }
        self.data[x as usize]
    }
    
    pub fn set_1d(&mut self, x: i32, value: i32) {
        if x < 0 || x >= self.x_size {
            return;
        }
        self.data[x as usize] = value;
    }
    
    pub fn get_2d(&self, x: i32, y: i32) -> i32 {
        if x < 0 || x >= self.x_size || y < 0 || y >= self.y_size {
            return 0;
        }
        let index = (y * self.x_size + x) as usize;
        self.data[index]
    }
    
    pub fn set_2d(&mut self, x: i32, y: i32, value: i32) {
        if x < 0 || x >= self.x_size || y < 0 || y >= self.y_size {
            return;
        }
        let index = (y * self.x_size + x) as usize;
        self.data[index] = value;
    }
    
    pub fn get_3d(&self, x: i32, y: i32, z: i32) -> i32 {
        if x < 0 || x >= self.x_size || y < 0 || y >= self.y_size || z < 0 || z >= self.z_size {
            return 0;
        }
        let index = (z * self.y_size * self.x_size + y * self.x_size + x) as usize;
        self.data[index]
    }
    
    pub fn set_3d(&mut self, x: i32, y: i32, z: i32, value: i32) {
        if x < 0 || x >= self.x_size || y < 0 || y >= self.y_size || z < 0 || z >= self.z_size {
            return;
        }
        let index = (z * self.y_size * self.x_size + y * self.x_size + x) as usize;
        self.data[index] = value;
    }
    
    pub fn resize(&mut self, x_size: i32, y_size: i32, z_size: i32) {
        let new_x = x_size.max(1);
        let new_y = y_size.max(1);
        let new_z = z_size.max(1);
        
        let old_data = std::mem::take(&mut self.data);
        let old_x = self.x_size;
        let old_y = self.y_size;
        let old_z = self.z_size;
        
        self.data = vec![0; (new_x * new_y * new_z) as usize];
        self.x_size = new_x;
        self.y_size = new_y;
        self.z_size = new_z;
        
        for z in 0..old_z.min(new_z) {
            for y in 0..old_y.min(new_y) {
                for x in 0..old_x.min(new_x) {
                    let old_index = (z * old_y * old_x + y * old_x + x) as usize;
                    let new_index = (z * new_y * new_x + y * new_x + x) as usize;
                    self.data[new_index] = old_data[old_index];
                }
            }
        }
    }
}
