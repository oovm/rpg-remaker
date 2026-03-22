# rpg-translator

RPG Maker 翻译器，支持批量翻译、拆分合批翻译，以及特殊代码保护。

## 功能特性

- **批量翻译**：支持一次性翻译多个文本，减少 API 调用次数
- **拆分合批翻译**：自动拆分长文本，翻译后重新合并
- **特殊代码保护**：与 rpg-parser 集成，保护 RPG Maker 特殊代码不被翻译
- **多 API 支持**：支持 DeepL、Google Translate 等多种翻译 API
- **YAML 输出**：将翻译结果写入 YAML 格式文件

## 安装

将此库添加到你的 Cargo.toml 文件中：

```toml
[dependencies]
rpg-translator = { path = "path/to/rpg-translator" }
```

## 使用示例

### 基本翻译

```rust
use rpg_translator::{TranslatorConfig, TranslationManager};

// 创建翻译配置
let config = TranslatorConfig {
    target_lang: "zh".to_string(),
    api_key: "your-api-key".to_string(),
    ..Default::default()
};

// 创建翻译管理器
let manager = TranslationManager::new(config);

// 翻译单个文本
let text = "Hello \\V[1], welcome to the game!";
let result = manager.translate_with_special_codes(text).await?;
println!("Translated: {}", result);
```

### 批量翻译

```rust
use rpg_translator::{TranslatorConfig, TranslationManager};

// 创建翻译配置
let config = TranslatorConfig {
    target_lang: "zh".to_string(),
    api_key: "your-api-key".to_string(),
    ..Default::default()
};

// 创建翻译管理器
let manager = TranslationManager::new(config);

// 批量翻译
let texts = vec![
    "Hello world".to_string(),
    "Welcome to RPG Maker".to_string(),
    "\\V[1] is your variable".to_string()
];

let results = manager.translate_batch_with_special_codes(&texts).await?;
for (i, result) in results.iter().enumerate() {
    println!("{} -> {}", texts[i], result);
}
```

### 翻译并写入 YAML

```rust
use rpg_translator::{TranslatorConfig, TranslationManager};

// 创建翻译配置
let config = TranslatorConfig {
    target_lang: "zh".to_string(),
    api_key: "your-api-key".to_string(),
    ..Default::default()
};

// 创建翻译管理器
let manager = TranslationManager::new(config);

// 批量翻译并写入 YAML
let texts = vec![
    "Hello world".to_string(),
    "Welcome to RPG Maker".to_string()
];

manager.translate_and_write_to_yaml(&texts, "translations.yaml").await?;
println!("Translations written to translations.yaml");
```

## 配置选项

| 选项 | 说明 | 默认值 |
|------|------|--------|
| `api` | 翻译 API 类型 | `TranslatorApi::Deepl` |
| `api_key` | API 密钥 | `""` |
| `target_lang` | 目标语言 | `"zh"` |
| `source_lang` | 源语言（可选） | `None` |
| `batch_size` | 批量翻译最大批次大小 | `50` |
| `split_threshold` | 长文本拆分阈值（字符数） | `5000` |
| `timeout` | 请求超时时间（秒） | `30` |

## 支持的翻译 API

- **DeepL**：高质量翻译，支持多种语言
- **Google Translate**：广泛支持的翻译服务
- **Local**：本地翻译（占位符，可扩展）

## 技术细节

- **批量翻译**：减少 API 调用次数，提高效率
- **长文本处理**：自动拆分超过 API 限制的长文本
- **特殊代码保护**：与 rpg-parser 集成，确保特殊代码不被翻译
- **错误处理**：完善的错误处理和重试机制

## 测试

运行测试：

```bash
cargo test
```

## 许可证

MIT 许可证