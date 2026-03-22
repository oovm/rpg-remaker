use bevy::prelude::*;

/// Graphics 系统资源，管理游戏画面相关状态
#[derive(Resource)]
pub struct Graphics {
    /// 屏幕宽度
    width: u32,
    /// 屏幕高度
    height: u32,
    /// 帧率
    frame_rate: u32,
    /// 亮度（0.0 - 1.0）
    brightness: f32,
    /// 是否冻结画面更新
    frozen: bool,
    /// 帧计数
    frame_count: u64,
}

impl Graphics {
    /// 创建新的 Graphics 资源
    pub fn new() -> Self {
        Self {
            width: 640,
            height: 480,
            frame_rate: 60,
            brightness: 1.0,
            frozen: false,
            frame_count: 0,
        }
    }

    /// 获取屏幕宽度
    pub fn width(&self) -> u32 {
        self.width
    }

    /// 获取屏幕高度
    pub fn height(&self) -> u32 {
        self.height
    }

    /// 设置屏幕分辨率
    pub fn set_resolution(&mut self, width: u32, height: u32) {
        self.width = width.max(1);
        self.height = height.max(1);
    }

    /// 获取亮度
    pub fn brightness(&self) -> f32 {
        self.brightness
    }

    /// 设置亮度
    pub fn set_brightness(&mut self, brightness: f32) {
        self.brightness = brightness.clamp(0.0, 1.0);
    }

    /// 获取帧率
    pub fn frame_rate(&self) -> u32 {
        self.frame_rate
    }

    /// 设置帧率
    pub fn set_frame_rate(&mut self, frame_rate: u32) {
        self.frame_rate = frame_rate.max(1);
    }

    /// 检查画面是否冻结
    pub fn is_frozen(&self) -> bool {
        self.frozen
    }

    /// 冻结画面更新
    pub fn freeze(&mut self) {
        self.frozen = true;
    }

    /// 解冻画面更新
    pub fn unfreeze(&mut self) {
        self.frozen = false;
    }

    /// 获取帧计数
    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }

    /// 重置帧计数
    pub fn reset_frame_count(&mut self) {
        self.frame_count = 0;
    }

    /// 更新一帧
    pub fn update(&mut self) {
        if !self.frozen {
            self.frame_count += 1;
        }
    }

    /// 等待指定帧数
    pub fn wait(&mut self, duration: u32) {
        let target = self.frame_count + duration as u64;
        while self.frame_count < target {
            self.update();
        }
    }
}

impl Default for Graphics {
    fn default() -> Self {
        Self::new()
    }
}

/// 设置 Graphics 资源的系统
pub fn setup_graphics(mut commands: Commands) {
    commands.insert_resource(Graphics::default());
}

/// 更新 Graphics 资源的系统
pub fn update_graphics(mut graphics: ResMut<Graphics>) {
    graphics.update();
}
