use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RgssScript {
    pub name: String,
    pub content: String,
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RgssProject {
    pub scripts: Vec<RgssScript>,
    pub title: String,
    pub author: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RgssResource {
    pub name: String,
    pub path: String,
    pub resource_type: ResourceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResourceType {
    Image,
    Audio,
    Script,
    Data,
    Other,
}
