//! RPG Maker 图块集数据类型

use serde::{Deserialize, Serialize};
use super::shared::BlendMode;

/// RPG Maker 图块集数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpgTileset {
    /// 图块集 ID
    #[serde(rename = "@id")]
    pub id: i32,
    /// 图块集名称
    #[serde(rename = "@name")]
    #[serde(default)]
    pub name: String,
    /// 图块集文件名
    #[serde(rename = "@tileset_name")]
    #[serde(default)]
    pub tileset_name: String,
    /// 自动图块文件名列表
    #[serde(rename = "@autotile_names")]
    #[serde(default)]
    pub autotile_names: Vec<String>,
    /// 全景图文件名
    #[serde(rename = "@panorama_name")]
    #[serde(default)]
    pub panorama_name: String,
    /// 全景图色相
    #[serde(rename = "@panorama_hue")]
    #[serde(default)]
    pub panorama_hue: i32,
    /// 雾效文件名
    #[serde(rename = "@fog_name")]
    #[serde(default)]
    pub fog_name: String,
    /// 雾效色相
    #[serde(rename = "@fog_hue")]
    #[serde(default)]
    pub fog_hue: i32,
    /// 雾效透明度
    #[serde(rename = "@fog_opacity")]
    #[serde(default)]
    pub fog_opacity: i32,
    /// 雾效混合模式
    #[serde(rename = "@fog_blend_type")]
    #[serde(default)]
    pub fog_blend_type: i32,
    /// 雾效缩放
    #[serde(rename = "@fog_zoom")]
    #[serde(default)]
    pub fog_zoom: i32,
    /// 雾效 X 速度
    #[serde(rename = "@fog_sx")]
    #[serde(default)]
    pub fog_sx: i32,
    /// 雾效 Y 速度
    #[serde(rename = "@fog_sy")]
    #[serde(default)]
    pub fog_sy: i32,
    /// 战斗背景文件名
    #[serde(rename = "@battleback_name")]
    #[serde(default)]
    pub battleback_name: String,
    /// 通行表（Table 类型）
    #[serde(rename = "@passages")]
    pub passages: Option<String>,
    /// 优先级表（Table 类型）
    #[serde(rename = "@priorities")]
    pub priorities: Option<String>,
    /// 地形标签表（Table 类型）
    #[serde(rename = "@terrain_tags")]
    pub terrain_tags: Option<String>,
}

impl Default for RpgTileset {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            tileset_name: String::new(),
            autotile_names: vec![String::new(); 7],
            panorama_name: String::new(),
            panorama_hue: 0,
            fog_name: String::new(),
            fog_hue: 0,
            fog_opacity: 64,
            fog_blend_type: BlendMode::Normal as i32,
            fog_zoom: 200,
            fog_sx: 0,
            fog_sy: 0,
            battleback_name: String::new(),
            passages: None,
            priorities: None,
            terrain_tags: None,
        }
    }
}
