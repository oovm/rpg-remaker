//! RPG Maker 地图数据类型

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use super::shared::AudioFile;
use super::event::RpgEvent;

/// RPG Maker 地图数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpgMap {
    /// 图块集 ID
    #[serde(rename = "@tileset_id")]
    #[serde(default)]
    pub tileset_id: i32,
    /// 地图宽度
    #[serde(rename = "@width")]
    #[serde(default)]
    pub width: i32,
    /// 地图高度
    #[serde(rename = "@height")]
    #[serde(default)]
    pub height: i32,
    /// 是否自动播放 BGM
    #[serde(rename = "@autoplay_bgm")]
    #[serde(default)]
    pub autoplay_bgm: bool,
    /// BGM
    #[serde(rename = "@bgm")]
    #[serde(default)]
    pub bgm: AudioFile,
    /// 是否自动播放 BGS
    #[serde(rename = "@autoplay_bgs")]
    #[serde(default)]
    pub autoplay_bgs: bool,
    /// BGS
    #[serde(rename = "@bgs")]
    #[serde(default)]
    pub bgs: AudioFile,
    /// 遭遇列表
    #[serde(rename = "@encounter_list")]
    #[serde(default)]
    pub encounter_list: Vec<i32>,
    /// 遭遇步数
    #[serde(rename = "@encounter_step")]
    #[serde(default)]
    pub encounter_step: i32,
    /// 地图数据（Table 类型）
    #[serde(rename = "@data")]
    pub data: Option<String>,
    /// 事件列表
    #[serde(rename = "@events")]
    #[serde(default)]
    pub events: BTreeMap<i32, RpgEvent>,
}

impl Default for RpgMap {
    fn default() -> Self {
        Self {
            tileset_id: 1,
            width: 20,
            height: 15,
            autoplay_bgm: false,
            bgm: AudioFile::default(),
            autoplay_bgs: false,
            bgs: AudioFile::default(),
            encounter_list: Vec::new(),
            encounter_step: 1,
            data: None,
            events: BTreeMap::new(),
        }
    }
}
