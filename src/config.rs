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
    /// 英文名称
    pub name_en: String,
    /// 简体中文名称
    pub name_zh_cn: String,
    /// 繁体中文名称
    pub name_zh_tw: String,
    /// 国家简称和别称
    pub abbreviations: Vec<String>,
}

/// 解析器设置
#[derive(Debug, Deserialize, Clone)]
pub struct ParserSettings {
    /// 是否区分大小写
    pub case_sensitive: bool,
    /// 是否启用模糊匹配
    pub fuzzy_match: bool,
    /// 超时时间（毫秒）
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

/// 国家配置
#[derive(Debug, Deserialize, Clone)]
pub struct CountriesConfig {
    /// 配置版本
    pub version: String,
    /// 国家信息列表
    pub countries: Vec<CountryInfo>,
}

/// 完整配置
#[derive(Debug, Clone)]
pub struct Configuration {
    /// 国家配置
    pub countries_config: CountriesConfig,
    /// 模式配置
    pub patterns: PatternConfig,
    /// 解析器设置
    pub settings: ParserSettings,
}

impl Configuration {
    /// 从多个嵌入的JSON文件加载配置
    pub fn load() -> Result<Self, ParseError> {
        // 加载国家配置
        let countries_str = include_str!("../resources/countries.json");
        let countries_config: CountriesConfig = serde_json::from_str(countries_str)
            .map_err(|e| ParseError::config_error(&format!("国家配置解析失败: {}", e)))?;
        
        // 加载模式配置
        let patterns_str = include_str!("../resources/patterns.json");
        let patterns: PatternConfig = serde_json::from_str(patterns_str)
            .map_err(|e| ParseError::config_error(&format!("模式配置解析失败: {}", e)))?;
        
        // 加载解析器设置
        let settings_str = include_str!("../resources/settings.json");
        let settings: ParserSettings = serde_json::from_str(settings_str)
            .map_err(|e| ParseError::config_error(&format!("设置配置解析失败: {}", e)))?;
        
        Ok(Configuration {
            countries_config,
            patterns,
            settings,
        })
    }
    
    /// 创建国家代码到国家信息的映射
    pub fn create_country_mapping(&self) -> HashMap<String, &CountryInfo> {
        let mut mapping = HashMap::new();
        
        for country in &self.countries_config.countries {
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
            
            // 添加所有简称和别称
            for abbr in &country.abbreviations {
                mapping.insert(abbr.clone(), country);
            }
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
        &self.countries_config.countries
    }
    
    /// 获取配置版本
    pub fn get_version(&self) -> &str {
        &self.countries_config.version
    }
}