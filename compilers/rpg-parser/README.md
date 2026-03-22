# rpg-parser

RPG Maker 特殊代码解析器，用于识别和保护 RPG Maker 游戏中的特殊代码，如 `\V[1]`、`\N[1]`、`\C[2]` 等。

## 功能特性

- **特殊代码识别**：能够识别 RPG Maker 中的各种特殊代码
- **代码保护**：将特殊代码替换为占位符，避免翻译时被修改
- **代码恢复**：翻译后将占位符恢复为原始特殊代码
- **轻量级**：无外部依赖，纯 Rust 实现

## 安装

将此库添加到你的 Cargo.toml 文件中：

```toml
[dependencies]
rpg-parser = { path = "path/to/rpg-parser" }
```

## 使用示例

### 基本用法

```rust
use rpg_parser::parse_special_codes;

let text = "Hello \\V[1], your name is \\N[1]!";
let result = parse_special_codes(text);

// 处理后的文本（可用于翻译）
println!("Processed text: {}", result.processed);
// 特殊代码映射
println!("Special codes: {:?}", result.special_codes);
```

### 翻译后恢复特殊代码

```rust
use rpg_parser::{parse_special_codes, restore_special_codes};

let text = "Hello \\V[1], your name is \\N[1]!";
let parse_result = parse_special_codes(text);

// 翻译处理后的文本
let translated = translate_function(&parse_result.processed);

// 恢复特殊代码
let final_result = restore_special_codes(&translated, &parse_result.special_codes);
println!("Final result: {}", final_result);
```

### 检测特殊代码

```rust
use rpg_parser::contains_special_codes;

let text1 = "Hello \\V[1]";
let text2 = "Hello world";

assert_eq!(contains_special_codes(text1), true);
assert_eq!(contains_special_codes(text2), false);
```

## 支持的特殊代码

- `\V[n]` - 变量引用
- `\N[n]` - 名称引用
- `\C[n]` - 颜色代码
- `\W[n]` - 等待时间
- `\I[n]` - 物品图标
- `\S[n]` - 声音效果
- `\G` - 金币
- `\P[n]` - 队伍成员名称
- `\T[n]` - 时间
- `\{` - 小字体
- `\}` - 大字体
- `\.` - 等待
- `\|` - 等待
- `\!` - 消除等待
- `\>` - 快速显示
- `\<` - 慢速显示
- `\^` - 换行

## 技术细节

- **解析原理**：使用状态机解析文本，识别特殊代码模式
- **性能**：线性时间复杂度，适用于处理大量文本
- **可靠性**：经过测试，能够正确处理各种特殊代码组合

## 测试

运行测试：

```bash
cargo test
```

## 许可证

MIT 许可证