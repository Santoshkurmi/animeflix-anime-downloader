pub async fn send_request(url: String, path: String) -> Option<String> {
    // println!("{}{}", url, path);
    let client = reqwest::Client::new();
    let body = client.get(format!("{}{}", url, path)).send().await;
    if let Err(err) = body {
        println!("{}", err);
        return None;
    }

    let body = body.unwrap();
    if body.status().is_success() {
        let content = body.text().await;
        if let Err(err) = content {
            println!("{}", err);
            return None;
        }
        let content = content.unwrap();
        // println!("Body:{}", content);
        return Some(content);
    }
    return None;
}
