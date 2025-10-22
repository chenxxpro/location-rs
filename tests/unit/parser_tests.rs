use location_rs::{parse_country_code, CountryCode, ParseError};

#[test]
fn test_basic_iso_code_parsing() {
    // 测试基本的ISO代码解析
    assert_eq!(parse_country_code("US Node").unwrap(), CountryCode::US);
    assert_eq!(parse_country_code("CN Server").unwrap(), CountryCode::CN);
    assert_eq!(parse_country_code("HK Vip").unwrap(), CountryCode::HK);
    // 增加更多国家的测试
    assert_eq!(parse_country_code("JP Server").unwrap(), CountryCode::JP);
    assert_eq!(parse_country_code("KR Node").unwrap(), CountryCode::KR);
    assert_eq!(parse_country_code("SG Vip").unwrap(), CountryCode::SG);
    assert_eq!(parse_country_code("TW Server").unwrap(), CountryCode::TW);
    assert_eq!(parse_country_code("GB Node").unwrap(), CountryCode::GB);
    assert_eq!(parse_country_code("DE Vip").unwrap(), CountryCode::DE);
    assert_eq!(parse_country_code("FR Server").unwrap(), CountryCode::FR);
}

#[test]
fn test_chinese_name_parsing() {
    // 测试中文名称解析
    assert_eq!(parse_country_code("美国节点").unwrap(), CountryCode::US);
    assert_eq!(parse_country_code("中国服务器").unwrap(), CountryCode::CN);
    assert_eq!(parse_country_code("香港服务").unwrap(), CountryCode::HK);
    // 增加更多国家的中文名称测试
    assert_eq!(parse_country_code("日本节点").unwrap(), CountryCode::JP);
    assert_eq!(parse_country_code("韩国服务器").unwrap(), CountryCode::KR);
    assert_eq!(parse_country_code("新加坡服务").unwrap(), CountryCode::SG);
    assert_eq!(parse_country_code("台湾节点").unwrap(), CountryCode::TW);
    assert_eq!(parse_country_code("英国服务器").unwrap(), CountryCode::GB);
    assert_eq!(parse_country_code("德国服务").unwrap(), CountryCode::DE);
    assert_eq!(parse_country_code("法国节点").unwrap(), CountryCode::FR);
    
    // 测试繁体中文名称
    assert_eq!(parse_country_code("中國服務器").unwrap(), CountryCode::CN);
    assert_eq!(parse_country_code("美國節點").unwrap(), CountryCode::US);
    assert_eq!(parse_country_code("臺灣服務").unwrap(), CountryCode::TW);
    assert_eq!(parse_country_code("英國服務器").unwrap(), CountryCode::GB);
    assert_eq!(parse_country_code("德國節點").unwrap(), CountryCode::DE);
    assert_eq!(parse_country_code("法國服務").unwrap(), CountryCode::FR);
}

#[test]
fn test_alpha3_code_parsing() {
    // 测试alpha-3代码解析
    assert_eq!(parse_country_code("USA Node").unwrap(), CountryCode::US);
    assert_eq!(parse_country_code("CHN Server").unwrap(), CountryCode::CN);
    assert_eq!(parse_country_code("HKG Vip").unwrap(), CountryCode::HK);
    // 增加更多国家的alpha-3代码测试
    assert_eq!(parse_country_code("JPN Server").unwrap(), CountryCode::JP);
    assert_eq!(parse_country_code("KOR Node").unwrap(), CountryCode::KR);
    assert_eq!(parse_country_code("SGP Vip").unwrap(), CountryCode::SG);
    assert_eq!(parse_country_code("TWN Server").unwrap(), CountryCode::TW);
    assert_eq!(parse_country_code("GBR Node").unwrap(), CountryCode::GB);
    assert_eq!(parse_country_code("DEU Vip").unwrap(), CountryCode::DE);
    assert_eq!(parse_country_code("FRA Server").unwrap(), CountryCode::FR);
}

#[test]
fn test_fuzzy_matching() {
    // 测试模糊匹配
    assert_eq!(parse_country_code("United States Node").unwrap(), CountryCode::US);
    assert_eq!(parse_country_code("China Server").unwrap(), CountryCode::CN);
    assert_eq!(parse_country_code("Hong Kong Service").unwrap(), CountryCode::HK);
    // 增加更多国家的英文名称模糊匹配测试
    assert_eq!(parse_country_code("Japan Node").unwrap(), CountryCode::JP);
    assert_eq!(parse_country_code("South Korea Server").unwrap(), CountryCode::KR);
    assert_eq!(parse_country_code("Singapore Service").unwrap(), CountryCode::SG);
    assert_eq!(parse_country_code("Taiwan Node").unwrap(), CountryCode::TW);
    assert_eq!(parse_country_code("United Kingdom Server").unwrap(), CountryCode::GB);
    assert_eq!(parse_country_code("Germany Service").unwrap(), CountryCode::DE);
    assert_eq!(parse_country_code("France Node").unwrap(), CountryCode::FR);
}

#[test]
fn test_mixed_patterns() {
    // 测试混合模式
    assert_eq!(parse_country_code("@HK Vip1").unwrap(), CountryCode::HK);
    assert_eq!(parse_country_code("【SS】US1").unwrap(), CountryCode::US);
    assert_eq!(parse_country_code("V1 美国").unwrap(), CountryCode::US);
    assert_eq!(parse_country_code("#CN2").unwrap(), CountryCode::CN);
    
    // 增加更多混合模式测试
    // 前缀模式测试
    assert_eq!(parse_country_code("@JP 节点").unwrap(), CountryCode::JP);
    assert_eq!(parse_country_code("【KR】Server").unwrap(), CountryCode::KR);
    assert_eq!(parse_country_code("[SG] Vip").unwrap(), CountryCode::SG);
    assert_eq!(parse_country_code("#TW 节点").unwrap(), CountryCode::TW);
    
    // 后缀模式测试
    assert_eq!(parse_country_code("GB VIP").unwrap(), CountryCode::GB);
    assert_eq!(parse_country_code("DE 节点").unwrap(), CountryCode::DE);
    assert_eq!(parse_country_code("FR Node").unwrap(), CountryCode::FR);
    
    // 复杂混合模式
    assert_eq!(parse_country_code("【测试】US-123-节点").unwrap(), CountryCode::US);
    assert_eq!(parse_country_code("@CN-456-Server").unwrap(), CountryCode::CN);
    assert_eq!(parse_country_code("[HK#789]VIP").unwrap(), CountryCode::HK);
    assert_eq!(parse_country_code("#JP_123_节点").unwrap(), CountryCode::JP);
    assert_eq!(parse_country_code("【KR-456】Server").unwrap(), CountryCode::KR);
}

#[test]
fn test_case_insensitive() {
    // 测试大小写不敏感
    assert_eq!(parse_country_code("us node").unwrap(), CountryCode::US);
    assert_eq!(parse_country_code("cn server").unwrap(), CountryCode::CN);
    assert_eq!(parse_country_code("hk vip").unwrap(), CountryCode::HK);
    // 增加更多大小写不敏感测试
    assert_eq!(parse_country_code("Jp nODE").unwrap(), CountryCode::JP);
    assert_eq!(parse_country_code("Kr sERVER").unwrap(), CountryCode::KR);
    assert_eq!(parse_country_code("sG ViP").unwrap(), CountryCode::SG);
    assert_eq!(parse_country_code("TW server").unwrap(), CountryCode::TW);
    assert_eq!(parse_country_code("Gb NODE").unwrap(), CountryCode::GB);
    assert_eq!(parse_country_code("dE vip").unwrap(), CountryCode::DE);
    assert_eq!(parse_country_code("Fr SERVER").unwrap(), CountryCode::FR);
    
    // 测试英文名称大小写不敏感
    assert_eq!(parse_country_code("japan node").unwrap(), CountryCode::JP);
    assert_eq!(parse_country_code("SOUTH KOREA server").unwrap(), CountryCode::KR);
    assert_eq!(parse_country_code("singapore VIP").unwrap(), CountryCode::SG);
}

#[test]
fn test_edge_cases() {
    // 测试边界情况
    assert_eq!(parse_country_code("US").unwrap(), CountryCode::US);
    assert_eq!(parse_country_code("美国").unwrap(), CountryCode::US);
    assert_eq!(parse_country_code("United States").unwrap(), CountryCode::US);
    
    // 增加更多国家的边界情况测试
    assert_eq!(parse_country_code("JP").unwrap(), CountryCode::JP);
    assert_eq!(parse_country_code("日本").unwrap(), CountryCode::JP);
    assert_eq!(parse_country_code("Japan").unwrap(), CountryCode::JP);
    
    assert_eq!(parse_country_code("KR").unwrap(), CountryCode::KR);
    assert_eq!(parse_country_code("韩国").unwrap(), CountryCode::KR);
    assert_eq!(parse_country_code("South Korea").unwrap(), CountryCode::KR);
    
    assert_eq!(parse_country_code("SG").unwrap(), CountryCode::SG);
    assert_eq!(parse_country_code("新加坡").unwrap(), CountryCode::SG);
    assert_eq!(parse_country_code("Singapore").unwrap(), CountryCode::SG);
    
    // 测试包含数字的情况
    assert_eq!(parse_country_code("US1").unwrap(), CountryCode::US);
    assert_eq!(parse_country_code("CN2").unwrap(), CountryCode::CN);
    assert_eq!(parse_country_code("JP3").unwrap(), CountryCode::JP);
    assert_eq!(parse_country_code("KR4").unwrap(), CountryCode::KR);
    assert_eq!(parse_country_code("SG5").unwrap(), CountryCode::SG);
    
    // 测试数字在中间的情况
    assert_eq!(parse_country_code("U1S节点").unwrap(), CountryCode::US);
    assert_eq!(parse_country_code("C2N服务器").unwrap(), CountryCode::CN);
    assert_eq!(parse_country_code("H3KVIP").unwrap(), CountryCode::HK);
}

#[test]
fn test_error_cases() {
    // 测试错误情况
    assert!(parse_country_code("普通标题").is_err());
    assert!(parse_country_code("12345").is_err());
    assert!(parse_country_code("").is_err());
    assert!(parse_country_code("   ").is_err());
    
    // 增加更多错误情况测试
    assert!(parse_country_code("未知国家").is_err());
    assert!(parse_country_code("XYZ").is_err()); // 不存在的国家代码
    assert!(parse_country_code("123Node").is_err()); // 纯数字开头
    assert!(parse_country_code("@@##$$").is_err()); // 只有特殊字符
    
    // 测试部分匹配但不是完整国家信息的情况
    assert!(parse_country_code("美国的").is_err()); // 中文名称后有额外字符
    assert!(parse_country_code("useful").is_err()); // 英文单词包含国家代码但不是国家代码
    assert!(parse_country_code("cannot").is_err()); // 英文单词包含国家代码但不是国家代码
    
    // 测试超长文本
    let long_text = "a".repeat(1025);
    assert!(parse_country_code(&long_text).is_err());
}

#[test]
fn test_complex_title_formats() {
    // 测试复杂标题格式
    assert_eq!(parse_country_code("【游戏加速】US-纽约-01节点").unwrap(), CountryCode::US);
    assert_eq!(parse_country_code("@CN-北京-电信-02服务器").unwrap(), CountryCode::CN);
    assert_eq!(parse_country_code("[HK-香港-移动]VIP03").unwrap(), CountryCode::HK);
    assert_eq!(parse_country_code("#JP-东京-联通-04节点").unwrap(), CountryCode::JP);
    
    // 测试多层嵌套格式
    assert_eq!(parse_country_code("【测试【US】测试】05服务器").unwrap(), CountryCode::US);
    assert_eq!(parse_country_code("@[CN]#06VIP").unwrap(), CountryCode::CN);
    assert_eq!(parse_country_code("[[[HK]]]07节点").unwrap(), CountryCode::HK);
    
    // 测试包含标点符号的格式
    assert_eq!(parse_country_code("US,纽约,08号节点").unwrap(), CountryCode::US);
    assert_eq!(parse_country_code("CN：北京；09号服务器").unwrap(), CountryCode::CN);
    assert_eq!(parse_country_code("HK-香港-10号-VIP").unwrap(), CountryCode::HK);
}

#[test]
fn test_multiple_country_mentions() {
    // 测试标题中包含多个国家提及的情况（应该返回第一个匹配的国家）
    assert_eq!(parse_country_code("US转CN节点").unwrap(), CountryCode::US);
    assert_eq!(parse_country_code("CN到JP服务器").unwrap(), CountryCode::CN);
    assert_eq!(parse_country_code("从HK到US的VIP").unwrap(), CountryCode::HK);
    assert_eq!(parse_country_code("【KR】和【JP】的对比").unwrap(), CountryCode::KR);
}