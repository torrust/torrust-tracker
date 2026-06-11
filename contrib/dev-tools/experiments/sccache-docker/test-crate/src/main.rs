fn main() {
    let data = serde_json::json!({"hello": "world"});
    println!("{}", serde_json::to_string_pretty(&data).unwrap());

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        println!("Tokio runtime works");
    });
}
