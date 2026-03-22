# RPG Remaker

RPG Remaker 是一个基于 Rust 开发的 RPG 游戏制作工具集，提供了从数据管理到游戏创建的完整解决方案。

## 项目结构

本项目采用 Rust 工作区结构，包含多个子项目：

```
rpg-remaker/
├── compilers/
│   ├── rpg-data/          # RPG 数据模块
│   ├── rpg-remaker/       # RPG 游戏制作工具
│   └── rpg-types/         # RPG Maker 类型定义库
├── Cargo.toml             # 工作区配置文件
├── License.md             # 许可证文件
└── readme.md              # 项目说明文件
```

## 子项目说明

### rpg-types

RPG Maker 类型定义库，提供 RPG Maker 系列游戏中使用的各种数据类型的 Rust 定义。

- **类型定义**：提供 RPG Maker 游戏中常见数据结构的 Rust 类型定义
- **错误处理**：定义了专门的错误类型，用于处理 RPG Maker 数据相关的错误
- **类型安全**：使用 Rust 的类型系统确保数据操作的安全性
- **无外部依赖**：纯 Rust 实现

### rpg-data

RPG 数据模块，用于存储和管理游戏数据。

- **提供游戏数据结构定义**
- **支持数据序列化和反序列化**
- **与其他模块集成**
- **依赖**：alox-48（用于数据序列化）

### rpg-remaker

RPG 游戏制作工具，用于创建和管理 RPG 游戏。

- **提供命令行界面**
- **集成 rpg-data 模块**
- **支持游戏数据管理**
- **依赖**：
  - clap（用于命令行参数解析）
  - rpg-data（游戏数据模块）
  - alox-48（用于数据序列化）

## 技术栈

- **语言**：Rust 2024
- **构建工具**：Cargo
- **许可证**：AGPL3.0
- **依赖管理**：Cargo 工作区

## 安装

1. 确保已安装 Rust 工具链（推荐使用 rustup）
2. 克隆项目仓库
3. 进入项目根目录
4. 构建项目：

```bash
cargo build --release
```

## 使用

### 运行 RPG Remaker 工具

```bash
cargo run --package rpg-remaker -- [命令行参数]
```

### 作为库使用

在你的 Cargo.toml 文件中添加依赖：

```toml
[dependencies]
rpg-types = { path = "path/to/rpg-remaker/compilers/rpg-types" }
rpg-data = { path = "path/to/rpg-remaker/compilers/rpg-data" }
```

## 项目目标

- 提供完整的 RPG 游戏制作工具链
- 支持 RPG Maker 系列游戏的数据格式
- 实现类型安全的数据管理
- 提供简洁易用的命令行界面
- 支持跨平台使用

## 贡献

欢迎提交问题和拉取请求！请确保遵循项目的代码风格和贡献指南。

## 许可证

本项目采用 AGPL3.0 许可证，详见 [License.md](License.md) 文件。
