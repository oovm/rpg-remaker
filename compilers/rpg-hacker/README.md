# RPG Hacker

RPG Hacker 是一个用 Rust 编写的 RPG Maker 游戏修改器后端，支持多种 RPG Maker 版本，包括 RPG Maker 2000/2003、VX、VXAce 等。

## 功能特性

- **游戏文件解析**：支持解析不同版本的 RPG Maker 游戏文件
- **资源管理**：支持图像、音频、文本等资源的读取、修改和保存
- **游戏数据修改**：支持角色、物品、技能、敌人、地图等数据的编辑
- **脚本编辑**：支持事件脚本和系统脚本的编辑
- **文件导出**：支持导出修改后的游戏文件为不同版本的 RPG Maker 格式
- **版本兼容性检查**：检查游戏在不同版本 RPG Maker 中的兼容性

## 目录结构

```
rpg-hacker/
├── src/
│   ├── lib.rs          # 主库文件
│   ├── parser.rs       # 游戏文件解析器
│   ├── resource.rs     # 资源管理
│   ├── data.rs         # 游戏数据修改
│   ├── script.rs       # 脚本编辑
│   ├── export.rs       # 文件导出
│   └── version.rs      # 版本兼容性检查
├── Cargo.toml          # 依赖配置
└── README.md          # 项目说明
```

## 安装

```bash
# 添加依赖
cargo add rpg-hacker
```

## 使用示例

### 解析游戏文件

```rust
use rpg_hacker::*;

#[tokio::main]
async fn main() {
    // 创建服务实例
    let service = MemoryRpgHackerService::new();
    
    // 创建解析器
    let parser = GameFileParser::new(Box::new(service));
    
    // 解析游戏文件
    let game_file = parser.parse("path/to/game.exe").await.unwrap();
    
    println!("游戏标题: {}", game_file.title);
    println!("RPG Maker 版本: {:?}", game_file.version);
}
```

### 修改游戏数据

```rust
use rpg_hacker::*;

#[tokio::main]
async fn main() {
    // 创建服务实例
    let service = MemoryRpgHackerService::new();
    
    // 创建数据修改器
    let modifier = GameDataModifier::new(Box::new(service));
    
    // 假设我们已经有了一个 game_file
    let mut game_file = GameFile {
        // 初始化游戏文件
        // ...
    };
    
    // 修改角色数据
    let actor_mod = ActorModification {
        id: 1,
        name: Some("新角色名称"),
        level: Some(99),
        attributes: Some(ActorAttributes {
            max_hp: 9999,
            max_mp: 999,
            attack: 999,
            defense: 999,
            magic_attack: 999,
            magic_defense: 999,
            agility: 999,
            luck: 999,
        }),
        skills: None,
    };
    
    modifier.modify_actor(&mut game_file, actor_mod).await.unwrap();
}
```

### 导出游戏文件

```rust
use rpg_hacker::*;

#[tokio::main]
async fn main() {
    // 创建服务实例
    let service = MemoryRpgHackerService::new();
    
    // 创建导出器
    let exporter = FileExporter::new(Box::new(service));
    
    // 假设我们已经有了一个 game_file
    let game_file = GameFile {
        // 初始化游戏文件
        // ...
    };
    
    // 导出为 RPG Maker VX Ace 格式
    exporter.export_as_rpgvx_ace(&game_file, "output/game.zip").await.unwrap();
}
```

## 版本兼容性

| 源版本 | 目标版本 | 兼容性 |
|--------|--------|--------|
| RPG Maker 2000 | RPG Maker 2003 | ✅ |
| RPG Maker 2003 | RPG Maker 2000 | ⚠️ |
| RPG Maker 2003 | RPG Maker VX | ❌ |
| RPG Maker VX | RPG Maker VX Ace | ✅ |
| RPG Maker VX Ace | RPG Maker VX | ⚠️ |

## 贡献

欢迎提交 Issue 和 Pull Request！

## 许可证

MIT
