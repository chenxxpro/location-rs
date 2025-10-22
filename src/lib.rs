//! 国家代码解析器库
//! 
//! 这个库提供从标题文本中解析国家或地区代码的功能。
//! 支持ISO 3166-1 alpha-2/alpha-3代码以及简体中文和繁体中文的国家名称。

pub mod error;
pub mod config;
pub mod parser;

// 重新导出主要类型
pub use error::ParseError;
pub use config::{Configuration, CountryInfo, ParserSettings};
pub use parser::parse_country_code;

// 重新导出isocountry的CountryCode
pub use isocountry::CountryCode;

/// 主要的解析函数
/// 
/// # 示例
/// 
/// ```rust
/// use location_rs::parse_country_code;
/// 
/// let result = parse_country_code("@HK Vip1");
/// assert!(result.is_ok());
/// ```
pub fn parse_country_code(text: &str) -> Result<CountryCode, ParseError> {
    parser::parse_country_code(text)
}

/// 解析器配置
#[derive(Debug, Clone)]
pub struct ParserConfig {
    /// 是否区分大小写
    pub case_sensitive: bool,
    /// 是否启用模糊匹配
    pub fuzzy_match: bool,
    /// 解析超时时间（毫秒）
    pub timeout_ms: u64,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            case_sensitive: false,
            fuzzy_match: true,
            timeout_ms: 100,
        }
    }
}

/// 解析器实例
pub struct Parser {
    config: ParserConfig,
}

impl Parser {
    /// 使用默认配置创建解析器
    pub fn new() -> Self {
        Self {
            config: ParserConfig::default(),
        }
    }
    
    /// 使用自定义配置创建解析器
    pub fn with_config(config: ParserConfig) -> Self {
        Self { config }
    }
    
    /// 解析文本中的国家代码
    pub fn parse(&self, text: &str) -> Result<CountryCode, ParseError> {
        parser::parse_country_code_with_config(text, &self.config)
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}