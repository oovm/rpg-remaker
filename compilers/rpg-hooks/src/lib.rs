#![warn(missing_docs)]

//! RPG Hacker - RPG Maker 游戏修改器后端
//! 
//! 提供 RPG Maker 游戏的修改、编辑和管理功能，支持多种 RPG Maker 版本。

mod parser;
mod resource;
mod data;
mod hook;

use std::error::Error;
use std::fmt;

pub use parser::*;
pub use resource::*;
pub use data::*;

/// 通用错误类型
#[derive(Debug)]
pub enum RpgHackerError {
    /// 未实现的功能
    NotImplemented(String),
    /// 资源未找到
    NotFound(String),
    /// 无效参数
    InvalidArgument(String),
    /// 内部错误
    InternalError(String),
    /// 文件格式错误
    InvalidFormat(String),
    /// 版本不兼容
    IncompatibleVersion(String),
}

impl fmt::Display for RpgHackerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RpgHackerError::NotImplemented(msg) => write!(f, "Not implemented: {}", msg),
            RpgHackerError::NotFound(msg) => write!(f, "Not found: {}", msg),
            RpgHackerError::InvalidArgument(msg) => write!(f, "Invalid argument: {}", msg),
            RpgHackerError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            RpgHackerError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            RpgHackerError::IncompatibleVersion(msg) => write!(f, "Incompatible version: {}", msg),
        }
    }
}

impl Error for RpgHackerError {}

/// 通用结果类型
pub type Result<T> = std::result::Result<T, RpgHackerError>;

/// RPG Maker 版本枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RpgMakerVersion {
    /// RPG Maker 2000
    Rpg2000,
    /// RPG Maker 2003
    Rpg2003,
    /// RPG Maker VX
    RpgVx,
    /// RPG Maker VX Ace
    RpgVxAce,
    /// 未知版本
    Unknown,
}

/// 游戏文件结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GameFile {
    /// 文件路径
    pub path: String,
    /// RPG Maker 版本
    pub version: RpgMakerVersion,
    /// 游戏标题
    pub title: String,
    /// 游戏数据
    pub data: GameData,
    /// 资源列表
    pub resources: Vec<Resource>,
}

/// 游戏数据结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GameData {
    /// 角色数据
    pub actors: Vec<Actor>,
    /// 物品数据
    pub items: Vec<Item>,
    /// 技能数据
    pub skills: Vec<Skill>,
    /// 敌人数据
    pub enemies: Vec<Enemy>,
    /// 地图数据
    pub maps: Vec<Map>,
    /// 事件数据
    pub events: Vec<Event>,
    /// 系统数据
    pub system: SystemData,
}

/// 角色结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Actor {
    /// 角色 ID
    pub id: u32,
    /// 角色名称
    pub name: String,
    /// 角色等级
    pub level: u32,
    /// 角色属性
    pub attributes: ActorAttributes,
    /// 角色技能
    pub skills: Vec<u32>,
}

/// 角色属性结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActorAttributes {
    /// 最大生命值
    pub max_hp: u32,
    /// 最大魔法值
    pub max_mp: u32,
    /// 攻击力
    pub attack: u32,
    /// 防御力
    pub defense: u32,
    /// 魔法攻击力
    pub magic_attack: u32,
    /// 魔法防御力
    pub magic_defense: u32,
    /// 敏捷度
    pub agility: u32,
    /// 幸运值
    pub luck: u32,
}

/// 物品结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Item {
    /// 物品 ID
    pub id: u32,
    /// 物品名称
    pub name: String,
    /// 物品类型
    pub item_type: ItemType,
    /// 物品效果
    pub effect: ItemEffect,
    /// 物品描述
    pub description: String,
}

/// 物品类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemType {
    /// 普通物品
    Normal,
    /// 武器
    Weapon,
    /// 防具
    Armor,
    /// 道具
    KeyItem,
}

/// 物品效果结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ItemEffect {
    /// 效果类型
    pub effect_type: EffectType,
    /// 效果值
    pub value: i32,
    /// 目标类型
    pub target_type: TargetType,
}

/// 效果类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectType {
    /// 恢复生命值
    RecoverHp,
    /// 恢复魔法值
    RecoverMp,
    /// 增加攻击力
    IncreaseAttack,
    /// 增加防御力
    IncreaseDefense,
    /// 增加魔法攻击力
    IncreaseMagicAttack,
    /// 增加魔法防御力
    IncreaseMagicDefense,
    /// 增加敏捷度
    IncreaseAgility,
    /// 增加幸运值
    IncreaseLuck,
    /// 施加状态
    ApplyState,
    /// 解除状态
    RemoveState,
}

/// 目标类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetType {
    /// 单个角色
    SingleActor,
    /// 所有角色
    AllActors,
    /// 单个敌人
    SingleEnemy,
    /// 所有敌人
    AllEnemies,
    /// 使用者
    User,
}

/// 技能结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Skill {
    /// 技能 ID
    pub id: u32,
    /// 技能名称
    pub name: String,
    /// 技能消耗
    pub cost: u32,
    /// 技能效果
    pub effect: SkillEffect,
    /// 技能描述
    pub description: String,
}

/// 技能效果结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SkillEffect {
    /// 效果类型
    pub effect_type: EffectType,
    /// 效果值
    pub value: i32,
    /// 目标类型
    pub target_type: TargetType,
    /// 命中率
    pub hit_rate: f32,
    /// 魔法消耗
    pub mp_cost: u32,
}

/// 敌人结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Enemy {
    /// 敌人 ID
    pub id: u32,
    /// 敌人名称
    pub name: String,
    /// 敌人等级
    pub level: u32,
    /// 敌人属性
    pub attributes: EnemyAttributes,
    /// 敌人技能
    pub skills: Vec<u32>,
    /// 敌人掉落物品
    pub drops: Vec<DropItem>,
}

/// 敌人属性结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EnemyAttributes {
    /// 最大生命值
    pub max_hp: u32,
    /// 最大魔法值
    pub max_mp: u32,
    /// 攻击力
    pub attack: u32,
    /// 防御力
    pub defense: u32,
    /// 魔法攻击力
    pub magic_attack: u32,
    /// 魔法防御力
    pub magic_defense: u32,
    /// 敏捷度
    pub agility: u32,
    /// 经验值
    pub exp: u32,
    /// 金钱
    pub gold: u32,
}

/// 掉落物品结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DropItem {
    /// 物品 ID
    pub item_id: u32,
    /// 掉落概率
    pub probability: f32,
}

/// 地图结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Map {
    /// 地图 ID
    pub id: u32,
    /// 地图名称
    pub name: String,
    /// 地图宽度
    pub width: u32,
    /// 地图高度
    pub height: u32,
    /// 地图数据
    pub data: Vec<Vec<u16>>,
    /// 地图事件
    pub events: Vec<Event>,
}

/// 事件结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Event {
    /// 事件 ID
    pub id: u32,
    /// 事件名称
    pub name: String,
    /// 事件位置
    pub position: (u32, u32),
    /// 事件页面
    pub pages: Vec<EventPage>,
}

/// 事件页面结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EventPage {
    /// 页面条件
    pub conditions: EventConditions,
    /// 事件指令
    pub commands: Vec<EventCommand>,
}

/// 事件条件结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EventConditions {
    /// 开关条件
    pub switch_id: Option<u32>,
    /// 变量条件
    pub variable_id: Option<u32>,
    /// 变量值
    pub variable_value: Option<i32>,
    /// 独立开关条件
    pub self_switch: Option<char>,
    /// 计时器条件
    pub timer: Option<u32>,
    /// 角色条件
    pub actor_id: Option<u32>,
}

/// 事件指令结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EventCommand {
    /// 指令类型
    pub command_type: CommandType,
    /// 指令参数
    pub parameters: Vec<String>,
}

/// 指令类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandType {
    /// 显示文本
    ShowText,
    /// 显示选择
    ShowChoices,
    /// 设置变量
    SetVariable,
    /// 开关操作
    ControlSwitch,
    /// 更改角色
    ChangeActor,
    /// 更改物品
    ChangeItem,
    /// 更改技能
    ChangeSkill,
    /// 战斗处理
    BattleProcessing,
    /// 场景转移
    SceneTransfer,
    /// 等待
    Wait,
    /// 条件分支
    ConditionalBranch,
    /// 循环
    Loop,
    /// 跳出循环
    BreakLoop,
    /// 标签
    Label,
    /// 跳转
    GotoLabel,
    /// 公共事件
    CommonEvent,
    /// 脚本
    Script,
}

/// 系统数据结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemData {
    /// 游戏标题
    pub game_title: String,
    /// 游戏版本
    pub game_version: String,
    /// 系统设置
    pub system_settings: SystemSettings,
    /// 数据库设置
    pub database_settings: DatabaseSettings,
}

/// 系统设置结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemSettings {
    /// 窗口皮肤
    pub window_skin: String,
    /// 战斗背景
    pub battle_background: String,
    /// 战斗 BGM
    pub battle_bgm: String,
    /// 战斗结束 BGM
    pub battle_end_bgm: String,
    /// 游戏结束 BGM
    pub game_over_bgm: String,
    /// 菜单 BGM
    pub menu_bgm: String,
    /// 系统声音
    pub system_sounds: SystemSounds,
}

/// 系统声音结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemSounds {
    /// 确定声音
    pub ok: String,
    /// 取消声音
    pub cancel: String,
    /// 按钮声音
    pub buzzer: String,
    /// 装备声音
    pub equip: String,
    /// 战斗开始声音
    pub battle_start: String,
    /// 逃跑声音
    pub escape: String,
    /// 敌人攻击声音
    pub enemy_attack: String,
    /// 敌人伤害声音
    pub enemy_damage: String,
    /// 角色攻击声音
    pub actor_attack: String,
    /// 角色伤害声音
    pub actor_damage: String,
    /// 角色恢复声音
    pub recover: String,
    /// 技能使用声音
    pub skill: String,
    /// 物品使用声音
    pub item: String,
}

/// 数据库设置结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DatabaseSettings {
    /// 最大角色数
    pub max_actors: u32,
    /// 最大物品数
    pub max_items: u32,
    /// 最大技能数
    pub max_skills: u32,
    /// 最大敌人数
    pub max_enemies: u32,
    /// 最大地图数
    pub max_maps: u32,
    /// 最大事件数
    pub max_events: u32,
}

/// 资源结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Resource {
    /// 资源路径
    pub path: String,
    /// 资源类型
    pub resource_type: ResourceType,
    /// 资源大小
    pub size: u64,
    /// 资源内容（可选）
    pub content: Option<Vec<u8>>,
}

/// 资源类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceType {
    /// 图像资源
    Image,
    /// 音频资源
    Audio,
    /// 文本资源
    Text,
    /// 脚本资源
    Script,
    /// 其他资源
    Other,
}

/// 脚本结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Script {
    /// 脚本 ID
    pub id: u32,
    /// 脚本名称
    pub name: String,
    /// 脚本内容
    pub content: String,
}

/// RPG Hacker 服务 trait
#[async_trait::async_trait]
pub trait RpgHackerService: Send + Sync {
    /// 解析游戏文件
    async fn parse_game_file(&self, file_path: &str) -> Result<GameFile>;
    
    /// 保存游戏文件
    async fn save_game_file(&self, game_file: &GameFile, output_path: &str) -> Result<()>;
    
    /// 获取游戏资源
    async fn get_resource(&self, game_file: &GameFile, resource_path: &str) -> Result<Resource>;
    
    /// 添加资源
    async fn add_resource(&self, game_file: &mut GameFile, resource: Resource) -> Result<()>;
    
    /// 删除资源
    async fn remove_resource(&self, game_file: &mut GameFile, resource_path: &str) -> Result<()>;
    
    /// 修改游戏数据
    async fn modify_game_data(&self, game_file: &mut GameFile, modification: GameDataModification) -> Result<()>;
    
    /// 编辑脚本
    async fn edit_script(&self, game_file: &mut GameFile, script_id: u32, content: &str) -> Result<()>;
    
    /// 导出游戏文件
    async fn export_game_file(&self, game_file: &GameFile, output_path: &str, version: RpgMakerVersion) -> Result<()>;
    
    /// 检查版本兼容性
    async fn check_version_compatibility(&self, game_file: &GameFile, target_version: RpgMakerVersion) -> Result<bool>;
    
    /// 识别游戏版本
    async fn identify_game_version(&self, file_path: &str) -> Result<RpgMakerVersion>;
}

/// 游戏数据修改结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GameDataModification {
    /// 角色修改
    pub actor_modifications: Option<Vec<ActorModification>>,
    /// 物品修改
    pub item_modifications: Option<Vec<ItemModification>>,
    /// 技能修改
    pub skill_modifications: Option<Vec<SkillModification>>,
    /// 敌人修改
    pub enemy_modifications: Option<Vec<EnemyModification>>,
    /// 地图修改
    pub map_modifications: Option<Vec<MapModification>>,
    /// 事件修改
    pub event_modifications: Option<Vec<EventModification>>,
    /// 系统数据修改
    pub system_modification: Option<SystemModification>,
}

/// 角色修改结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActorModification {
    /// 角色 ID
    pub id: u32,
    /// 角色名称
    pub name: Option<String>,
    /// 角色等级
    pub level: Option<u32>,
    /// 角色属性
    pub attributes: Option<ActorAttributes>,
    /// 角色技能
    pub skills: Option<Vec<u32>>,
}

/// 物品修改结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ItemModification {
    /// 物品 ID
    pub id: u32,
    /// 物品名称
    pub name: Option<String>,
    /// 物品类型
    pub item_type: Option<ItemType>,
    /// 物品效果
    pub effect: Option<ItemEffect>,
    /// 物品描述
    pub description: Option<String>,
}

/// 技能修改结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SkillModification {
    /// 技能 ID
    pub id: u32,
    /// 技能名称
    pub name: Option<String>,
    /// 技能消耗
    pub cost: Option<u32>,
    /// 技能效果
    pub effect: Option<SkillEffect>,
    /// 技能描述
    pub description: Option<String>,
}

/// 敌人修改结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EnemyModification {
    /// 敌人 ID
    pub id: u32,
    /// 敌人名称
    pub name: Option<String>,
    /// 敌人等级
    pub level: Option<u32>,
    /// 敌人属性
    pub attributes: Option<EnemyAttributes>,
    /// 敌人技能
    pub skills: Option<Vec<u32>>,
    /// 敌人掉落物品
    pub drops: Option<Vec<DropItem>>,
}

/// 地图修改结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MapModification {
    /// 地图 ID
    pub id: u32,
    /// 地图名称
    pub name: Option<String>,
    /// 地图宽度
    pub width: Option<u32>,
    /// 地图高度
    pub height: Option<u32>,
    /// 地图数据
    pub data: Option<Vec<Vec<u16>>>,
}

/// 事件修改结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EventModification {
    /// 事件 ID
    pub id: u32,
    /// 事件名称
    pub name: Option<String>,
    /// 事件位置
    pub position: Option<(u32, u32)>,
    /// 事件页面
    pub pages: Option<Vec<EventPage>>,
}

/// 系统修改结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemModification {
    /// 游戏标题
    pub game_title: Option<String>,
    /// 游戏版本
    pub game_version: Option<String>,
    /// 系统设置
    pub system_settings: Option<SystemSettings>,
    /// 数据库设置
    pub database_settings: Option<DatabaseSettings>,
}

/// 内存 RPG Hacker 服务实现
pub struct MemoryRpgHackerService;

impl MemoryRpgHackerService {
    /// 创建新的内存 RPG Hacker 服务
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl RpgHackerService for MemoryRpgHackerService {
    async fn parse_game_file(&self, _file_path: &str) -> Result<GameFile> {
        Err(RpgHackerError::NotImplemented("parse_game_file not implemented".to_string()))
    }
    
    async fn save_game_file(&self, _game_file: &GameFile, _output_path: &str) -> Result<()> {
        Err(RpgHackerError::NotImplemented("save_game_file not implemented".to_string()))
    }
    
    async fn get_resource(&self, game_file: &GameFile, resource_path: &str) -> Result<Resource> {
        // 在资源列表中查找资源
        for resource in &game_file.resources {
            if resource.path == resource_path {
                return Ok(resource.clone());
            }
        }
        Err(RpgHackerError::NotFound(format!("Resource not found: {}", resource_path)))
    }
    
    async fn add_resource(&self, game_file: &mut GameFile, resource: Resource) -> Result<()> {
        // 检查资源是否已存在
        for existing_resource in &game_file.resources {
            if existing_resource.path == resource.path {
                return Err(RpgHackerError::InvalidArgument(format!("Resource already exists: {}", resource.path)));
            }
        }
        
        // 添加资源
        game_file.resources.push(resource);
        Ok(())
    }
    
    async fn remove_resource(&self, game_file: &mut GameFile, resource_path: &str) -> Result<()> {
        // 查找并删除资源
        let index = game_file.resources
            .iter()
            .position(|r| r.path == resource_path);
        
        match index {
            Some(i) => {
                game_file.resources.remove(i);
                Ok(())
            }
            None => Err(RpgHackerError::NotFound(format!("Resource not found: {}", resource_path))),
        }
    }
    
    async fn modify_game_data(&self, _game_file: &mut GameFile, _modification: GameDataModification) -> Result<()> {
        Err(RpgHackerError::NotImplemented("modify_game_data not implemented".to_string()))
    }
    
    async fn edit_script(&self, _game_file: &mut GameFile, _script_id: u32, _content: &str) -> Result<()> {
        Err(RpgHackerError::NotImplemented("edit_script not implemented".to_string()))
    }
    
    async fn export_game_file(&self, _game_file: &GameFile, _output_path: &str, _version: RpgMakerVersion) -> Result<()> {
        Err(RpgHackerError::NotImplemented("export_game_file not implemented".to_string()))
    }
    
    async fn check_version_compatibility(&self, _game_file: &GameFile, _target_version: RpgMakerVersion) -> Result<bool> {
        Err(RpgHackerError::NotImplemented("check_version_compatibility not implemented".to_string()))
    }
    
    async fn identify_game_version(&self, file_path: &str) -> Result<RpgMakerVersion> {
        // 基于文件路径和扩展名识别游戏版本
        let path = std::path::Path::new(file_path);
        
        // 检查文件扩展名
        if let Some(extension) = path.extension() {
            let ext = extension.to_str().unwrap_or("");
            match ext.to_lowercase().as_str() {
                "exe" => {
                    // 检查文件名
                    if let Some(file_name) = path.file_name() {
                        let name = file_name.to_str().unwrap_or("");
                        if name.to_lowercase().contains("rpg2000") {
                            return Ok(RpgMakerVersion::Rpg2000);
                        } else if name.to_lowercase().contains("rpg2003") {
                            return Ok(RpgMakerVersion::Rpg2003);
                        } else if name.to_lowercase().contains("rgss2") || name.to_lowercase().contains("rpgvx") {
                            return Ok(RpgMakerVersion::RpgVx);
                        } else if name.to_lowercase().contains("rgss3") || name.to_lowercase().contains("rpgvxace") {
                            return Ok(RpgMakerVersion::RpgVxAce);
                        }
                    }
                }
                "rvproj" => {
                    return Ok(RpgMakerVersion::RpgVx);
                }
                "rvproj2" => {
                    return Ok(RpgMakerVersion::RpgVxAce);
                }
                _ => {}
            }
        }
        
        // 检查目录结构
        if path.is_dir() {
            // 检查是否存在 RGSS.dll, RGSS2.dll 或 RGSS3.dll
            let rgss_dll = path.join("RGSS.dll");
            let rgss2_dll = path.join("RGSS2.dll");
            let rgss3_dll = path.join("RGSS3.dll");
            
            if rgss_dll.exists() {
                return Ok(RpgMakerVersion::Rpg2003);
            } else if rgss2_dll.exists() {
                return Ok(RpgMakerVersion::RpgVx);
            } else if rgss3_dll.exists() {
                return Ok(RpgMakerVersion::RpgVxAce);
            }
        }
        
        Ok(RpgMakerVersion::Unknown)
    }
}

impl Default for MemoryRpgHackerService {
    fn default() -> Self {
        Self::new()
    }
}
