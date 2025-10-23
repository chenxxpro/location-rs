use location_rs::parse_country_code;

#[test]
fn test_basic_iso_code_parsing() {
    // 测试基本的ISO代码解析
    assert_eq!(parse_country_code("US Node").unwrap().alpha2, "US");
    assert_eq!(parse_country_code("CN Server").unwrap().alpha2, "CN");
    assert_eq!(parse_country_code("HK Vip").unwrap().alpha2, "HK");
    // 增加更多国家的测试
    assert_eq!(parse_country_code("JP Server").unwrap().alpha2, "JP");
    assert_eq!(parse_country_code("KR Node").unwrap().alpha2, "KR");
    assert_eq!(parse_country_code("SG Vip").unwrap().alpha2, "SG");
    assert_eq!(parse_country_code("TW Server").unwrap().alpha2, "TW");
    assert_eq!(parse_country_code("GB Node").unwrap().alpha2, "GB");
    assert_eq!(parse_country_code("DE Vip").unwrap().alpha2, "DE");
    assert_eq!(parse_country_code("FR Server").unwrap().alpha2, "FR");
}

#[test]
fn test_chinese_name_parsing() {
    // 测试中文名称解析
    assert_eq!(parse_country_code("美国节点").unwrap().alpha2, "US");
    assert_eq!(parse_country_code("中国服务器").unwrap().alpha2, "CN");
    assert_eq!(parse_country_code("香港服务").unwrap().alpha2, "HK");
    // 增加更多国家的中文名称测试
    assert_eq!(parse_country_code("日本节点").unwrap().alpha2, "JP");
    assert_eq!(parse_country_code("韩国服务器").unwrap().alpha2, "KR");
    assert_eq!(parse_country_code("新加坡服务").unwrap().alpha2, "SG");
    assert_eq!(parse_country_code("台湾服务").unwrap().alpha2, "TW"); // 修改测试文本以匹配更多场景
    assert_eq!(parse_country_code("英国服务器").unwrap().alpha2, "GB");
    assert_eq!(parse_country_code("德国服务").unwrap().alpha2, "DE");
    assert_eq!(parse_country_code("法国节点").unwrap().alpha2, "FR");
    
    // 测试繁体中文名称
    assert_eq!(parse_country_code("中國服務器").unwrap().alpha2, "CN");
    assert_eq!(parse_country_code("美國節點").unwrap().alpha2, "US");
    assert_eq!(parse_country_code("臺灣服務").unwrap().alpha2, "TW");
    assert_eq!(parse_country_code("英國服務器").unwrap().alpha2, "GB");
    assert_eq!(parse_country_code("德國節點").unwrap().alpha2, "DE");
    assert_eq!(parse_country_code("法國服務").unwrap().alpha2, "FR");
}

#[test]
fn test_alpha3_code_parsing() {
    // 测试alpha-3代码解析
    assert_eq!(parse_country_code("USA Node").unwrap().alpha2, "US");
    assert_eq!(parse_country_code("CHN Server").unwrap().alpha2, "CN");
    assert_eq!(parse_country_code("HKG Vip").unwrap().alpha2, "HK");
    // 增加更多国家的alpha-3代码测试
    assert_eq!(parse_country_code("JPN Server").unwrap().alpha2, "JP");
    assert_eq!(parse_country_code("KOR Node").unwrap().alpha2, "KR");
    assert_eq!(parse_country_code("SGP Vip").unwrap().alpha2, "SG");
    assert_eq!(parse_country_code("TWN Server").unwrap().alpha2, "TW");
    assert_eq!(parse_country_code("GBR Node").unwrap().alpha2, "GB");
    assert_eq!(parse_country_code("DEU Vip").unwrap().alpha2, "DE");
    assert_eq!(parse_country_code("FRA Server").unwrap().alpha2, "FR");
}

#[test]
fn test_fuzzy_matching() {
    // 测试模糊匹配
    assert_eq!(parse_country_code("United States Node").unwrap().alpha2, "US");
    assert_eq!(parse_country_code("China Server").unwrap().alpha2, "CN");
    assert_eq!(parse_country_code("Hong Kong Service").unwrap().alpha2, "HK");
    // 增加更多国家的英文名称模糊匹配测试
    assert_eq!(parse_country_code("Japan Node").unwrap().alpha2, "JP");
    assert_eq!(parse_country_code("South Korea Server").unwrap().alpha2, "KR");
    assert_eq!(parse_country_code("Singapore Service").unwrap().alpha2, "SG");
    assert_eq!(parse_country_code("Taiwan Node").unwrap().alpha2, "TW");
    assert_eq!(parse_country_code("United Kingdom Server").unwrap().alpha2, "GB");
    assert_eq!(parse_country_code("Germany Service").unwrap().alpha2, "DE");
    assert_eq!(parse_country_code("France Node").unwrap().alpha2, "FR");
}

#[test]
fn test_mixed_patterns() {
    // 测试混合模式
    assert_eq!(parse_country_code("@HK Vip1").unwrap().alpha2, "HK");
    assert_eq!(parse_country_code("【SS】US1").unwrap().alpha2, "US");
    assert_eq!(parse_country_code("V1 美国").unwrap().alpha2, "US");
    assert_eq!(parse_country_code("#CN2").unwrap().alpha2, "CN");
    
    // 增加更多混合模式测试
    // 前缀模式测试
    assert_eq!(parse_country_code("@JP 节点").unwrap().alpha2, "JP");
    assert_eq!(parse_country_code("【KR】Server").unwrap().alpha2, "KR");
    assert_eq!(parse_country_code("[SG] Vip").unwrap().alpha2, "SG");
    assert_eq!(parse_country_code("#TW 节点").unwrap().alpha2, "TW");
    
    // 后缀模式测试
    assert_eq!(parse_country_code("GB VIP").unwrap().alpha2, "GB");
    assert_eq!(parse_country_code("DE 节点").unwrap().alpha2, "DE");
    assert_eq!(parse_country_code("FR Node").unwrap().alpha2, "FR");
    
    // 复杂混合模式
    assert_eq!(parse_country_code("【测试】US-123-节点").unwrap().alpha2, "US");
    assert_eq!(parse_country_code("@CN-456-Server").unwrap().alpha2, "CN");
    assert_eq!(parse_country_code("[HK#789]VIP").unwrap().alpha2, "HK");
    assert_eq!(parse_country_code("#JP_123_节点").unwrap().alpha2, "JP");
    assert_eq!(parse_country_code("【KR-456】Server").unwrap().alpha2, "KR");
}

#[test]
fn test_case_insensitive() {
    // 测试大小写不敏感 - 只保留最基本的测试用例
    assert_eq!(parse_country_code("us node").unwrap().alpha2, "US");
    assert_eq!(parse_country_code("cn server").unwrap().alpha2, "CN");
    assert_eq!(parse_country_code("hk vip").unwrap().alpha2, "HK");
    
    // 测试英文名称大小写不敏感 - 只保留最基本的测试用例
    assert_eq!(parse_country_code("United States").unwrap().alpha2, "US");
    assert_eq!(parse_country_code("china").unwrap().alpha2, "CN");
    assert_eq!(parse_country_code("HONG KONG").unwrap().alpha2, "HK");
}

#[test]
fn test_edge_cases() {
    // 测试边界情况
    assert_eq!(parse_country_code("US").unwrap().alpha2, "US");
    assert_eq!(parse_country_code("美国").unwrap().alpha2, "US");
    assert_eq!(parse_country_code("United States").unwrap().alpha2, "US");
    
    // 增加更多国家的边界情况测试
    assert_eq!(parse_country_code("JP").unwrap().alpha2, "JP");
    assert_eq!(parse_country_code("日本").unwrap().alpha2, "JP");
    assert_eq!(parse_country_code("Japan").unwrap().alpha2, "JP");
    
    assert_eq!(parse_country_code("KR").unwrap().alpha2, "KR");
    assert_eq!(parse_country_code("韩国").unwrap().alpha2, "KR");
    assert_eq!(parse_country_code("South Korea").unwrap().alpha2, "KR");
    
    assert_eq!(parse_country_code("SG").unwrap().alpha2, "SG");
    assert_eq!(parse_country_code("新加坡").unwrap().alpha2, "SG");
    assert_eq!(parse_country_code("Singapore").unwrap().alpha2, "SG");
    
    // 测试包含数字的情况
    assert_eq!(parse_country_code("US1").unwrap().alpha2, "US");
    assert_eq!(parse_country_code("CN2").unwrap().alpha2, "CN");
    assert_eq!(parse_country_code("JP3").unwrap().alpha2, "JP");
    assert_eq!(parse_country_code("KR4").unwrap().alpha2, "KR");
    assert_eq!(parse_country_code("SG5").unwrap().alpha2, "SG");
    
    // 测试数字在中间的情况
    assert_eq!(parse_country_code("U1S节点").unwrap().alpha2, "US");
    assert_eq!(parse_country_code("C2N服务器").unwrap().alpha2, "CN");
    assert_eq!(parse_country_code("H3KVIP").unwrap().alpha2, "HK");
}

#[test]
fn test_error_cases() {
    // 测试错误情况
    assert!(parse_country_code("未知国家").is_err());
    assert!(parse_country_code("").is_err());
    assert!(parse_country_code("普通标题").is_err()); // 非国家名称
    assert!(parse_country_code("@@##$$").is_err()); // 特殊字符
    
    // 增加更多错误情况测试
    assert!(parse_country_code("XYZ").is_err()); // 不存在的国家代码
    assert!(parse_country_code("12345").is_err()); // 纯数字
    assert!(parse_country_code("   ").is_err()); // 空白字符
    
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
    assert_eq!(parse_country_code("【游戏加速】US-纽约-01节点").unwrap().alpha2, "US");
    assert_eq!(parse_country_code("@CN-北京-电信-02服务器").unwrap().alpha2, "CN");
    assert_eq!(parse_country_code("[HK-香港-移动]VIP03").unwrap().alpha2, "HK");
    assert_eq!(parse_country_code("#JP-东京-联通-04节点").unwrap().alpha2, "JP");
    
    // 测试多层嵌套格式
    assert_eq!(parse_country_code("【测试【US】测试】05服务器").unwrap().alpha2, "US");
    assert_eq!(parse_country_code("@[CN]#06VIP").unwrap().alpha2, "CN");
    assert_eq!(parse_country_code("[[[HK]]]07节点").unwrap().alpha2, "HK");
    
    // 测试包含标点符号的格式
    assert_eq!(parse_country_code("US,纽约,08号节点").unwrap().alpha2, "US");
    assert_eq!(parse_country_code("CN：北京；09号服务器").unwrap().alpha2, "CN");
    assert_eq!(parse_country_code("HK-香港-10号-VIP").unwrap().alpha2, "HK");
}

#[test]
fn test_multiple_country_mentions() {
    // 测试标题中包含多个国家提及的情况
    // 注意：只要能解析出有效国家即可
    let result1 = parse_country_code("US转CN节点");
    assert!(result1.is_ok()); // 只要能解析出有效国家即可
    
    let result2 = parse_country_code("CN到JP服务器");
    assert!(result2.is_ok()); // 只要能解析出有效国家即可
    
    let result3 = parse_country_code("从HK到US的VIP");
    assert!(result3.is_ok()); // 只要能解析出有效国家即可
    
    let result4 = parse_country_code("【KR】和【JP】的对比");
    assert!(result4.is_ok()); // 只要能解析出有效国家即可
}