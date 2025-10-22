use thiserror::Error;

/// 解析错误类型
#[derive(Error, Debug, Clone)]
pub enum ParseError {
    /// 未找到匹配的国家代码
    #[error("未找到匹配的国家代码: {text}")]
    NotFound {
        text: String,
    },
    
    /// 匹配到多个可能的国家代码
    #[error("匹配到多个可能的国家代码: {text}, 候选: {candidates:?}")]
    Ambiguous {
        text: String,
        candidates: Vec<String>,
    },
    
    /// 输入文本格式无效
    #[error("输入文本格式无效: {text}")]
    InvalidInput {
        text: String,
    },
    
    /// 解析超时
    #[error("解析超时: 超过 {timeout_ms}ms 限制")]
    Timeout {
        timeout_ms: u64,
    },
    
    /// 配置错误
    #[error("配置错误: {message}")]
    ConfigError {
        message: String,
    },
}

impl ParseError {
    /// 创建未找到错误
    pub fn not_found(text: &str) -> Self {
        ParseError::NotFound {
            text: text.to_string(),
        }
    }
    
    /// 创建模糊匹配错误
    pub fn ambiguous(text: &str, candidates: Vec<String>) -> Self {
        ParseError::Ambiguous {
            text: text.to_string(),
            candidates,
        }
    }
    
    /// 创建无效输入错误
    pub fn invalid_input(text: &str) -> Self {
        ParseError::InvalidInput {
            text: text.to_string(),
        }
    }
    
    /// 创建超时错误
    pub fn timeout(timeout_ms: u64) -> Self {
        ParseError::Timeout { timeout_ms }
    }
    
    /// 创建配置错误
    pub fn config_error(message: &str) -> Self {
        ParseError::ConfigError {
            message: message.to_string(),
        }
    }
}