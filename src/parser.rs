use std::collections::HashMap;
use std::time::{Duration, Instant};
use isocountry::CountryCode;
use crate::error::ParseError;
use crate::config::{Configuration, ParserConfig};

/// 主要的解析函数
pub fn parse_country_code(text: &str) -> Result<CountryCode, ParseError> {
    parse_country_code_with_config(text, &ParserConfig::default())
}

/// 使用配置的解析函数
pub fn parse_country_code_with_config(
    text: &str,
    config: &ParserConfig,
) -> Result<CountryCode, ParseError> {
    let start_time = Instant::now();
    
    // 输入验证
    if text.trim().is_empty() {
        return Err(ParseError::invalid_input("输入文本为空"));
    }
    
    if text.len() > 1024 {
        return Err(ParseError::invalid_input("输入文本过长"));
    }
    
    // 加载配置
    let config_data = Configuration::load()
        .map_err(|e| ParseError::config_error(&format!("配置加载失败: {}", e)))?;
    
    // 创建国家代码映射
    let country_mapping = config_data.create_country_mapping();
    
    // 预处理文本
    let processed_text = if config.case_sensitive {
        text.to_string()
    } else {
        text.to_lowercase()
    };
    
    // 多阶段解析
    let result = match parse_iso_codes(&processed_text, &country_mapping, config) {
        Ok(code) => Ok(code),
        Err(_) => match parse_chinese_names(&processed_text, &country_mapping, config) {
            Ok(code) => Ok(code),
            Err(_) => parse_pattern_matching(&processed_text, &country_mapping, config),
        },
    };
    
    // 检查超时
    let elapsed = start_time.elapsed();
    if elapsed > Duration::from_millis(config.timeout_ms) {
        return Err(ParseError::timeout(config.timeout_ms));
    }
    
    result
}

/// 解析ISO代码（alpha-2和alpha-3）
fn parse_iso_codes(
    text: &str,
    mapping: &HashMap<String, &crate::config::CountryInfo>,
    config: &ParserConfig,
) -> Result<CountryCode, ParseError> {
    // 查找2字母代码
    for i in 0..text.len().saturating_sub(1) {
        let slice = &text[i..i+2];
        if let Some(country_info) = mapping.get(slice) {
            if is_valid_iso_code_position(text, i, 2) {
                return CountryCode::from_str(&country_info.alpha2)
                    .map_err(|_| ParseError::not_found(text));
            }
        }
    }
    
    // 查找3字母代码
    for i in 0..text.len().saturating_sub(2) {
        let slice = &text[i..i+3];
        if let Some(country_info) = mapping.get(slice) {
            if is_valid_iso_code_position(text, i, 3) {
                return CountryCode::from_str(&country_info.alpha2)
                    .map_err(|_| ParseError::not_found(text));
            }
        }
    }
    
    Err(ParseError::not_found(text))
}

/// 解析中文国家名称
fn parse_chinese_names(
    text: &str,
    mapping: &HashMap<String, &crate::config::CountryInfo>,
    config: &ParserConfig,
) -> Result<CountryCode, ParseError> {
    // 简体中文名称匹配
    for (name, country_info) in mapping {
        if name.len() >= 2 && text.contains(name) {
            // 检查是否是有效的中文名称位置
            if is_valid_chinese_name_position(text, name) {
                return CountryCode::from_str(&country_info.alpha2)
                    .map_err(|_| ParseError::not_found(text));
            }
        }
    }
    
    Err(ParseError::not_found(text))
}

/// 模式匹配解析
fn parse_pattern_matching(
    text: &str,
    mapping: &HashMap<String, &crate::config::CountryInfo>,
    config: &ParserConfig,
) -> Result<CountryCode, ParseError> {
    // 简单的关键词匹配（模糊匹配）
    if config.fuzzy_match {
        let mut candidates = Vec::new();
        
        for (name, country_info) in mapping {
            // 检查名称是否在文本中出现
            if text.contains(name) && name.len() >= 2 {
                candidates.push(country_info.alpha2.clone());
            }
        }
        
        match candidates.len() {
            0 => Err(ParseError::not_found(text)),
            1 => CountryCode::from_str(&candidates[0])
                .map_err(|_| ParseError::not_found(text)),
            _ => Err(ParseError::ambiguous(text, candidates)),
        }
    } else {
        Err(ParseError::not_found(text))
    }
}

/// 检查是否是有效的ISO代码位置
fn is_valid_iso_code_position(text: &str, start: usize, length: usize) -> bool {
    // 检查前面是否是边界或分隔符
    if start > 0 {
        let prev_char = text.chars().nth(start - 1).unwrap();
        if !is_boundary_char(prev_char) {
            return false;
        }
    }
    
    // 检查后面是否是边界或分隔符
    if start + length < text.len() {
        let next_char = text.chars().nth(start + length).unwrap();
        if !is_boundary_char(next_char) {
            return false;
        }
    }
    
    true
}

/// 检查是否是有效的中文名称位置
fn is_valid_chinese_name_position(text: &str, name: &str) -> bool {
    // 简单的边界检查
    if let Some(start) = text.find(name) {
        // 检查前面是否是边界
        if start > 0 {
            let prev_char = text.chars().nth(start - 1).unwrap();
            if !is_boundary_char(prev_char) {
                return false;
            }
        }
        
        // 检查后面是否是边界
        let end = start + name.len();
        if end < text.len() {
            let next_char = text.chars().nth(end).unwrap();
            if !is_boundary_char(next_char) {
                return false;
            }
        }
        
        true
    } else {
        false
    }
}

/// 检查字符是否是边界字符
fn is_boundary_char(c: char) -> bool {
    c.is_whitespace() || c == '@' || c == '【' || c == '[' || c == '#' || 
    c == ']' || c == '】' || c == ' ' || c == '\t' || c == '\n'
}