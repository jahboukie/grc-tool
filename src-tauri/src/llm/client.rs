use sqlx::PgPool;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::llm::{self, LlmContext};

pub async fn build_context(
    pool: &PgPool,
    engagement_id: Option<Uuid>,
    ai_system_id: Option<Uuid>,
) -> Result<Option<LlmContext>, String> {
    llm::build_context_from_db(pool, engagement_id, ai_system_id).await
}

pub fn build_system_prompt(context: &Option<LlmContext>) -> String {
    let base = "You are an AI Governance regulatory assistant for a Certified AI Governance Professional. \
                Provide responses that: 1) Reference specific articles, clauses, and sections, \
                2) Give actionable, practitioner-level guidance, 3) Flag cross-framework implications, \
                4) Suggest specific evidence that would demonstrate compliance, \
                5) Be precise — cite the regulation, not just general advice.";

    let Some(ctx) = context else { return base.to_string() };

    let pct = if ctx.total_count > 0 {
        (ctx.met_count as f64 / ctx.total_count as f64 * 100.0) as i64
    } else { 0 };

    let mut prompt = format!("{}\n\nContext:\n- Engagement: {} (Role: {})", base, ctx.engagement_name, ctx.primary_role);

    if let Some(ref name) = ctx.system_name {
        prompt.push_str(&format!("\n- AI System: {}", name));
    }
    if let Some(ref rc) = ctx.risk_category {
        prompt.push_str(&format!(" (Risk: {})", rc));
    }
    if let Some(ref domain) = ctx.domain {
        prompt.push_str(&format!(", Domain: {}", domain));
    }
    if !ctx.frameworks.is_empty() {
        prompt.push_str(&format!("\n- Frameworks in scope: {}", ctx.frameworks.join(", ")));
    }
    prompt.push_str(&format!("\n- Compliance: {}/{} requirements met ({}%)", ctx.met_count, ctx.total_count, pct));

    if !ctx.gap_titles.is_empty() {
        prompt.push_str("\n- Current gaps:");
        for g in &ctx.gap_titles {
            prompt.push_str(&format!("\n  • {}", g));
        }
    }

    prompt
}

pub async fn call_llm(
    provider: &str,
    api_key: &str,
    model: &str,
    system_prompt: &str,
    user_query: &str,
) -> Result<String, String> {
    let client = reqwest::Client::new();

    match provider {
        "openai" => call_openai(&client, api_key, model, system_prompt, user_query).await,
        "anthropic" => call_anthropic(&client, api_key, model, system_prompt, user_query).await,
        "ollama" => call_ollama(&client, model, system_prompt, user_query).await,
        _ => Err(format!("Unknown LLM provider: {}", provider)),
    }
}

#[derive(Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<OpenAiMessage>,
}

#[derive(Serialize, Deserialize)]
struct OpenAiMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAiResponse {
    choices: Vec<OpenAiChoice>,
}

#[derive(Deserialize)]
struct OpenAiChoice {
    message: OpenAiMessage,
}

async fn call_openai(
    client: &reqwest::Client,
    api_key: &str,
    model: &str,
    system_prompt: &str,
    user_query: &str,
) -> Result<String, String> {
    let request = OpenAiRequest {
        model: model.to_string(),
        messages: vec![
            OpenAiMessage { role: "system".to_string(), content: system_prompt.to_string() },
            OpenAiMessage { role: "user".to_string(), content: user_query.to_string() },
        ],
    };

    let resp = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&request)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let body: OpenAiResponse = resp.json().await.map_err(|e| e.to_string())?;
    body.choices.first()
        .map(|c| c.message.content.clone())
        .ok_or_else(|| "No response from OpenAI".to_string())
}

#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    system: String,
    messages: Vec<AnthropicMessage>,
}

#[derive(Serialize, Deserialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicContent>,
}

#[derive(Deserialize)]
struct AnthropicContent {
    text: String,
}

async fn call_anthropic(
    client: &reqwest::Client,
    api_key: &str,
    model: &str,
    system_prompt: &str,
    user_query: &str,
) -> Result<String, String> {
    let request = AnthropicRequest {
        model: model.to_string(),
        max_tokens: 4096,
        system: system_prompt.to_string(),
        messages: vec![
            AnthropicMessage { role: "user".to_string(), content: user_query.to_string() },
        ],
    };

    let resp = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&request)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let body: AnthropicResponse = resp.json().await.map_err(|e| e.to_string())?;
    body.content.first()
        .map(|c| c.text.clone())
        .ok_or_else(|| "No response from Anthropic".to_string())
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    system: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

async fn call_ollama(
    client: &reqwest::Client,
    model: &str,
    system_prompt: &str,
    user_query: &str,
) -> Result<String, String> {
    let request = OllamaRequest {
        model: model.to_string(),
        prompt: user_query.to_string(),
        system: system_prompt.to_string(),
        stream: false,
    };

    let resp = client
        .post("http://localhost:11434/api/generate")
        .json(&request)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let body: OllamaResponse = resp.json().await.map_err(|e| e.to_string())?;
    Ok(body.response)
}
