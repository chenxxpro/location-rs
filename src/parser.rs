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
    _config: &ParserConfig,
) -> Result<CountryInfo, ParseError> {
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
    
    // 注意：这里不再需要预处理文本变量，因为我们直接在各个匹配函数中处理
    
    // 按照指定优先级进行匹配：别名和简称 > 繁体中文名 > 简体中文名 > 英文名 > alpha3 > alpha2
    // 首先尝试精确匹配
    match parse_abbreviations(text, &config_data) {
        Ok(country_info) => return Ok(country_info),
        Err(_) => {},
    }
    
    match parse_traditional_chinese_names(text, &config_data) {
        Ok(country_info) => return Ok(country_info),
        Err(_) => {},
    }
    
    match parse_simplified_chinese_names(text, &config_data) {
        Ok(country_info) => return Ok(country_info),
        Err(_) => {},
    }
    
    match parse_english_names(text, &config_data) {
        Ok(country_info) => return Ok(country_info),
        Err(_) => {},
    }
    
    // 对于代码匹配，我们需要支持更灵活的情况，包括数字后缀
    // 首先尝试 alpha3 代码（3字符）
    let chars: Vec<char> = text.chars().collect();
    for i in 0..chars.len().saturating_sub(2) {
        if chars[i].is_alphabetic() && chars[i+1].is_alphabetic() && chars[i+2].is_alphabetic() {
            let slice: String = chars[i..i+3].iter().collect();
            let slice_upper = slice.to_uppercase();
            
            for country in config_data.get_countries() {
                if country.alpha3 == slice_upper {
                    // 检查是否是有效的ISO代码位置，允许后面跟着数字
                    let mut valid = true;
                    if i > 0 {
                        let prev_char = chars[i-1];
                        if !is_boundary_char(prev_char) && !prev_char.is_numeric() {
                            valid = false;
                        }
                    }
                    
                    if valid {
                        return Ok(country.clone());
                    }
                }
            }
        }
    }
    
    // 然后尝试 alpha2 代码（2字符）
    for i in 0..chars.len().saturating_sub(1) {
        if chars[i].is_alphabetic() && chars[i+1].is_alphabetic() {
            let slice: String = chars[i..i+2].iter().collect();
            let slice_upper = slice.to_uppercase();
            
            for country in config_data.get_countries() {
                if country.alpha2 == slice_upper {
                    // 检查是否是有效的ISO代码位置，允许后面跟着数字
                    let mut valid = true;
                    if i > 0 {
                        let prev_char = chars[i-1];
                        if !is_boundary_char(prev_char) && !prev_char.is_numeric() {
                            valid = false;
                        }
                    }
                    
                    if valid {
                        return Ok(country.clone());
                    }
                }
            }
        }
    }
    
    // 最后，尝试使用简化的边界检查再次匹配ISO代码
    for country in config_data.get_countries() {
        // 检查 alpha3 代码
        if text.to_uppercase().contains(&country.alpha3) {
            let pattern = country.alpha3.to_string();
            if let Some(pos) = text.to_uppercase().find(&pattern) {
                let start = pos;
                let end = pos + pattern.len();
                
                // 简化的边界检查
                let prev_valid = start == 0 || is_boundary_char(text.chars().nth(start-1).unwrap_or(' '));
                let next_valid = end >= text.len() || is_boundary_char(text.chars().nth(end).unwrap_or(' ')) || text.chars().nth(end).unwrap_or(' ').is_numeric();
                
                if prev_valid && next_valid {
                    return Ok(country.clone());
                }
            }
        }
        
        // 检查 alpha2 代码
        if text.to_uppercase().contains(&country.alpha2) {
            let pattern = country.alpha2.to_string();
            if let Some(pos) = text.to_uppercase().find(&pattern) {
                let start = pos;
                let end = pos + pattern.len();
                
                // 简化的边界检查
                let prev_valid = start == 0 || is_boundary_char(text.chars().nth(start-1).unwrap_or(' '));
                let next_valid = end >= text.len() || is_boundary_char(text.chars().nth(end).unwrap_or(' ')) || text.chars().nth(end).unwrap_or(' ').is_numeric();
                
                if prev_valid && next_valid {
                    return Ok(country.clone());
                }
            }
        }
    }
    
    Err(ParseError::not_found(text))
}

/// 解析别名和简称
fn parse_abbreviations(
    text: &str,
    config_data: &Configuration,
) -> Result<CountryInfo, ParseError> {
    let processed_text = text.to_lowercase();
    
    for country in config_data.get_countries() {
        for abbr in &country.abbreviations {
            if processed_text.contains(&abbr.to_lowercase()) {
                return Ok(country.clone());
            }
        }
    }
    
    Err(ParseError::not_found(text))
}

/// 解析繁体中文名称
fn parse_traditional_chinese_names(
    text: &str,
    config_data: &Configuration,
) -> Result<CountryInfo, ParseError> {
    for country in config_data.get_countries() {
        if text.contains(&country.name_zh_tw) {
            return Ok(country.clone());
        }
    }
    
    Err(ParseError::not_found(text))
}

/// 解析简体中文名称
fn parse_simplified_chinese_names(
    text: &str,
    config_data: &Configuration,
) -> Result<CountryInfo, ParseError> {
    for country in config_data.get_countries() {
        if text.contains(&country.name_zh_cn) {
            return Ok(country.clone());
        }
    }
    
    Err(ParseError::not_found(text))
}

/// 解析英文名称
fn parse_english_names(
    text: &str,
    config_data: &Configuration,
) -> Result<CountryInfo, ParseError> {
    let processed_text = text.to_lowercase();
    
    for country in config_data.get_countries() {
        if processed_text.contains(&country.name_en.to_lowercase()) {
            return Ok(country.clone());
        }
    }
    
    Err(ParseError::not_found(text))
}



// 不再需要pattern_matching函数，已被新的匹配算法替代

/// 检查字符是否是边界字符
fn is_boundary_char(c: char) -> bool {
    c.is_whitespace() || c == '@' || c == '【' || c == '[' || c == '#' || 
    c == ']' || c == '】' || c == ' ' || c == '\t' || c == '\n'
}