use crate::deepseek_client;

#[tauri::command]
pub async fn deepseek_generate(
    prompt: String,
    api_key: String,
) -> Result<String, String> {
    deepseek_client::call_deepseek(&api_key, &prompt, "https://api.deepseek.com/v1").await
}
