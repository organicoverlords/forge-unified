use crate::tool::ToolExecutor;
use crate::types::{ToolRequest, ToolResult, ToolKind, BrowserProofResult, BrowserProofRequest, VisionReviewResult, VisionReviewRequest};
use anyhow::Result;
use std::collections::HashMap;
use std::process::Command;

impl ToolExecutor {
    pub async fn execute_browser_proof(&self, request: ToolRequest) -> Result<ToolResult> {
        let args: BrowserProofRequest = serde_json::from_value(request.args.clone())
            .map_err(|e| anyhow::anyhow!("Invalid browser_proof args: {}", e))?;

        let width = args.width.unwrap_or(1280);
        let height = args.height.unwrap_or(720);
        let capture_dom = args.capture_dom.unwrap_or(true);

        let chrome = find_chrome().ok_or_else(|| anyhow::anyhow!(
            "Chrome/Chromium not found. Install Chrome or set CHROME_PATH env var."
        ))?;

        let ts = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
        let out_dir = std::env::temp_dir().join(format!("forge-browser-{}", ts));
        std::fs::create_dir_all(&out_dir)?;

        let screenshot_path = out_dir.join("screenshot.png");

        let screenshot_output = Command::new(&chrome)
            .arg("--headless")
            .arg("--disable-gpu")
            .arg("--no-sandbox")
            .arg(format!("--screenshot={}", screenshot_path.display()))
            .arg(format!("--window-size={},{}", width, height))
            .arg("--virtual-time-budget=15000")
            .arg(&args.url)
            .output();

        match screenshot_output {
            Ok(output) if output.status.success() => {
                let bytes = std::fs::read(&screenshot_path).unwrap_or_default();
                let b64 = base64_encode(&bytes);

                let (dom_snapshot, page_title) = if capture_dom {
                    capture_dom_and_title(&chrome, &args.url)
                } else {
                    (None, String::new())
                };

                let _ = std::fs::remove_dir_all(&out_dir);

                Ok(ToolResult {
                    id: request.id,
                    kind: ToolKind::BrowserProof,
                    success: true,
                    output: serde_json::to_string(&BrowserProofResult {
                        screenshot_base64: b64,
                        console_logs: vec![],
                        dom_snapshot,
                        url: args.url.clone(),
                        page_title,
                        success: true,
                        error: None,
                    })?,
                    error: None,
                    duration_ms: 0,
                    metadata: HashMap::from([
                        ("url".to_string(), serde_json::json!(args.url)),
                        ("width".to_string(), serde_json::json!(width)),
                        ("height".to_string(), serde_json::json!(height)),
                    ]),
                })
            }
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                let _ = std::fs::remove_dir_all(&out_dir);
                Ok(ToolResult {
                    id: request.id,
                    kind: ToolKind::BrowserProof,
                    success: false,
                    output: String::new(),
                    error: Some(format!("Chrome error: {}", stderr)),
                    duration_ms: 0,
                    metadata: HashMap::new(),
                })
            }
            Err(e) => {
                let _ = std::fs::remove_dir_all(&out_dir);
                Ok(ToolResult {
                    id: request.id,
                    kind: ToolKind::BrowserProof,
                    success: false,
                    output: String::new(),
                    error: Some(format!("Failed to launch Chrome: {}", e)),
                    duration_ms: 0,
                    metadata: HashMap::new(),
                })
            }
        }
    }

    pub async fn execute_vision_review(&self, request: ToolRequest) -> Result<ToolResult> {
        let args: VisionReviewRequest = serde_json::from_value(request.args.clone())
            .map_err(|e| anyhow::anyhow!("Invalid vision_review args: {}", e))?;

        let result = analyze_with_vision(&args).await?;

        Ok(ToolResult {
            id: request.id,
            kind: ToolKind::VisionReview,
            success: result.success,
            output: serde_json::to_string(&result)?,
            error: result.error.clone(),
            duration_ms: 0,
            metadata: HashMap::from([
                ("provider".to_string(), serde_json::json!(result.provider.0)),
                ("model".to_string(), serde_json::json!(result.model.0)),
            ]),
        })
    }
}

async fn analyze_with_vision(args: &VisionReviewRequest) -> Result<VisionReviewResult> {
    let provider_id = args.provider_id.clone().unwrap_or(crate::types::ProviderId("nvidia_nim".to_string()));
    let model_id = args.model_id.clone().unwrap_or(crate::types::ModelId("meta/llama-3.2-11b-vision-instruct".to_string()));
    let prompt = args.prompt.clone().unwrap_or_else(||
        "Describe what you see in this screenshot. Identify any UI issues, errors, or unexpected elements.".to_string()
    );

    let api_key = std::env::var("NVIDIA_NIM_API_KEY")
        .map_err(|_| anyhow::anyhow!("NVIDIA_NIM_API_KEY not set for vision"))?;

    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "model": model_id.0,
        "messages": [{
            "role": "user",
            "content": [
                { "type": "text", "text": prompt },
                { "type": "image_url", "image_url": { "url": format!("data:image/png;base64,{}", args.image_base64) } }
            ]
        }],
        "max_tokens": 1024,
        "temperature": 0.2,
    });

    let resp = client
        .post("https://integrate.api.nvidia.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(60))
        .json(&body)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Vision request failed: {}", e))?;

    let status = resp.status();
    let text = resp.text().await.unwrap_or_default();

    if !status.is_success() {
        return Ok(VisionReviewResult {
            analysis: String::new(),
            verdict: None,
            provider: provider_id,
            model: model_id,
            success: false,
            error: Some(format!("Vision API {}: {}", status.as_u16(), text)),
        });
    }

    let parsed: serde_json::Value = serde_json::from_str(&text)
        .map_err(|e| anyhow::anyhow!("Failed to parse vision response: {}", e))?;

    let content = parsed["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("No analysis returned")
        .to_string();

    Ok(VisionReviewResult {
        analysis: content.clone(),
        verdict: extract_verdict(&content),
        provider: provider_id,
        model: model_id,
        success: true,
        error: None,
    })
}

fn extract_verdict(analysis: &str) -> Option<String> {
    let lower = analysis.to_lowercase();
    if lower.contains("no issue") || lower.contains("looks correct") || lower.contains("no error") || lower.contains("everything looks") {
        Some("pass".to_string())
    } else if lower.contains("error") || lower.contains("issue") || lower.contains("problem") || lower.contains("bug") || lower.contains("missing") || lower.contains("broken") || lower.contains("unexpected") {
        Some("fail".to_string())
    } else {
        None
    }
}

fn base64_encode(bytes: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.encode(bytes)
}

fn capture_dom_and_title(chrome: &str, url: &str) -> (Option<String>, String) {
    let dom_output = Command::new(chrome)
        .arg("--headless")
        .arg("--disable-gpu")
        .arg("--no-sandbox")
        .arg("--dump-dom")
        .arg("--virtual-time-budget=10000")
        .arg(url)
        .output();

    match dom_output {
        Ok(dom) if dom.status.success() => {
            let raw = String::from_utf8_lossy(&dom.stdout).to_string();
            let title = extract_title_from_dom(&raw);
            (Some(raw), title)
        }
        _ => (None, String::new()),
    }
}

fn extract_title_from_dom(dom: &str) -> String {
    if let Some(start) = dom.find("<title>") {
        let after = &dom[start + 7..];
        if let Some(end) = after.find("</title>") {
            return after[..end].to_string();
        }
    }
    String::new()
}

fn find_chrome() -> Option<String> {
    if let Ok(path) = std::env::var("CHROME_PATH") {
        if std::path::Path::new(&path).exists() {
            return Some(path);
        }
    }

    let candidates: Vec<&str> = if cfg!(target_os = "windows") {
        vec![
            r"C:\Program Files\Google\Chrome\Application\chrome.exe",
            r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
            r"C:\Program Files\Chromium\Application\chrome.exe",
        ]
    } else if cfg!(target_os = "macos") {
        vec![
            "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
            "/Applications/Chromium.app/Contents/MacOS/Chromium",
        ]
    } else {
        vec![]
    };

    for c in &candidates {
        if std::path::Path::new(c).exists() {
            return Some(c.to_string());
        }
    }

    if cfg!(target_os = "linux") {
        for name in &["google-chrome", "google-chrome-stable", "chromium", "chromium-browser"] {
            if let Ok(output) = Command::new("which").arg(name).output() {
                if output.status.success() {
                    let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !path.is_empty() {
                        return Some(path);
                    }
                }
            }
        }
    }

    None
}
