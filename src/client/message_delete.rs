pub fn post(pronto_base_url: &str, client: &reqwest::blocking::Client, message_id: u64) -> HashMap<String, bool> {
    let r = client.post(format!("{pronto_base_url}v1/message.delete"))
        .json(&json!({"message_id": message_id }))
        .send().unwrap();
    let json = r.json::<HashMap<String, bool>>().unwrap();
    json
}
