#[allow(dead_code)]
pub fn invalid_info_hashes() -> Vec<String> {
    [
        "0".to_string(),
        "-1".to_string(),
        "1.1".to_string(),
        "INVALID INFOHASH".to_string(),
        "9c38422213e30bff212b30c360d26f9a0213642".to_string(), // 39-char length instead of 40
        "9c38422213e30bff212b30c360d26f9a0213642&".to_string(), // Invalid char
    ]
    .to_vec()
}
