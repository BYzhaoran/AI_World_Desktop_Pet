use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)] pub struct ProviderConfig { pub base_url: String, pub model: String, pub api_key: String, pub language: String }
#[derive(Serialize)] struct RequestBody<'a> { model: &'a str, messages: Vec<Message<'a>>, temperature: f32, response_format: ResponseFormat }
#[derive(Serialize)] struct Message<'a> { role: &'a str, content: &'a str }
#[derive(Serialize)] struct ResponseFormat { r#type: &'static str }
#[derive(Deserialize)] struct ResponseBody { choices: Vec<Choice> }
#[derive(Deserialize)] struct Choice { message: ResponseMessage }
#[derive(Deserialize)] struct ResponseMessage { content: String }

pub async fn generate(config: ProviderConfig, prompt: String) -> Result<String, String> {
    if config.base_url.trim().is_empty() || config.model.trim().is_empty() { return Err("Base URL and model are required".into()); }
    let endpoint = format!("{}/chat/completions", config.base_url.trim_end_matches('/'));
    let client = Client::builder().timeout(std::time::Duration::from_secs(35)).build().map_err(|e| e.to_string())?;
    let system_prompt = if config.language == "en" { "You are an event director. Return only valid JSON. Prefer concise English summaries." } else { "你是持久世界的事件导演。只返回合法 JSON。优先使用简体中文，summary 使用 20-80 个中文字符；只有用户明确要求 English 时才使用英文。不得直接修改数据库。" };
    let mut request = client.post(endpoint).json(&RequestBody { model: &config.model, messages: vec![Message { role: "system", content: system_prompt }, Message { role: "user", content: &prompt }], temperature: .7, response_format: ResponseFormat { r#type: "json_object" } });
    if !config.api_key.trim().is_empty() { request = request.bearer_auth(config.api_key); }
    let response = request.send().await.map_err(|e| if e.is_timeout() { "provider timeout".into() } else { e.to_string() })?;
    let status = response.status(); let body = response.text().await.map_err(|e| e.to_string())?;
    if !status.is_success() { return Err(format!("provider returned HTTP {}", status.as_u16())); }
    let parsed: ResponseBody = serde_json::from_str(&body).map_err(|_| "provider returned invalid JSON".to_string())?;
    parsed.choices.into_iter().next().map(|choice| choice.message.content).ok_or_else(|| "provider returned no choices".into())
}

pub async fn test_connection(config: ProviderConfig) -> Result<String, String> { let _ = generate(config, "返回 {\"type\":\"no_event\"}。".into()).await?; Ok("Connection successful".into()) }

pub fn parse_proposal(raw: &str) -> Result<serde_json::Value, String> {
    let cleaned = raw.trim().trim_start_matches("```json").trim_start_matches("```").trim_end_matches("```").trim();
    serde_json::from_str(cleaned).map_err(|_| "LLM event was not valid JSON".into())
}

#[cfg(test)]
mod tests {
    use super::parse_proposal;
    #[test] fn parses_fenced_json() { assert_eq!(parse_proposal("```json\n{\"type\":\"no_event\"}\n```").unwrap()["type"], "no_event"); }
    #[test] fn rejects_invalid_json() { assert!(parse_proposal("not json").is_err()); }
}
