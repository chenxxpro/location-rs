use serde::Deserialize;
use std::collections::HashMap;
use crate::error::ParseError;

/// 国家信息配置
#[derive(Debug, Deserialize, Clone)]
pub struct CountryInfo {
    /// ISO 3166-1 alpha-2代码
    pub alpha2: String,
    /// ISO 3166-1 alpha-3代码
    pub alpha3: String,
    /// ISO 3166-1 数字代码
    pub numeric: String,
    /// 英文名称
    pub name_en: String,
    /// 简体中文名称
    pub name_zh_cn: String,
    /// 繁体中文名称
    pub name_zh_tw: String,
    /// 所属地区
    pub region: String,
}

/// 解析器设置
#[derive(Debug, Deserialize, Clone)]
pub struct ParserSettings {
    /// 是否区分大小写
    pub case_sensitive: bool,
    /// 是否启用模糊匹配
    pub fuzzy_match: bool,
    /// 解析超时时间（毫秒）
    pub timeout_ms: u64,
}

/// 模式配置
#[derive(Debug, Deserialize, Clone)]
pub struct PatternConfig {
    /// 前缀模式
    pub prefix_patterns: Vec<String>,
    /// 后缀模式
    pub suffix_patterns: Vec<String>,
}

/// 完整配置
#[derive(Debug, Deserialize, Clone)]
pub struct Configuration {
    /// 配置版本
    pub version: String,
    /// 国家信息列表
    pub countries: Vec<CountryInfo>,
    /// 模式配置
    pub patterns: PatternConfig,
    /// 解析器设置
    pub settings: ParserSettings,
}

impl Configuration {
    /// 从嵌入的JSON字符串加载配置
    pub fn load() -> Result<Self, ParseError> {
        let config_str = include_str!("../resources/countries.json");
        
        serde_json::from_str(config_str)
            .map_err(|e| ParseError::config_error(&format!("配置解析失败: {}", e)))
    }
    
    /// 创建国家代码到国家信息的映射
    pub fn create_country_mapping(&self) -> HashMap<String, &CountryInfo> {
        let mut mapping = HashMap::new();
        
        for country in &self.countries {
            // 添加alpha-2代码
            mapping.insert(country.alpha2.clone(), country);
            
            // 添加alpha-3代码
            mapping.insert(country.alpha3.clone(), country);
            
            // 添加英文名称
            mapping.insert(country.name_en.clone(), country);
            
            // 添加简体中文名称
            mapping.insert(country.name_zh_cn.clone(), country);
            
            // 添加繁体中文名称
            mapping.insert(country.name_zh_tw.clone(), country);
        }
        
        mapping
    }
    
    /// 获取解析器设置
    pub fn get_settings(&self) -> &ParserSettings {
        &self.settings
    }
    
    /// 获取模式配置
    pub fn get_patterns(&self) -> &PatternConfig {
        &self.patterns
    }
    
    /// 获取所有国家信息
    pub fn get_countries(&self) -> &[CountryInfo] {
        &self.countries
    }
}