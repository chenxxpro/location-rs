use location_rs::{parse_country_code, CountryCode};

#[test]
fn test_basic_iso_code_parsing() {
    // 测试基本的ISO代码解析
    assert_eq!(parse_country_code("US Node").unwrap(), CountryCode::USA);
    assert_eq!(parse_country_code("CN Server").unwrap(), CountryCode::CHN);
    assert_eq!(parse_country_code("HK Vip").unwrap(), CountryCode::HKG);
}

#[test]
fn test_chinese_name_parsing() {
    // 测试中文名称解析
    assert_eq!(parse_country_code("美国节点").unwrap(), CountryCode::USA);
    assert_eq!(parse_country_code("中国服务器").unwrap(), CountryCode::CHN);
    assert_eq!(parse_country_code("香港服务").unwrap(), CountryCode::HKG);
}

#[test]
fn test_mixed_patterns() {
    // 测试混合模式
    assert_eq!(parse_country_code("@HK Vip1").unwrap(), CountryCode::HKG);
    assert_eq!(parse_country_code("【SS】USA1").unwrap(), CountryCode::USA);
    assert_eq!(parse_country_code("【SS】US1").unwrap(), CountryCode::USA);
    assert_eq!(parse_country_code("V1 美国").unwrap(), CountryCode::USA);
}

#[test]
fn test_error_cases() {
    // 测试错误情况
    assert!(parse_country_code("普通标题").is_err());
    assert!(parse_country_code("12345").is_err());
    assert!(parse_country_code("").is_err());
}

#[test]
fn test_case_insensitive() {
    // 测试大小写不敏感
    assert_eq!(parse_country_code("us node").unwrap(), CountryCode::USA);
    assert_eq!(parse_country_code("cn server").unwrap(), CountryCode::CHN);
}

#[test]
fn test_edge_cases() {
    // 测试边界情况
    assert_eq!(parse_country_code("US").unwrap(), CountryCode::USA);
    assert_eq!(parse_country_code("美国").unwrap(), CountryCode::USA);
    assert_eq!(parse_country_code("United States").unwrap(), CountryCode::USA);
}