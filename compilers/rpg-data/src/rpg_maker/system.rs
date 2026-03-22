//! RPG Maker 系统数据类型

use serde::{Deserialize, Serialize};
use super::shared::AudioFile;

/// RPG Maker 系统数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpgSystem {
    /// 魔法数字
    #[serde(rename = "@magic_number")]
    #[serde(default)]
    pub magic_number: i32,
    /// 队伍成员列表
    #[serde(rename = "@party_members")]
    #[serde(default)]
    pub party_members: Vec<i32>,
    /// 元素列表
    #[serde(rename = "@elements")]
    #[serde(default)]
    pub elements: Vec<String>,
    /// 开关列表
    #[serde(rename = "@switches")]
    #[serde(default)]
    pub switches: Vec<Option<String>>,
    /// 变量列表
    #[serde(rename = "@variables")]
    #[serde(default)]
    pub variables: Vec<Option<String>>,
    /// 窗口皮肤文件名
    #[serde(rename = "@windowskin_name")]
    #[serde(default)]
    pub windowskin_name: String,
    /// 标题画面文件名
    #[serde(rename = "@title_name")]
    #[serde(default)]
    pub title_name: String,
    /// 游戏结束画面文件名
    #[serde(rename = "@gameover_name")]
    #[serde(default)]
    pub gameover_name: String,
    /// 战斗转场文件名
    #[serde(rename = "@battle_transition")]
    #[serde(default)]
    pub battle_transition: String,
    /// 标题 BGM
    #[serde(rename = "@title_bgm")]
    #[serde(default)]
    pub title_bgm: AudioFile,
    /// 战斗 BGM
    #[serde(rename = "@battle_bgm")]
    #[serde(default)]
    pub battle_bgm: AudioFile,
    /// 战斗结束 ME
    #[serde(rename = "@battle_end_me")]
    #[serde(default)]
    pub battle_end_me: AudioFile,
    /// 游戏结束 ME
    #[serde(rename = "@gameover_me")]
    #[serde(default)]
    pub gameover_me: AudioFile,
    /// 光标 SE
    #[serde(rename = "@cursor_se")]
    #[serde(default)]
    pub cursor_se: AudioFile,
    /// 确定 SE
    #[serde(rename = "@decision_se")]
    #[serde(default)]
    pub decision_se: AudioFile,
    /// 取消 SE
    #[serde(rename = "@cancel_se")]
    #[serde(default)]
    pub cancel_se: AudioFile,
    /// 蜂鸣 SE
    #[serde(rename = "@buzzer_se")]
    #[serde(default)]
    pub buzzer_se: AudioFile,
    /// 装备 SE
    #[serde(rename = "@equip_se")]
    #[serde(default)]
    pub equip_se: AudioFile,
    /// 商店 SE
    #[serde(rename = "@shop_se")]
    #[serde(default)]
    pub shop_se: AudioFile,
    /// 存档 SE
    #[serde(rename = "@save_se")]
    #[serde(default)]
    pub save_se: AudioFile,
    /// 读档 SE
    #[serde(rename = "@load_se")]
    #[serde(default)]
    pub load_se: AudioFile,
    /// 战斗开始 SE
    #[serde(rename = "@battle_start_se")]
    #[serde(default)]
    pub battle_start_se: AudioFile,
    /// 逃跑 SE
    #[serde(rename = "@escape_se")]
    #[serde(default)]
    pub escape_se: AudioFile,
    /// 角色倒下 SE
    #[serde(rename = "@actor_collapse_se")]
    #[serde(default)]
    pub actor_collapse_se: AudioFile,
    /// 敌人倒下 SE
    #[serde(rename = "@enemy_collapse_se")]
    #[serde(default)]
    pub enemy_collapse_se: AudioFile,
    /// 词汇
    #[serde(rename = "@words")]
    #[serde(default)]
    pub words: RpgWords,
    /// 测试队伍 ID
    #[serde(rename = "@test_troop_id")]
    #[serde(default)]
    pub test_troop_id: i32,
    /// 起始地图 ID
    #[serde(rename = "@start_map_id")]
    #[serde(default)]
    pub start_map_id: i32,
    /// 起始 X 坐标
    #[serde(rename = "@start_x")]
    #[serde(default)]
    pub start_x: i32,
    /// 起始 Y 坐标
    #[serde(rename = "@start_y")]
    #[serde(default)]
    pub start_y: i32,
    /// 战斗背景文件名
    #[serde(rename = "@battleback_name")]
    #[serde(default)]
    pub battleback_name: String,
    /// 战斗图形文件名
    #[serde(rename = "@battler_name")]
    #[serde(default)]
    pub battler_name: String,
    /// 战斗图形色相
    #[serde(rename = "@battler_hue")]
    #[serde(default)]
    pub battler_hue: i32,
    /// 编辑地图 ID
    #[serde(rename = "@edit_map_id")]
    #[serde(default)]
    pub edit_map_id: i32,
}

impl Default for RpgSystem {
    fn default() -> Self {
        Self {
            magic_number: 0,
            party_members: vec![1],
            elements: Vec::new(),
            switches: Vec::new(),
            variables: Vec::new(),
            windowskin_name: String::new(),
            title_name: String::new(),
            gameover_name: String::new(),
            battle_transition: String::new(),
            title_bgm: AudioFile::default(),
            battle_bgm: AudioFile::default(),
            battle_end_me: AudioFile::default(),
            gameover_me: AudioFile::default(),
            cursor_se: AudioFile::default(),
            decision_se: AudioFile::default(),
            cancel_se: AudioFile::default(),
            buzzer_se: AudioFile::default(),
            equip_se: AudioFile::default(),
            shop_se: AudioFile::default(),
            save_se: AudioFile::default(),
            load_se: AudioFile::default(),
            battle_start_se: AudioFile::default(),
            escape_se: AudioFile::default(),
            actor_collapse_se: AudioFile::default(),
            enemy_collapse_se: AudioFile::default(),
            words: RpgWords::default(),
            test_troop_id: 1,
            start_map_id: 1,
            start_x: 0,
            start_y: 0,
            battleback_name: String::new(),
            battler_name: String::new(),
            battler_hue: 0,
            edit_map_id: 1,
        }
    }
}

/// 系统词汇
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpgWords {
    /// 金币
    #[serde(rename = "@gold")]
    #[serde(default = "default_gold")]
    pub gold: String,
    /// HP
    #[serde(rename = "@hp")]
    #[serde(default = "default_hp")]
    pub hp: String,
    /// SP
    #[serde(rename = "@sp")]
    #[serde(default = "default_sp")]
    pub sp: String,
    /// 力量
    #[serde(rename = "@str")]
    #[serde(default = "default_str")]
    pub str: String,
    /// 灵巧
    #[serde(rename = "@dex")]
    #[serde(default = "default_dex")]
    pub dex: String,
    /// 速度
    #[serde(rename = "@agi")]
    #[serde(default = "default_agi")]
    pub agi: String,
    /// 魔力
    #[serde(rename = "@int")]
    #[serde(default = "default_int")]
    pub int: String,
    /// 攻击力
    #[serde(rename = "@atk")]
    #[serde(default = "default_atk")]
    pub atk: String,
    /// 物理防御
    #[serde(rename = "@pdef")]
    #[serde(default = "default_pdef")]
    pub pdef: String,
    /// 魔法防御
    #[serde(rename = "@mdef")]
    #[serde(default = "default_mdef")]
    pub mdef: String,
    /// 武器
    #[serde(rename = "@weapon")]
    #[serde(default = "default_weapon")]
    pub weapon: String,
    /// 防具 1
    #[serde(rename = "@armor1")]
    #[serde(default = "default_armor1")]
    pub armor1: String,
    /// 防具 2
    #[serde(rename = "@armor2")]
    #[serde(default = "default_armor2")]
    pub armor2: String,
    /// 防具 3
    #[serde(rename = "@armor3")]
    #[serde(default = "default_armor3")]
    pub armor3: String,
    /// 防具 4
    #[serde(rename = "@armor4")]
    #[serde(default = "default_armor4")]
    pub armor4: String,
    /// 攻击
    #[serde(rename = "@attack")]
    #[serde(default = "default_attack")]
    pub attack: String,
    /// 技能
    #[serde(rename = "@skill")]
    #[serde(default = "default_skill")]
    pub skill: String,
    /// 防御
    #[serde(rename = "@guard")]
    #[serde(default = "default_guard")]
    pub guard: String,
    /// 物品
    #[serde(rename = "@item")]
    #[serde(default = "default_item")]
    pub item: String,
    /// 装备
    #[serde(rename = "@equip")]
    #[serde(default = "default_equip")]
    pub equip: String,
}

fn default_gold() -> String { "G".to_string() }
fn default_hp() -> String { "HP".to_string() }
fn default_sp() -> String { "SP".to_string() }
fn default_str() -> String { "STR".to_string() }
fn default_dex() -> String { "DEX".to_string() }
fn default_agi() -> String { "AGI".to_string() }
fn default_int() -> String { "INT".to_string() }
fn default_atk() -> String { "ATK".to_string() }
fn default_pdef() -> String { "PDEF".to_string() }
fn default_mdef() -> String { "MDEF".to_string() }
fn default_weapon() -> String { "Weapon".to_string() }
fn default_armor1() -> String { "Shield".to_string() }
fn default_armor2() -> String { "Helmet".to_string() }
fn default_armor3() -> String { "Body Armor".to_string() }
fn default_armor4() -> String { "Accessory".to_string() }
fn default_attack() -> String { "Attack".to_string() }
fn default_skill() -> String { "Skill".to_string() }
fn default_guard() -> String { "Guard".to_string() }
fn default_item() -> String { "Item".to_string() }
fn default_equip() -> String { "Equip".to_string() }

impl Default for RpgWords {
    fn default() -> Self {
        Self {
            gold: default_gold(),
            hp: default_hp(),
            sp: default_sp(),
            str: default_str(),
            dex: default_dex(),
            agi: default_agi(),
            int: default_int(),
            atk: default_atk(),
            pdef: default_pdef(),
            mdef: default_mdef(),
            weapon: default_weapon(),
            armor1: default_armor1(),
            armor2: default_armor2(),
            armor3: default_armor3(),
            armor4: default_armor4(),
            attack: default_attack(),
            skill: default_skill(),
            guard: default_guard(),
            item: default_item(),
            equip: default_equip(),
        }
    }
}
