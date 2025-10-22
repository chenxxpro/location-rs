use std::collections::HashMap;
use std::time::{Duration, Instant};
use crate::error::ParseError;
use crate::config::{Configuration, CountryInfo};
use crate::ParserConfig;

/// 主要的解析函数
pub fn parse_country_code(text: &str) -> Result<CountryInfo, ParseError> {
    parse_country_code_with_config(text, &ParserConfig::default())
}

/// 使用配置的解析函数
pub fn parse_country_code_with_config(
    text: &str,
    config: &ParserConfig,
) -> Result<CountryInfo, ParseError> {
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
        Ok(country_info) => Ok(country_info),
        Err(_) => match parse_chinese_names(&processed_text, &country_mapping, config) {
            Ok(country_info) => Ok(country_info),
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
    mapping: &HashMap<String, &CountryInfo>,
    _config: &ParserConfig,
) -> Result<CountryInfo, ParseError> {
    let chars: Vec<char> = text.chars().collect();
    
    // 查找2字母代码
    for i in 0..chars.len().saturating_sub(1) {
        if chars[i].is_alphabetic() && chars[i+1].is_alphabetic() {
            let slice: String = chars[i..i+2].iter().collect();
            if let Some(country_info) = mapping.get(&slice.to_uppercase()) {
                if is_valid_iso_code_position_chars(&chars, i, 2) {
                    return Ok((*country_info).clone());
                }
            }
        }
    }
    
    // 查找3字母代码
    for i in 0..chars.len().saturating_sub(2) {
        if chars[i].is_alphabetic() && chars[i+1].is_alphabetic() && chars[i+2].is_alphabetic() {
            let slice: String = chars[i..i+3].iter().collect();
            if let Some(country_info) = mapping.get(&slice.to_uppercase()) {
                if is_valid_iso_code_position_chars(&chars, i, 3) {
                    return Ok((*country_info).clone());
                }
            }
        }
    }
    
    Err(ParseError::not_found(text))
}

/// 解析中文国家名称
fn parse_chinese_names(
    text: &str,
    mapping: &HashMap<String, &CountryInfo>,
    _config: &ParserConfig,
) -> Result<CountryInfo, ParseError> {
    // 简体中文名称匹配
    for (name, country_info) in mapping {
        // 检查是否是中文字符串
        if name.chars().any(|c| c.is_alphabetic() && c as u32 > 255) && text.contains(name) {
            return Ok((*country_info).clone());
        }
    }
    
    Err(ParseError::not_found(text))
}



/// 检查是否是有效的ISO代码位置（使用字符索引）
fn is_valid_iso_code_position_chars(chars: &[char], start: usize, length: usize) -> bool {
    // 检查前面是否是边界或分隔符
    if start > 0 {
        let prev_char = chars[start - 1];
        if !is_boundary_char(prev_char) {
            return false;
        }
    }
    
    // 检查后面是否是边界或分隔符
    if start + length < chars.len() {
        let next_char = chars[start + length];
        if !is_boundary_char(next_char) {
            return false;
        }
    }
    
    true
}

/// 模式匹配解析
fn parse_pattern_matching(
    text: &str,
    mapping: &HashMap<String, &CountryInfo>,
    config: &ParserConfig,
) -> Result<CountryInfo, ParseError> {
    // 简单的关键词匹配（模糊匹配）
    if config.fuzzy_match {
        let mut candidates = Vec::new();
        let text_lower = text.to_lowercase();
        
        for (name, country_info) in mapping {
            let name_lower = name.to_lowercase();
            
            // 检查alpha-2代码
            if text_lower.contains(&name_lower) && name.len() == 2 && name.chars().all(char::is_uppercase) {
                candidates.push(((**country_info).clone(), 2)); // 2字母代码优先级更高
            }
            // 检查alpha-3代码
            else if text_lower.contains(&name_lower) && name.len() == 3 && name.chars().all(char::is_uppercase) {
                candidates.push(((**country_info).clone(), 1));
            }
            // 检查中文名称
            else if text.contains(name) && name.chars().any(|c| c.is_alphabetic() && c as u32 > 255) {
                candidates.push(((**country_info).clone(), 3));
            }
            // 检查英文名称
            else if text_lower.contains(&name_lower) && name_lower.chars().all(|c| c.is_alphabetic() || c.is_whitespace()) {
                candidates.push(((**country_info).clone(), 4));
            }
        }
        
        if !candidates.is_empty() {
            // 按优先级排序
            candidates.sort_by_key(|&(_, priority)| priority);
            return Ok(candidates[0].0.clone());
        }
    }
    
    Err(ParseError::not_found(text))
}

/// 检查字符是否是边界字符
fn is_boundary_char(c: char) -> bool {
    c.is_whitespace() || c == '@' || c == '【' || c == '[' || c == '#' || 
    c == ']' || c == '】' || c == ' ' || c == '\t' || c == '\n'
}