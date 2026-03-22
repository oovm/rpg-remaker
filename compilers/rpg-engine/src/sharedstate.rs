use bevy::prelude::*;
use std::time::Instant;

/// RGSS 版本枚举，表示 RPG Maker 的不同版本
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RgssVersion {
    /// RGSS 1.x (RPG Maker XP)
    V1,
    /// RGSS 2.x (RPG Maker VX)
    V2,
    /// RGSS 3.x (RPG Maker VX Ace)
    V3,
}

impl Default for RgssVersion {
    fn default() -> Self {
        RgssVersion::V3
    }
}

/// 全局共享状态资源，提供对核心系统组件的访问
#[derive(Resource)]
pub struct SharedState {
    /// RGSS 版本
    rgss_version: RgssVersion,
    /// 程序启动时间
    start_time: Instant,
    /// 当前帧时间戳
    frame_count: u64,
    /// 绑定数据（用于脚本绑定）
    binding_data: Option<Box<dyn std::any::Any + Send + Sync>>,
}

impl SharedState {
    /// 创建新的共享状态
    pub fn new(rgss_version: RgssVersion) -> Self {
        Self {
            rgss_version,
            start_time: Instant::now(),
            frame_count: 0,
            binding_data: None,
        }
    }

    /// 获取当前 RGSS 版本
    pub fn rgss_version(&self) -> RgssVersion {
        self.rgss_version
    }

    /// 获取程序运行时间（毫秒）
    pub fn run_time_ms(&self) -> u64 {
        self.start_time.elapsed().as_millis() as u64
    }

    /// 获取程序运行时间（秒）
    pub fn run_time_seconds(&self) -> f64 {
        self.start_time.elapsed().as_secs_f64()
    }

    /// 获取当前帧计数
    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }

    /// 增加帧计数
    pub fn increment_frame(&mut self) {
        self.frame_count += 1;
    }

    /// 生成时间戳
    pub fn gen_time_stamp(&self) -> u32 {
        self.frame_count as u32
    }

    /// 获取绑定数据
    pub fn binding_data(&self) -> Option<&(dyn std::any::Any + Send + Sync)> {
        self.binding_data.as_ref().map(|b| b.as_ref())
    }

    /// 设置绑定数据
    pub fn set_binding_data<T: 'static + Send + Sync>(&mut self, data: T) {
        self.binding_data = Some(Box::new(data));
    }

    /// 向下转型获取绑定数据
    pub fn get_binding_data<T: 'static>(&self) -> Option<&T> {
        self.binding_data
            .as_ref()
            .and_then(|b| b.downcast_ref::<T>())
    }
}

impl Default for SharedState {
    fn default() -> Self {
        Self::new(RgssVersion::default())
    }
}

/// 设置共享状态的系统
pub fn setup_shared_state(mut commands: Commands) {
    commands.insert_resource(SharedState::default());
}

/// 更新共享状态的系统（增加帧计数）
pub fn update_shared_state(mut shared_state: ResMut<SharedState>) {
    shared_state.increment_frame();
}
