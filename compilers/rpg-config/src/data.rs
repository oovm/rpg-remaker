//! 游戏数据修改
//! 
//! 负责游戏数据的修改和管理。

use super::*;

/// 游戏数据修改器
pub struct GameDataModifier {
    /// 服务实例
    service: Box<dyn RpgHackerService>,
}

impl GameDataModifier {
    /// 创建新的游戏数据修改器
    pub fn new(service: Box<dyn RpgHackerService>) -> Self {
        Self {
            service,
        }
    }
    
    /// 修改游戏数据
    pub async fn modify_game_data(&self, game_file: &mut GameFile, modification: GameDataModification) -> Result<()> {
        self.service.modify_game_data(game_file, modification).await
    }
    
    /// 修改角色数据
    pub async fn modify_actor(&self, game_file: &mut GameFile, modification: ActorModification) -> Result<()> {
        // 查找角色
        if let Some(actor) = game_file.data.actors.iter_mut().find(|a| a.id == modification.id) {
            // 修改角色数据
            if let Some(name) = modification.name {
                actor.name = name;
            }
            if let Some(level) = modification.level {
                actor.level = level;
            }
            if let Some(attributes) = modification.attributes {
                actor.attributes = attributes;
            }
            if let Some(skills) = modification.skills {
                actor.skills = skills;
            }
            Ok(())
        } else {
            Err(RpgHackerError::NotFound(format!("Actor not found: {}", modification.id)))
        }
    }
    
    /// 修改物品数据
    pub async fn modify_item(&self, game_file: &mut GameFile, modification: ItemModification) -> Result<()> {
        // 查找物品
        if let Some(item) = game_file.data.items.iter_mut().find(|i| i.id == modification.id) {
            // 修改物品数据
            if let Some(name) = modification.name {
                item.name = name;
            }
            if let Some(item_type) = modification.item_type {
                item.item_type = item_type;
            }
            if let Some(effect) = modification.effect {
                item.effect = effect;
            }
            if let Some(description) = modification.description {
                item.description = description;
            }
            Ok(())
        } else {
            Err(RpgHackerError::NotFound(format!("Item not found: {}", modification.id)))
        }
    }
    
    /// 修改技能数据
    pub async fn modify_skill(&self, game_file: &mut GameFile, modification: SkillModification) -> Result<()> {
        // 查找技能
        if let Some(skill) = game_file.data.skills.iter_mut().find(|s| s.id == modification.id) {
            // 修改技能数据
            if let Some(name) = modification.name {
                skill.name = name;
            }
            if let Some(cost) = modification.cost {
                skill.cost = cost;
            }
            if let Some(effect) = modification.effect {
                skill.effect = effect;
            }
            if let Some(description) = modification.description {
                skill.description = description;
            }
            Ok(())
        } else {
            Err(RpgHackerError::NotFound(format!("Skill not found: {}", modification.id)))
        }
    }
    
    /// 修改敌人数据
    pub async fn modify_enemy(&self, game_file: &mut GameFile, modification: EnemyModification) -> Result<()> {
        // 查找敌人
        if let Some(enemy) = game_file.data.enemies.iter_mut().find(|e| e.id == modification.id) {
            // 修改敌人数据
            if let Some(name) = modification.name {
                enemy.name = name;
            }
            if let Some(level) = modification.level {
                enemy.level = level;
            }
            if let Some(attributes) = modification.attributes {
                enemy.attributes = attributes;
            }
            if let Some(skills) = modification.skills {
                enemy.skills = skills;
            }
            if let Some(drops) = modification.drops {
                enemy.drops = drops;
            }
            Ok(())
        } else {
            Err(RpgHackerError::NotFound(format!("Enemy not found: {}", modification.id)))
        }
    }
    
    /// 修改地图数据
    pub async fn modify_map(&self, game_file: &mut GameFile, modification: MapModification) -> Result<()> {
        // 查找地图
        if let Some(map) = game_file.data.maps.iter_mut().find(|m| m.id == modification.id) {
            // 修改地图数据
            if let Some(name) = modification.name {
                map.name = name;
            }
            if let Some(width) = modification.width {
                map.width = width;
            }
            if let Some(height) = modification.height {
                map.height = height;
            }
            if let Some(data) = modification.data {
                map.data = data;
            }
            Ok(())
        } else {
            Err(RpgHackerError::NotFound(format!("Map not found: {}", modification.id)))
        }
    }
    
    /// 修改事件数据
    pub async fn modify_event(&self, game_file: &mut GameFile, modification: EventModification) -> Result<()> {
        // 查找事件
        if let Some(event) = game_file.data.events.iter_mut().find(|e| e.id == modification.id) {
            // 修改事件数据
            if let Some(name) = modification.name {
                event.name = name;
            }
            if let Some(position) = modification.position {
                event.position = position;
            }
            if let Some(pages) = modification.pages {
                event.pages = pages;
            }
            Ok(())
        } else {
            // 尝试在地图中查找事件
            for map in &mut game_file.data.maps {
                if let Some(event) = map.events.iter_mut().find(|e| e.id == modification.id) {
                    // 修改事件数据
                    if let Some(name) = modification.name {
                        event.name = name;
                    }
                    if let Some(position) = modification.position {
                        event.position = position;
                    }
                    if let Some(pages) = modification.pages {
                        event.pages = pages;
                    }
                    return Ok(());
                }
            }
            Err(RpgHackerError::NotFound(format!("Event not found: {}", modification.id)))
        }
    }
    
    /// 修改系统数据
    pub async fn modify_system(&self, game_file: &mut GameFile, modification: SystemModification) -> Result<()> {
        // 修改系统数据
        if let Some(game_title) = modification.game_title {
            game_file.data.system.game_title = game_title;
        }
        if let Some(game_version) = modification.game_version {
            game_file.data.system.game_version = game_version;
        }
        if let Some(system_settings) = modification.system_settings {
            game_file.data.system.system_settings = system_settings;
        }
        if let Some(database_settings) = modification.database_settings {
            game_file.data.system.database_settings = database_settings;
        }
        Ok(())
    }
    
    /// 添加角色
    pub async fn add_actor(&self, game_file: &mut GameFile, actor: Actor) -> Result<()> {
        // 检查角色 ID 是否已存在
        if game_file.data.actors.iter().any(|a| a.id == actor.id) {
            return Err(RpgHackerError::InvalidArgument(format!("Actor ID already exists: {}", actor.id)));
        }
        
        // 添加角色
        game_file.data.actors.push(actor);
        Ok(())
    }
    
    /// 删除角色
    pub async fn remove_actor(&self, game_file: &mut GameFile, actor_id: u32) -> Result<()> {
        // 查找并删除角色
        let index = game_file.data.actors
            .iter()
            .position(|a| a.id == actor_id);
        
        match index {
            Some(i) => {
                game_file.data.actors.remove(i);
                Ok(())
            }
            None => Err(RpgHackerError::NotFound(format!("Actor not found: {}", actor_id))),
        }
    }
    
    /// 添加物品
    pub async fn add_item(&self, game_file: &mut GameFile, item: Item) -> Result<()> {
        // 检查物品 ID 是否已存在
        if game_file.data.items.iter().any(|i| i.id == item.id) {
            return Err(RpgHackerError::InvalidArgument(format!("Item ID already exists: {}", item.id)));
        }
        
        // 添加物品
        game_file.data.items.push(item);
        Ok(())
    }
    
    /// 删除物品
    pub async fn remove_item(&self, game_file: &mut GameFile, item_id: u32) -> Result<()> {
        // 查找并删除物品
        let index = game_file.data.items
            .iter()
            .position(|i| i.id == item_id);
        
        match index {
            Some(i) => {
                game_file.data.items.remove(i);
                Ok(())
            }
            None => Err(RpgHackerError::NotFound(format!("Item not found: {}", item_id))),
        }
    }
    
    /// 添加技能
    pub async fn add_skill(&self, game_file: &mut GameFile, skill: Skill) -> Result<()> {
        // 检查技能 ID 是否已存在
        if game_file.data.skills.iter().any(|s| s.id == skill.id) {
            return Err(RpgHackerError::InvalidArgument(format!("Skill ID already exists: {}", skill.id)));
        }
        
        // 添加技能
        game_file.data.skills.push(skill);
        Ok(())
    }
    
    /// 删除技能
    pub async fn remove_skill(&self, game_file: &mut GameFile, skill_id: u32) -> Result<()> {
        // 查找并删除技能
        let index = game_file.data.skills
            .iter()
            .position(|s| s.id == skill_id);
        
        match index {
            Some(i) => {
                game_file.data.skills.remove(i);
                Ok(())
            }
            None => Err(RpgHackerError::NotFound(format!("Skill not found: {}", skill_id))),
        }
    }
    
    /// 添加敌人
    pub async fn add_enemy(&self, game_file: &mut GameFile, enemy: Enemy) -> Result<()> {
        // 检查敌人 ID 是否已存在
        if game_file.data.enemies.iter().any(|e| e.id == enemy.id) {
            return Err(RpgHackerError::InvalidArgument(format!("Enemy ID already exists: {}", enemy.id)));
        }
        
        // 添加敌人
        game_file.data.enemies.push(enemy);
        Ok(())
    }
    
    /// 删除敌人
    pub async fn remove_enemy(&self, game_file: &mut GameFile, enemy_id: u32) -> Result<()> {
        // 查找并删除敌人
        let index = game_file.data.enemies
            .iter()
            .position(|e| e.id == enemy_id);
        
        match index {
            Some(i) => {
                game_file.data.enemies.remove(i);
                Ok(())
            }
            None => Err(RpgHackerError::NotFound(format!("Enemy not found: {}", enemy_id))),
        }
    }
}

/// 数据验证工具
pub struct DataValidator;

impl DataValidator {
    /// 验证角色数据
    pub fn validate_actor(actor: &Actor) -> Result<()> {
        // 验证角色 ID
        if actor.id == 0 {
            return Err(RpgHackerError::InvalidArgument("Actor ID cannot be 0".to_string()));
        }
        
        // 验证角色名称
        if actor.name.is_empty() {
            return Err(RpgHackerError::InvalidArgument("Actor name cannot be empty".to_string()));
        }
        
        // 验证角色属性
        if actor.attributes.max_hp == 0 {
            return Err(RpgHackerError::InvalidArgument("Actor max HP cannot be 0".to_string()));
        }
        
        Ok(())
    }
    
    /// 验证物品数据
    pub fn validate_item(item: &Item) -> Result<()> {
        // 验证物品 ID
        if item.id == 0 {
            return Err(RpgHackerError::InvalidArgument("Item ID cannot be 0".to_string()));
        }
        
        // 验证物品名称
        if item.name.is_empty() {
            return Err(RpgHackerError::InvalidArgument("Item name cannot be empty".to_string()));
        }
        
        Ok(())
    }
    
    /// 验证技能数据
    pub fn validate_skill(skill: &Skill) -> Result<()> {
        // 验证技能 ID
        if skill.id == 0 {
            return Err(RpgHackerError::InvalidArgument("Skill ID cannot be 0".to_string()));
        }
        
        // 验证技能名称
        if skill.name.is_empty() {
            return Err(RpgHackerError::InvalidArgument("Skill name cannot be empty".to_string()));
        }
        
        Ok(())
    }
    
    /// 验证敌人数据
    pub fn validate_enemy(enemy: &Enemy) -> Result<()> {
        // 验证敌人 ID
        if enemy.id == 0 {
            return Err(RpgHackerError::InvalidArgument("Enemy ID cannot be 0".to_string()));
        }
        
        // 验证敌人名称
        if enemy.name.is_empty() {
            return Err(RpgHackerError::InvalidArgument("Enemy name cannot be empty".to_string()));
        }
        
        // 验证敌人属性
        if enemy.attributes.max_hp == 0 {
            return Err(RpgHackerError::InvalidArgument("Enemy max HP cannot be 0".to_string()));
        }
        
        Ok(())
    }
    
    /// 验证地图数据
    pub fn validate_map(map: &Map) -> Result<()> {
        // 验证地图 ID
        if map.id == 0 {
            return Err(RpgHackerError::InvalidArgument("Map ID cannot be 0".to_string()));
        }
        
        // 验证地图名称
        if map.name.is_empty() {
            return Err(RpgHackerError::InvalidArgument("Map name cannot be empty".to_string()));
        }
        
        // 验证地图尺寸
        if map.width == 0 || map.height == 0 {
            return Err(RpgHackerError::InvalidArgument("Map width and height cannot be 0".to_string()));
        }
        
        Ok(())
    }
    
    /// 验证事件数据
    pub fn validate_event(event: &Event) -> Result<()> {
        // 验证事件 ID
        if event.id == 0 {
            return Err(RpgHackerError::InvalidArgument("Event ID cannot be 0".to_string()));
        }
        
        // 验证事件名称
        if event.name.is_empty() {
            return Err(RpgHackerError::InvalidArgument("Event name cannot be empty".to_string()));
        }
        
        Ok(())
    }
    
    /// 验证系统数据
    pub fn validate_system(system: &SystemData) -> Result<()> {
        // 验证游戏标题
        if system.game_title.is_empty() {
            return Err(RpgHackerError::InvalidArgument("Game title cannot be empty".to_string()));
        }
        
        Ok(())
    }
}
