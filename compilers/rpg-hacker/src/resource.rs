//! 资源管理
//! 
//! 负责游戏资源的读取、修改和保存。

use super::*;
use std::path::Path;

/// 资源管理器
pub struct ResourceManager {
    /// 服务实例
    service: Box<dyn RpgHackerService>,
}

impl ResourceManager {
    /// 创建新的资源管理器
    pub fn new(service: Box<dyn RpgHackerService>) -> Self {
        Self {
            service,
        }
    }
    
    /// 获取资源
    pub async fn get_resource(&self, game_file: &GameFile, resource_path: &str) -> Result<Resource> {
        self.service.get_resource(game_file, resource_path).await
    }
    
    /// 添加资源
    pub async fn add_resource(&self, game_file: &mut GameFile, resource: Resource) -> Result<()> {
        self.service.add_resource(game_file, resource).await
    }
    
    /// 删除资源
    pub async fn remove_resource(&self, game_file: &mut GameFile, resource_path: &str) -> Result<()> {
        self.service.remove_resource(game_file, resource_path).await
    }
    
    /// 导入资源
    pub async fn import_resource(&self, game_file: &mut GameFile, source_path: &str, target_path: &str) -> Result<Resource> {
        // 读取源文件
        let content = FileUtils::read_file(source_path)?;
        
        // 确定资源类型
        let resource_type = self.determine_resource_type(source_path);
        
        // 创建资源
        let resource = Resource {
            path: target_path.to_string(),
            resource_type,
            size: content.len() as u64,
            content: Some(content),
        };
        
        // 添加资源
        self.add_resource(game_file, resource.clone()).await?;
        
        Ok(resource)
    }
    
    /// 导出资源
    pub async fn export_resource(&self, game_file: &GameFile, resource_path: &str, output_path: &str) -> Result<()> {
        // 获取资源
        let resource = self.get_resource(game_file, resource_path).await?;
        
        // 写入文件
        if let Some(content) = resource.content {
            FileUtils::write_file(output_path, &content)?;
        } else {
            return Err(RpgHackerError::NotFound(format!("Resource content not found: {}", resource_path)));
        }
        
        Ok(())
    }
    
    /// 确定资源类型
    fn determine_resource_type(&self, path: &str) -> ResourceType {
        let ext = Path::new(path).extension().and_then(|e| e.to_str()).unwrap_or("");
        match ext.to_lowercase().as_str() {
            // 图像资源
            "png" | "jpg" | "jpeg" | "bmp" | "gif" => ResourceType::Image,
            // 音频资源
            "mp3" | "wav" | "ogg" | "mid" => ResourceType::Audio,
            // 文本资源
            "txt" | "csv" | "json" | "xml" => ResourceType::Text,
            // 脚本资源
            "rb" | "js" | "lua" => ResourceType::Script,
            // 其他资源
            _ => ResourceType::Other,
        }
    }
    
    /// 列出所有资源
    pub fn list_resources(&self, game_file: &GameFile) -> Vec<Resource> {
        game_file.resources.clone()
    }
    
    /// 搜索资源
    pub fn search_resources(&self, game_file: &GameFile, pattern: &str) -> Vec<Resource> {
        game_file.resources
            .iter()
            .filter(|r| r.path.contains(pattern))
            .cloned()
            .collect()
    }
    
    /// 按类型过滤资源
    pub fn filter_resources_by_type(&self, game_file: &GameFile, resource_type: ResourceType) -> Vec<Resource> {
        game_file.resources
            .iter()
            .filter(|r| r.resource_type == resource_type)
            .cloned()
            .collect()
    }
}

/// 资源处理工具
pub struct ResourceUtils;

impl ResourceUtils {
    /// 读取图像资源
    pub fn read_image(path: &str) -> Result<Vec<u8>> {
        FileUtils::read_file(path)
    }
    
    /// 写入图像资源
    pub fn write_image(path: &str, content: &[u8]) -> Result<()> {
        FileUtils::write_file(path, content)
    }
    
    /// 读取音频资源
    pub fn read_audio(path: &str) -> Result<Vec<u8>> {
        FileUtils::read_file(path)
    }
    
    /// 写入音频资源
    pub fn write_audio(path: &str, content: &[u8]) -> Result<()> {
        FileUtils::write_file(path, content)
    }
    
    /// 读取文本资源
    pub fn read_text(path: &str) -> Result<String> {
        let content = FileUtils::read_file(path)?;
        String::from_utf8(content).map_err(|e| RpgHackerError::InvalidFormat(e.to_string()))
    }
    
    /// 写入文本资源
    pub fn write_text(path: &str, content: &str) -> Result<()> {
        FileUtils::write_file(path, content.as_bytes())
    }
    
    /// 读取脚本资源
    pub fn read_script(path: &str) -> Result<String> {
        Self::read_text(path)
    }
    
    /// 写入脚本资源
    pub fn write_script(path: &str, content: &str) -> Result<()> {
        Self::write_text(path, content)
    }
    
    /// 复制资源
    pub fn copy_resource(source: &str, destination: &str) -> Result<()> {
        let content = FileUtils::read_file(source)?;
        FileUtils::write_file(destination, &content)
    }
    
    /// 删除资源
    pub fn delete_resource(path: &str) -> Result<()> {
        std::fs::remove_file(path).map_err(|e| RpgHackerError::InternalError(e.to_string()))
    }
    
    /// 资源重命名
    pub fn rename_resource(old_path: &str, new_path: &str) -> Result<()> {
        std::fs::rename(old_path, new_path).map_err(|e| RpgHackerError::InternalError(e.to_string()))
    }
    
    /// 资源移动
    pub fn move_resource(source: &str, destination: &str) -> Result<()> {
        std::fs::rename(source, destination).map_err(|e| RpgHackerError::InternalError(e.to_string()))
    }
    
    /// 资源大小
    pub fn resource_size(path: &str) -> Result<u64> {
        FileUtils::file_size(path)
    }
    
    /// 资源是否存在
    pub fn resource_exists(path: &str) -> bool {
        FileUtils::file_exists(path)
    }
}
