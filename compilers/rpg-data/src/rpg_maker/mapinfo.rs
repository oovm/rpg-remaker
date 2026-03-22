//! RPG Maker 地图信息数据类型

use serde::{Deserialize, Serialize};

/// RPG Maker 地图信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RpgMapInfo {
    /// 地图名称
    #[serde(rename = "@name")]
    #[serde(default)]
    pub name: String,
    /// 父地图 ID
    #[serde(rename = "@parent_id")]
    #[serde(default)]
    pub parent_id: i32,
    /// 排序顺序
    #[serde(rename = "@order")]
    #[serde(default)]
    pub order: i32,
    /// 是否展开
    #[serde(rename = "@expanded")]
    #[serde(default)]
    pub expanded: bool,
    /// 滚动 X
    #[serde(rename = "@scroll_x")]
    #[serde(default = "default_scroll_x")]
    pub scroll_x: i32,
    /// 滚动 Y
    #[serde(rename = "@scroll_y")]
    #[serde(default = "default_scroll_y")]
    pub scroll_y: i32,
}

fn default_scroll_x() -> i32 { 320 }
fn default_scroll_y() -> i32 { 240 }

impl Default for RpgMapInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            parent_id: 0,
            order: 0,
            expanded: false,
            scroll_x: default_scroll_x(),
            scroll_y: default_scroll_y(),
        }
    }
}

impl PartialOrd for RpgMapInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RpgMapInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order.cmp(&other.order)
    }
}
