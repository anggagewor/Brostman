use reqwest::blocking::Client;

pub fn send_request(url: &str, method: &str) -> String {
    let client = Client::new();
    let result = match method.to_uppercase().as_str() {
        "GET" => client.get(url).send(),
        "POST" => client.post(url).send(),
        _ => return "Method not supported".to_string(),
    };

    match result {
        Ok(res) => res.text().unwrap_or_else(|_| "Failed to read body".into()),
        Err(e) => format!("Error: {}", e),
    }
}
