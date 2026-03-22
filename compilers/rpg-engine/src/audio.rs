use bevy::prelude::*;

/// 音频播放状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayState {
    /// 停止
    Stopped,
    /// 播放中
    Playing,
    /// 暂停
    Paused,
}

/// BGM 资源
#[derive(Resource, Default)]
pub struct Bgm {
    /// 当前播放的文件名
    filename: Option<String>,
    /// 音量（0.0 - 100.0）
    volume: f32,
    /// 音高（50 - 150）
    pitch: f32,
    /// 播放状态
    state: PlayState,
    /// 播放位置（毫秒）
    position: u64,
}

impl Bgm {
    /// 创建新的 BGM 资源
    pub fn new() -> Self {
        Self::default()
    }

    /// 播放 BGM
    pub fn play(&mut self, filename: &str, volume: f32, pitch: f32) {
        self.filename = Some(filename.to_string());
        self.volume = volume.clamp(0.0, 100.0);
        self.pitch = pitch.clamp(50.0, 150.0);
        self.state = PlayState::Playing;
        self.position = 0;
    }

    /// 停止 BGM
    pub fn stop(&mut self) {
        self.state = PlayState::Stopped;
        self.position = 0;
    }

    /// 暂停 BGM
    pub fn pause(&mut self) {
        if self.state == PlayState::Playing {
            self.state = PlayState::Paused;
        }
    }

    /// 恢复 BGM
    pub fn resume(&mut self) {
        if self.state == PlayState::Paused {
            self.state = PlayState::Playing;
        }
    }

    /// 淡出 BGM
    pub fn fade(&mut self, _duration: u32) {
        self.stop();
    }

    /// 获取当前音量
    pub fn volume(&self) -> f32 {
        self.volume
    }

    /// 获取当前音高
    pub fn pitch(&self) -> f32 {
        self.pitch
    }

    /// 获取播放状态
    pub fn state(&self) -> PlayState {
        self.state
    }

    /// 获取当前文件名
    pub fn filename(&self) -> Option<&str> {
        self.filename.as_deref()
    }
}

/// BGS 资源
#[derive(Resource, Default)]
pub struct Bgs {
    /// 当前播放的文件名
    filename: Option<String>,
    /// 音量（0.0 - 100.0）
    volume: f32,
    /// 音高（50 - 150）
    pitch: f32,
    /// 播放状态
    state: PlayState,
}

impl Bgs {
    /// 创建新的 BGS 资源
    pub fn new() -> Self {
        Self::default()
    }

    /// 播放 BGS
    pub fn play(&mut self, filename: &str, volume: f32, pitch: f32) {
        self.filename = Some(filename.to_string());
        self.volume = volume.clamp(0.0, 100.0);
        self.pitch = pitch.clamp(50.0, 150.0);
        self.state = PlayState::Playing;
    }

    /// 停止 BGS
    pub fn stop(&mut self) {
        self.state = PlayState::Stopped;
    }

    /// 淡出 BGS
    pub fn fade(&mut self, _duration: u32) {
        self.stop();
    }

    /// 获取当前音量
    pub fn volume(&self) -> f32 {
        self.volume
    }

    /// 获取当前音高
    pub fn pitch(&self) -> f32 {
        self.pitch
    }

    /// 获取播放状态
    pub fn state(&self) -> PlayState {
        self.state
    }
}

/// ME 资源
#[derive(Resource, Default)]
pub struct Me {
    /// 当前播放的文件名
    filename: Option<String>,
    /// 音量（0.0 - 100.0）
    volume: f32,
    /// 音高（50 - 150）
    pitch: f32,
    /// 播放状态
    state: PlayState,
}

impl Me {
    /// 创建新的 ME 资源
    pub fn new() -> Self {
        Self::default()
    }

    /// 播放 ME
    pub fn play(&mut self, filename: &str, volume: f32, pitch: f32) {
        self.filename = Some(filename.to_string());
        self.volume = volume.clamp(0.0, 100.0);
        self.pitch = pitch.clamp(50.0, 150.0);
        self.state = PlayState::Playing;
    }

    /// 停止 ME
    pub fn stop(&mut self) {
        self.state = PlayState::Stopped;
    }

    /// 淡出 ME
    pub fn fade(&mut self, _duration: u32) {
        self.stop();
    }

    /// 获取当前音量
    pub fn volume(&self) -> f32 {
        self.volume
    }

    /// 获取当前音高
    pub fn pitch(&self) -> f32 {
        self.pitch
    }

    /// 获取播放状态
    pub fn state(&self) -> PlayState {
        self.state
    }
}

/// SE 资源
#[derive(Resource, Default)]
pub struct Se {
    /// 主音量（0.0 - 100.0）
    master_volume: f32,
}

impl Se {
    /// 创建新的 SE 资源
    pub fn new() -> Self {
        Self {
            master_volume: 100.0,
        }
    }

    /// 播放 SE
    pub fn play(&mut self, _filename: &str, _volume: f32, _pitch: f32) {
    }

    /// 停止所有 SE
    pub fn stop(&mut self) {
    }

    /// 获取主音量
    pub fn master_volume(&self) -> f32 {
        self.master_volume
    }

    /// 设置主音量
    pub fn set_master_volume(&mut self, volume: f32) {
        self.master_volume = volume.clamp(0.0, 100.0);
    }
}

/// Audio 系统资源，管理所有音频相关功能
#[derive(Resource)]
pub struct Audio {
    /// BGM
    pub bgm: Bgm,
    /// BGS
    pub bgs: Bgs,
    /// ME
    pub me: Me,
    /// SE
    pub se: Se,
}

impl Audio {
    /// 创建新的 Audio 资源
    pub fn new() -> Self {
        Self {
            bgm: Bgm::new(),
            bgs: Bgs::new(),
            me: Me::new(),
            se: Se::new(),
        }
    }

    /// 播放 BGM
    pub fn bgm_play(&mut self, filename: &str, volume: f32, pitch: f32) {
        self.bgm.play(filename, volume, pitch);
    }

    /// 停止 BGM
    pub fn bgm_stop(&mut self) {
        self.bgm.stop();
    }

    /// 暂停 BGM
    pub fn bgm_pause(&mut self) {
        self.bgm.pause();
    }

    /// 恢复 BGM
    pub fn bgm_resume(&mut self) {
        self.bgm.resume();
    }

    /// 淡出 BGM
    pub fn bgm_fade(&mut self, duration: u32) {
        self.bgm.fade(duration);
    }

    /// 播放 BGS
    pub fn bgs_play(&mut self, filename: &str, volume: f32, pitch: f32) {
        self.bgs.play(filename, volume, pitch);
    }

    /// 停止 BGS
    pub fn bgs_stop(&mut self) {
        self.bgs.stop();
    }

    /// 淡出 BGS
    pub fn bgs_fade(&mut self, duration: u32) {
        self.bgs.fade(duration);
    }

    /// 播放 ME
    pub fn me_play(&mut self, filename: &str, volume: f32, pitch: f32) {
        self.me.play(filename, volume, pitch);
    }

    /// 停止 ME
    pub fn me_stop(&mut self) {
        self.me.stop();
    }

    /// 淡出 ME
    pub fn me_fade(&mut self, duration: u32) {
        self.me.fade(duration);
    }

    /// 播放 SE
    pub fn se_play(&mut self, filename: &str, volume: f32, pitch: f32) {
        self.se.play(filename, volume, pitch);
    }

    /// 停止所有 SE
    pub fn se_stop(&mut self) {
        self.se.stop();
    }
}

impl Default for Audio {
    fn default() -> Self {
        Self::new()
    }
}

/// 设置 Audio 资源的系统
pub fn setup_audio(mut commands: Commands) {
    commands.insert_resource(Audio::default());
}
