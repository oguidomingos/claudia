use reqwest::Client;
use serde::Serialize;

#[derive(Serialize)]
struct RequestBody<'a> {
    model: &'a str,
    prompt: &'a str,
}

pub async fn call_deepseek(
    api_key: &str,
    prompt: &str,
    endpoint: &str,
) -> Result<String, String> {
    let client = Client::new();
    let resp = client
        .post(format!("{}/generate", endpoint))
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&RequestBody {
            model: "DeepSeek-R1",
            prompt,
        })
        .send()
        .await
        .map_err(|e| e.to_string())?;

    resp.text().await.map_err(|e| e.to_string())
}
