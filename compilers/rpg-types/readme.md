# rpg-types

RPG Maker 类型定义库，提供 RPG Maker 系列游戏中使用的各种数据类型的 Rust 定义。

## 功能

- **类型定义**：提供 RPG Maker 游戏中常见数据结构的 Rust 类型定义
- **错误处理**：定义了专门的错误类型，用于处理 RPG Maker 数据相关的错误
- **类型安全**：使用 Rust 的类型系统确保数据操作的安全性

## 依赖

无外部依赖，纯 Rust 实现。

## 安装

将此库添加到你的 Cargo.toml 文件中：

```toml
[dependencies]
rpg-types = { path = "path/to/rpg-types" }
```

## 使用示例

### 使用错误类型

```rust
use rpg_types::errors::RpgError;

fn main() {
    let error = RpgError::InvalidData("Invalid RPG Maker data".to_string());
    println!("Error: {:?}", error);
}
```

## 模块结构

- `src/lib.rs`：导出模块和公共 API
- `src/errors.rs`：定义错误类型

## 支持的类型

- **错误类型**：`RpgError` - 用于表示 RPG Maker 数据相关的错误

## 许可证

MIT 许可证