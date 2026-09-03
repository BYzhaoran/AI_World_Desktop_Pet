use reqwest::Client;
use serde::Serialize;
use serde_json::Value;

#[derive(Clone, Debug)] pub struct ProviderConfig { pub base_url: String, pub model: String, pub api_key: String, pub language: String }
#[derive(Serialize)] struct RequestBody<'a> { model: &'a str, messages: Vec<Message<'a>>, temperature: f32 }
#[derive(Serialize)] struct Message<'a> { role: &'a str, content: &'a str }
pub async fn generate(config: ProviderConfig, prompt: String) -> Result<String, String> {
    if config.base_url.trim().is_empty() || config.model.trim().is_empty() { return Err("Base URL and model are required".into()); }
    let base_url = config.base_url.trim().trim_end_matches('/');
    let endpoint = if base_url.ends_with("/chat/completions") { base_url.to_string() } else { format!("{base_url}/chat/completions") };
    eprintln!("[llm] request endpoint={} model={} api_key_present={} prompt_chars={}", endpoint, config.model, !config.api_key.trim().is_empty(), prompt.chars().count());
    let client = Client::builder().timeout(std::time::Duration::from_secs(120)).build().map_err(|e| e.to_string())?;
    let system_prompt = if config.language == "en" {
        "You are an event director for a persistent world. Return only valid JSON. Write summaries in natural, conversational second-person style beginning with You when appropriate. Keep ordinary events to 1-2 sentences and richer events to 3-5 sentences without repetition. Light humor, teasing, self-deprecation, and personality are welcome when natural. Do not modify the database."
    } else {
        "你是持久世界的事件导演。只返回合法 JSON，不要 Markdown 或解释。summary 必须以“你……”为主要开头，像角色在和玩家发消息、随口说今天发生了什么。必须使用简单、具体、生活化的口语，不要写成小说、散文或诗歌；少用形容词和比喻，避免“仿佛、渗进、立体感、微光、层层叠加”等文学化表达，优先描述实际做了什么、看到了什么、心里怎么吐槽。普通小事通常写 1～2 句话，稍有内容的事件自然写 3～5 句话，不要重复凑字数。可以根据角色性格适量加入吐槽、毒舌、自嘲和小幽默，但不要破坏事件因果或显得刻意。优先使用简体中文。不得直接修改数据库。"
    };
    let mut request = client.post(endpoint).json(&RequestBody { model: &config.model, messages: vec![Message { role: "system", content: system_prompt }, Message { role: "user", content: &prompt }], temperature: 0.7 });
    if !config.api_key.trim().is_empty() { request = request.bearer_auth(config.api_key); }
    let response = request.send().await.map_err(|e| {
        let message = if e.is_timeout() { "provider timeout".to_string() } else { e.to_string() };
        eprintln!("[llm] request failed: {}", message);
        message
    })?;
    let status = response.status(); let body = response.text().await.map_err(|e| e.to_string())?;
    eprintln!("[llm] response status={} body_chars={}", status.as_u16(), body.chars().count());
    if !status.is_success() {
        let detail = body.chars().take(300).collect::<String>();
        eprintln!("[llm] provider error body={}", detail);
        return Err(format!("provider returned HTTP {}: {}", status.as_u16(), detail));
    }
    let content = response_content(&body)?;
    eprintln!("[llm] content_chars={} preview={}", content.chars().count(), content.trim().chars().take(160).collect::<String>());
    Ok(content)
}

pub async fn test_connection(config: ProviderConfig) -> Result<String, String> {
    let content = generate(config, "Reply with OK.".into()).await?;
    let preview = content.trim().chars().take(120).collect::<String>();
    if preview.is_empty() {
        Err("provider returned empty message".into())
    } else {
        Ok(format!("Connection successful: {}", preview))
    }
}

pub fn parse_proposal(raw: &str) -> Result<serde_json::Value, String> {
    let cleaned = repair_common_json_typos(strip_fences(raw.trim()));
    if let Ok(value) = serde_json::from_str(&cleaned) {
        return Ok(value);
    }
    if let Some(candidate) = extract_json_block(&cleaned) {
        if let Ok(value) = serde_json::from_str(&repair_common_json_typos(candidate)) {
            return Ok(value);
        }
    }
    let error = format!("LLM event was not valid JSON: {}", cleaned.chars().take(200).collect::<String>());
    eprintln!("[llm] parse failed: {}", error);
    Err(error)
}

fn repair_common_json_typos(raw: &str) -> String {
    let mut repaired = raw.to_string();
    for key in ["event_type", "summary", "importance", "location", "effects", "participants", "causes", "memory", "relation", "thread_id", "title", "estimated_duration", "progress"] {
        repaired = repaired.replace(&format!("\"{}':", key), &format!("\"{}\":", key));
        repaired = repaired.replace(&format!("'{}':", key), &format!("\"{}\":", key));
    }
    repaired
}

fn response_content(body: &str) -> Result<String, String> {
    if let Ok(parsed) = serde_json::from_str::<Value>(body) {
        if let Some(error) = parsed.pointer("/error/message").and_then(value_to_text) {
            return Err(error);
        }
        if let Some(error) = parsed.pointer("/error").and_then(value_to_text) {
            return Err(error);
        }
        let candidates = [
            parsed.pointer("/choices/0/message/content"),
            parsed.pointer("/choices/0/text"),
            parsed.pointer("/content"),
            parsed.pointer("/text"),
            parsed.pointer("/output_text"),
            parsed.pointer("/response"),
        ];
        for candidate in candidates.into_iter().flatten() {
            if let Some(content) = value_to_text(candidate) {
                if !content.trim().is_empty() { return Ok(content); }
            }
        }
        return Err(format!("provider response missing message content: {}", body.chars().take(300).collect::<String>()));
    }
    let trimmed = body.trim();
    if trimmed.starts_with('{') || trimmed.starts_with('[') {
        Err(format!("provider returned unsupported JSON shape: {}", trimmed.chars().take(300).collect::<String>()))
    } else if trimmed.is_empty() {
        Err("provider returned empty response".into())
    } else {
        Ok(trimmed.to_string())
    }
}

fn value_to_text(value: &Value) -> Option<String> {
    match value {
        Value::String(text) => Some(text.clone()),
        Value::Array(items) => {
            let text = items.iter().filter_map(value_to_text).collect::<Vec<_>>().join("");
            (!text.is_empty()).then_some(text)
        }
        Value::Object(object) => object.get("text").and_then(value_to_text).or_else(|| object.get("content").and_then(value_to_text)),
        _ => None,
    }
}

fn strip_fences(raw: &str) -> &str {
    raw.trim()
        .strip_prefix("```json").or_else(|| raw.trim().strip_prefix("```")).unwrap_or(raw.trim())
        .strip_suffix("```").unwrap_or(raw.trim())
        .trim()
}

fn extract_json_block(raw: &str) -> Option<&str> {
    let bytes = raw.as_bytes();
    let start = bytes.iter().position(|b| *b == b'{' || *b == b'[')?;
    let open = bytes[start];
    let close = if open == b'{' { b'}' } else { b']' };
    let mut depth = 0i32;
    let mut in_string = false;
    let mut escaped = false;
    for (idx, byte) in bytes.iter().enumerate().skip(start) {
        let ch = *byte;
        if in_string {
            if escaped {
                escaped = false;
            } else if ch == b'\\' {
                escaped = true;
            } else if ch == b'"' {
                in_string = false;
            }
            continue;
        }
        if ch == b'"' {
            in_string = true;
            continue;
        }
        if ch == open {
            depth += 1;
        } else if ch == close {
            depth -= 1;
            if depth == 0 {
                return raw.get(start..=idx);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{parse_proposal, response_content};
    #[test] fn parses_fenced_json() { assert_eq!(parse_proposal("```json\n{\"type\":\"no_event\"}\n```").unwrap()["type"], "no_event"); }
    #[test] fn extracts_json_from_explanatory_text() { assert_eq!(parse_proposal("好的：{\"type\":\"no_event\"}").unwrap()["type"], "no_event"); }
    #[test] fn reads_chat_completion_content() { let body=r#"{"choices":[{"message":{"content":"{\"type\":\"no_event\"}"}}]}"#; assert!(response_content(body).unwrap().contains("no_event")); }
    #[test] fn reads_text_completion_content() { let body=r#"{"choices":[{"text":"{\"type\":\"no_event\"}"}]}"#; assert!(response_content(body).unwrap().contains("no_event")); }
    #[test] fn rejects_invalid_json() { assert!(parse_proposal("not json").is_err()); }
    #[test] fn repairs_single_quote_json_key_typo() { assert_eq!(parse_proposal(r#"{"effects': {"energy": -2}, "memory": null}"#).unwrap()["effects"]["energy"], -2); }
}
