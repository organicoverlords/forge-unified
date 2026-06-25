//! Web operation tools.

use crate::tool::ToolExecutor;
use crate::types::{ToolRequest, ToolResult, ToolCallId, ToolKind};
use anyhow::{Context, Result};
use std::collections::HashMap;
use reqwest::Client;

impl ToolExecutor {
    pub async fn execute_web_fetch(&self, request: ToolRequest) -> Result<ToolResult> {
        #[derive(serde::Deserialize)]
        struct Args { url: String, timeout_ms: Option<u64> }
        let args: Args = serde_json::from_value(request.args)?;
        
        let client = Client::builder()
            .timeout(std::time::Duration::from_millis(args.timeout_ms.unwrap_or(30000)))
            .build()?;
        
        let response = client.get(&args.url).send().await
            .with_context(|| format!("Failed to fetch: {}", args.url))?;
        
        let status = response.status().as_u16();
        let content_type = response.headers().get("content-type")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());
        let text = response.text().await?;
        
        Ok(ToolResult {
            id: request.id,
            kind: ToolKind::WebFetch,
            success: status < 400,
            output: text,
            error: if status >= 400 { Some(format!("HTTP {}", status)) } else { None },
            duration_ms: 0,
            metadata: HashMap::from([
                ("url".to_string(), serde_json::json!(args.url)),
                ("status".to_string(), serde_json::json!(status)),
                ("content_type".to_string(), serde_json::json!(content_type)),
            ]),
        })
    }

    pub async fn execute_web_search(&self, request: ToolRequest) -> Result<ToolResult> {
        #[derive(serde::Deserialize)]
        struct Args { query: String, max_results: Option<usize> }
        let args: Args = serde_json::from_value(request.args)?;
        let max_results = args.max_results.unwrap_or(10);
        
        // Using DuckDuckGo HTML scraping as a simple search
        let url = format!("https://html.duckduckgo.com/html/?q={}", urlencoding::encode(&args.query));
        let client = Client::new();
        let response = client.get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send().await?;
        let html = response.text().await?;
        
        // Simple extraction of result snippets
        let mut results = Vec::new();
        for cap in regex::Regex::new(r#"<a[^>]*class="result__snippet"[^>]*>([^<]*)</a>"#)?.captures_iter(&html) {
            if let Some(m) = cap.get(1) {
                results.push(m.as_str().to_string());
                if results.len() >= max_results { break; }
            }
        }
        
        Ok(ToolResult {
            id: request.id,
            kind: ToolKind::WebSearch,
            success: true,
            output: serde_json::to_string_pretty(&results)?,
            error: None,
            duration_ms: 0,
            metadata: HashMap::from([
                ("query".to_string(), serde_json::json!(args.query)),
                ("results".to_string(), serde_json::json!(results.len())),
            ]),
        })
    }
}