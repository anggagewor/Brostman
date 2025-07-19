// src/http.rs
use reqwest::blocking::Client;

pub struct HttpResponse {
    pub status: String,
    pub body: String,
    pub content_type: Option<String>,
}

pub fn send_request(url: &str, method: &str) -> HttpResponse {
    let client = Client::new();

    let response_result = match method {
        "GET" => client.get(url).send(),
        "POST" => client.post(url).send(),
        "PUT" => client.put(url).send(),
        "DELETE" => client.delete(url).send(),
        "PATCH" => client.patch(url).send(),
        _ => {
            return HttpResponse {
                status: "Invalid Method".into(),
                body: format!("Unsupported method: {}", method),
                content_type: None,
            }
        }
    };

    match response_result {
        Ok(resp) => {
            let status = resp.status().to_string();
            let content_type = resp
                .headers()
                .get(reqwest::header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            let body = resp.text().unwrap_or_else(|e| format!("Failed to read body: {}", e));

            HttpResponse {
                status,
                body,
                content_type,
            }
        }
        Err(e) => HttpResponse {
            status: "Error".into(),
            body: format!("Request failed: {}", e),
            content_type: None,
        },
    }
}
