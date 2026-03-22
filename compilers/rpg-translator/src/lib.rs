/// RPG Maker 翻译器
/// 支持批量翻译、拆分合批翻译，以及特殊代码保护

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Duration;

/// 翻译错误类型
#[derive(Debug)]
pub enum TranslatorError {
    /// API 错误
    ApiError(String),
    /// 网络错误
    NetworkError(String),
    /// 解析错误
    ParseError(String),
    /// 未实现的功能
    NotImplemented(String),
    /// 无效参数
    InvalidArgument(String),
}

impl fmt::Display for TranslatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TranslatorError::ApiError(msg) => write!(f, "API error: {}", msg),
            TranslatorError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            TranslatorError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            TranslatorError::NotImplemented(msg) => write!(f, "Not implemented: {}", msg),
            TranslatorError::InvalidArgument(msg) => write!(f, "Invalid argument: {}", msg),
        }
    }
}

impl Error for TranslatorError {}

/// 通用结果类型
pub type Result<T> = std::result::Result<T, TranslatorError>;

/// 翻译 API 类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TranslatorApi {
    /// DeepL API
    Deepl,
    /// Google Translate API
    Google,
    /// 本地翻译（占位符）
    Local,
}

/// 翻译配置
#[derive(Debug, Clone)]
pub struct TranslatorConfig {
    /// API 类型
    pub api: TranslatorApi,
    /// API 密钥
    pub api_key: String,
    /// 目标语言
    pub target_lang: String,
    /// 源语言（可选）
    pub source_lang: Option<String>,
    /// 批量翻译最大批次大小
    pub batch_size: usize,
    /// 长文本拆分阈值（字符数）
    pub split_threshold: usize,
    /// 请求超时时间（秒）
    pub timeout: u64,
}

impl Default for TranslatorConfig {
    fn default() -> Self {
        Self {
            api: TranslatorApi::Deepl,
            api_key: String::new(),
            target_lang: "zh".to_string(),
            source_lang: None,
            batch_size: 50,
            split_threshold: 5000,
            timeout: 30,
        }
    }
}

/// 翻译请求
#[derive(Debug, Clone)]
pub struct TranslationRequest {
    /// 待翻译的文本
    pub texts: Vec<String>,
    /// 目标语言
    pub target_lang: String,
    /// 源语言（可选）
    pub source_lang: Option<String>,
}

/// 翻译结果
#[derive(Debug, Clone)]
pub struct TranslationResult {
    /// 原始文本
    pub original: String,
    /// 翻译后的文本
    pub translated: String,
}

/// 翻译器 trait
#[async_trait::async_trait]
pub trait Translator: Send + Sync {
    /// 翻译单个文本
    async fn translate(&self, text: &str, target_lang: &str, source_lang: Option<&str>) -> Result<String>;
    
    /// 批量翻译
    async fn translate_batch(&self, texts: &[String], target_lang: &str, source_lang: Option<&str>) -> Result<Vec<String>>;
    
    /// 翻译长文本（自动拆分合批）
    async fn translate_long_text(&self, text: &str, target_lang: &str, source_lang: Option<&str>) -> Result<String>;
}

/// 基于 DeepL API 的翻译器实现
pub struct DeeplTranslator {
    config: TranslatorConfig,
}

impl DeeplTranslator {
    /// 创建新的 DeepL 翻译器
    pub fn new(config: TranslatorConfig) -> Self {
        Self { config }
    }
}

#[async_trait::async_trait]
impl Translator for DeeplTranslator {
    async fn translate(&self, text: &str, target_lang: &str, source_lang: Option<&str>) -> Result<String> {
        let results = self.translate_batch(&[text.to_string()], target_lang, source_lang).await?;
        Ok(results[0].clone())
    }

    async fn translate_batch(&self, texts: &[String], target_lang: &str, source_lang: Option<&str>) -> Result<Vec<String>> {
        // 这里实现 DeepL API 调用
        // 由于是示例，返回模拟结果
        Ok(texts.iter().map(|text| format!("[Translated] {}", text)).collect())
    }

    async fn translate_long_text(&self, text: &str, target_lang: &str, source_lang: Option<&str>) -> Result<String> {
        // 拆分长文本
        let chunks = self.split_text(text, self.config.split_threshold);
        // 批量翻译
        let translated_chunks = self.translate_batch(&chunks, target_lang, source_lang).await?;
        // 合并结果
        Ok(translated_chunks.join(""))
    }
}

impl DeeplTranslator {
    /// 拆分长文本
    fn split_text(&self, text: &str, threshold: usize) -> Vec<String> {
        let mut chunks = Vec::new();
        let mut current_chunk = String::new();
        
        for sentence in text.split('.') {
            let sentence = sentence.trim();
            if sentence.is_empty() {
                continue;
            }
            
            let sentence_with_period = format!("{}.", sentence);
            
            if current_chunk.len() + sentence_with_period.len() > threshold {
                if !current_chunk.is_empty() {
                    chunks.push(current_chunk);
                    current_chunk = sentence_with_period;
                } else {
                    // 单个句子超过阈值，强制拆分
                    let mut words = sentence.split(' ');
                    let mut temp_chunk = String::new();
                    
                    while let Some(word) = words.next() {
                        if temp_chunk.len() + word.len() + 1 > threshold {
                            chunks.push(temp_chunk);
                            temp_chunk = word.to_string();
                        } else {
                            if !temp_chunk.is_empty() {
                                temp_chunk.push(' ');
                            }
                            temp_chunk.push_str(word);
                        }
                    }
                    
                    if !temp_chunk.is_empty() {
                        chunks.push(format!("{}.", temp_chunk));
                    }
                }
            } else {
                if !current_chunk.is_empty() {
                    current_chunk.push(' ');
                }
                current_chunk.push_str(&sentence_with_period);
            }
        }
        
        if !current_chunk.is_empty() {
            chunks.push(current_chunk);
        }
        
        chunks
    }
}

/// 翻译器工厂
pub struct TranslatorFactory;

impl TranslatorFactory {
    /// 创建翻译器
    pub fn create(config: TranslatorConfig) -> Box<dyn Translator> {
        match config.api {
            TranslatorApi::Deepl => Box::new(DeeplTranslator::new(config)),
            TranslatorApi::Google => {
                // 实现 Google Translate API
                Box::new(DeeplTranslator::new(config)) // 临时使用 DeepL 实现
            }
            TranslatorApi::Local => {
                // 实现本地翻译
                Box::new(DeeplTranslator::new(config)) // 临时使用 DeepL 实现
            }
        }
    }
}

/// 翻译管理器
pub struct TranslationManager {
    translator: Box<dyn Translator>,
    config: TranslatorConfig,
}

impl TranslationManager {
    /// 创建新的翻译管理器
    pub fn new(config: TranslatorConfig) -> Self {
        Self {
            translator: TranslatorFactory::create(config.clone()),
            config,
        }
    }
    
    /// 翻译单个文本，处理特殊代码
    pub async fn translate_with_special_codes(&self, text: &str) -> Result<String> {
        // 解析特殊代码
        let parse_result = rpg_parser::parse_special_codes(text);
        
        // 翻译处理后的文本
        let translated = self.translator
            .translate(&parse_result.processed, &self.config.target_lang, self.config.source_lang.as_deref())
            .await?;
        
        // 恢复特殊代码
        let result = rpg_parser::restore_special_codes(&translated, &parse_result.special_codes);
        Ok(result)
    }
    
    /// 批量翻译，处理特殊代码
    pub async fn translate_batch_with_special_codes(&self, texts: &[String]) -> Result<Vec<String>> {
        // 解析特殊代码
        let parse_results: Vec<_> = texts.iter().map(|text| rpg_parser::parse_special_codes(text)).collect();
        
        // 提取处理后的文本
        let processed_texts: Vec<_> = parse_results.iter().map(|r| r.processed.clone()).collect();
        
        // 批量翻译
        let translated_texts = self.translator
            .translate_batch(&processed_texts, &self.config.target_lang, self.config.source_lang.as_deref())
            .await?;
        
        // 恢复特殊代码
        let results: Vec<_> = parse_results.into_iter().zip(translated_texts).map(|(parse_result, translated)| {
            rpg_parser::restore_special_codes(&translated, &parse_result.special_codes)
        }).collect();
        
        Ok(results)
    }
    
    /// 翻译长文本，处理特殊代码
    pub async fn translate_long_text_with_special_codes(&self, text: &str) -> Result<String> {
        // 解析特殊代码
        let parse_result = rpg_parser::parse_special_codes(text);
        
        // 翻译处理后的长文本
        let translated = self.translator
            .translate_long_text(&parse_result.processed, &self.config.target_lang, self.config.source_lang.as_deref())
            .await?;
        
        // 恢复特殊代码
        let result = rpg_parser::restore_special_codes(&translated, &parse_result.special_codes);
        Ok(result)
    }
    
    /// 将翻译结果写入 YAML 文件
    pub fn write_to_yaml(&self, results: &[TranslationResult], output_path: &str) -> Result<()> {
        let path = Path::new(output_path);
        let mut file = File::create(path)
            .map_err(|e| TranslatorError::ParseError(format!("Failed to create file: {}", e)))?;
        
        let yaml_str = serde_yaml::to_string(&results)
            .map_err(|e| TranslatorError::ParseError(format!("Failed to serialize to YAML: {}", e)))?;
        
        file.write_all(yaml_str.as_bytes())
            .map_err(|e| TranslatorError::ParseError(format!("Failed to write to file: {}", e)))?;
        
        Ok(())
    }
    
    /// 批量翻译并写入 YAML 文件
    pub async fn translate_and_write_to_yaml(&self, texts: &[String], output_path: &str) -> Result<()> {
        let translated_texts = self.translate_batch_with_special_codes(texts).await?;
        
        let results: Vec<TranslationResult> = texts.iter()
            .zip(translated_texts.iter())
            .map(|(original, translated)| TranslationResult {
                original: original.clone(),
                translated: translated.clone(),
            })
            .collect();
        
        self.write_to_yaml(&results, output_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_text() {
        let config = TranslatorConfig::default();
        let translator = DeeplTranslator::new(config);
        
        let long_text = "This is a long sentence. This is another long sentence. This is a third long sentence. This is a fourth long sentence.";
        let chunks = translator.split_text(long_text, 30);
        
        assert!(!chunks.is_empty());
        for chunk in &chunks {
            assert!(chunk.len() <= 30);
        }
    }

    #[tokio::test]
    async fn test_translate_batch() {
        let config = TranslatorConfig::default();
        let translator = DeeplTranslator::new(config);
        
        let texts = vec!["Hello", "World"];
        let results = translator.translate_batch(&texts, "zh", None).await.unwrap();
        
        assert_eq!(results.len(), 2);
        assert!(results[0].contains("Hello"));
        assert!(results[1].contains("World"));
    }
}
