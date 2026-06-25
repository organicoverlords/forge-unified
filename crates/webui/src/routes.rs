//! REST API routes.

use axum::{extract::{Path, State}, Json};
use crate::state::AppState;
use forge_engine::types::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub async fn health(State(_s): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

#[derive(Serialize)]
pub struct ConversationListEntry {
    id: String,
    title: String,
    message_count: usize,
    mode: String,
    updated_at: String,
}

pub async fn list_conversations(State(s): State<AppState>) -> Json<Vec<ConversationListEntry>> {
    let convs = s.agent.list_conversations().await;
    Json(convs.into_iter().map(|c| ConversationListEntry {
        id: c.id.0.to_string(),
        title: c.title,
        message_count: c.message_count,
        mode: format!("{:?}", c.mode),
        updated_at: c.updated_at.to_rfc3339(),
    }).collect())
}

#[derive(Deserialize)]
pub struct CreateConversationRequest {
    title: String,
}

#[derive(Serialize)]
pub struct CreateConversationResponse {
    id: String,
}

pub async fn create_conversation(
    State(s): State<AppState>,
    Json(req): Json<CreateConversationRequest>,
) -> Json<CreateConversationResponse> {
    let id = s.agent.new_conversation(req.title).await;
    Json(CreateConversationResponse { id: id.0.to_string() })
}

pub async fn get_conversation(
    State(s): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let conv_id = ConversationId(id.parse().map_err(|_| axum::http::StatusCode::BAD_REQUEST)?);
    let conv = s.agent.get_conversation(&conv_id).await
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;
    Ok(Json(serde_json::to_value(conv).unwrap_or_default()))
}

pub async fn delete_conversation(
    State(s): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let conv_id = ConversationId(id.parse().map_err(|_| axum::http::StatusCode::BAD_REQUEST)?);
    s.agent.delete_conversation(&conv_id).await
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;
    Ok(Json(serde_json::json!({"deleted": true})))
}

#[derive(Deserialize)]
pub struct ChatRequest {
    message: String,
    #[allow(dead_code)]
    max_rounds: Option<u32>,
}

pub async fn chat(
    State(s): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<ChatRequest>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let conv_id = ConversationId(id.parse().map_err(|_| axum::http::StatusCode::BAD_REQUEST)?);
    let record = s.agent.chat(&conv_id, req.message).await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(serde_json::to_value(record).unwrap_or_default()))
}

pub async fn cancel(
    State(s): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let conv_id = ConversationId(id.parse().map_err(|_| axum::http::StatusCode::BAD_REQUEST)?);
    s.agent.cancel(&conv_id).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(serde_json::json!({"cancelled": true})))
}

pub async fn pause(
    State(s): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let conv_id = ConversationId(id.parse().map_err(|_| axum::http::StatusCode::BAD_REQUEST)?);
    s.agent.pause(&conv_id).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(serde_json::json!({"paused": true})))
}

pub async fn resume(
    State(s): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let conv_id = ConversationId(id.parse().map_err(|_| axum::http::StatusCode::BAD_REQUEST)?);
    s.agent.resume(&conv_id).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(serde_json::json!({"resumed": true})))
}

pub async fn save_snapshot(
    State(s): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let conv_id = ConversationId(id.parse().map_err(|_| axum::http::StatusCode::BAD_REQUEST)?);
    s.agent.save_snapshot(&conv_id).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(serde_json::json!({"snapshot_saved": true})))
}

#[derive(Deserialize)]
pub struct BrowserProofApiRequest {
    pub url: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub capture_dom: Option<bool>,
}

pub async fn browser_proof(
    State(s): State<AppState>,
    Json(req): Json<BrowserProofApiRequest>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let result = s.agent.browser_proof(
        &req.url,
        req.width.unwrap_or(1280),
        req.height.unwrap_or(720),
        req.capture_dom.unwrap_or(true),
    ).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(serde_json::to_value(result).unwrap_or_default()))
}

#[derive(Deserialize)]
pub struct VisionReviewApiRequest {
    pub image_base64: String,
    pub prompt: Option<String>,
    pub provider_id: Option<String>,
    pub model_id: Option<String>,
}

pub async fn vision_review(
    State(s): State<AppState>,
    Json(req): Json<VisionReviewApiRequest>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let provider_id = req.provider_id.map(ProviderId);
    let model_id = req.model_id.map(ModelId);
    let result = s.agent.vision_review(
        &req.image_base64,
        req.prompt.as_deref(),
        provider_id,
        model_id,
    ).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(serde_json::to_value(result).unwrap_or_default()))
}

pub async fn benchmark(State(_s): State<AppState>) -> Json<serde_json::Value> {
    let config = forge_engine::config::Config::default();
    let adapter = forge_engine::benchmark::BenchmarkAdapter::from_config(&config);
    let report: Vec<_> = adapter.report().into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect();
    Json(serde_json::json!({
        "score": adapter.score(),
        "capabilities": report,
    }))
}

pub async fn graph_visualization(State(_s): State<AppState>) -> axum::response::Html<String> {
    axum::response::Html(GRAPH_HTML.to_string())
}

pub async fn graph_data(State(s): State<AppState>) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let agent = s.agent.clone();
    let graph_json = agent.graph_build(None).await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(graph_json))
}

const GRAPH_HTML: &str = r#"<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <title>Code Graph Visualization</title>
  <script src="https://cdn.jsdelivr.net/npm/vis-network@9.1.2/dist/vis-network.min.js"></script>
  <style>
    body { margin: 0; font-family: system-ui; background: #1a1a2e; color: #eee; }
    #mynetwork { width: 100vw; height: 100vh; }
    #controls { position: fixed; top: 10px; left: 10px; z-index: 10; background: rgba(0,0,0,0.8); padding: 15px; border-radius: 8px; border: 1px solid #333; }
    #stats { position: fixed; bottom: 10px; left: 10px; z-index: 10; background: rgba(0,0,0,0.8); padding: 10px; border-radius: 8px; font-size: 12px; }
    input, select { padding: 8px; margin: 5px; border: 1px solid #444; background: #222; color: #eee; border-radius: 4px; }
    button { padding: 8px 12px; margin: 5px; background: #00bcd4; color: #000; border: none; border-radius: 4px; cursor: pointer; }
    button:hover { background: #0097a7; }
    .legend { display: flex; flex-wrap: wrap; gap: 8px; margin-top: 10px; font-size: 11px; }
    .legend-item { display: flex; align-items: center; gap: 4px; }
    .legend-color { width: 12px; height: 12px; border-radius: 3px; }
  </style>
</head>
<body>
  <div id="controls">
    <div>
      <input type="text" id="search" placeholder="Search nodes..." style="width: 200px;">
      <select id="filter-type"><option value="">All Types</option></select>
      <button id="reload">Reload Graph</button>
    </div>
    <div class="legend" id="legend"></div>
  </div>
  <div id="stats">Loading...</div>
  <div id="mynetwork"></div>
  <script>
    const typeColors = {
      'file': '#444', 'rust': '#dea584', 'javascript': '#f7df1e', 'typescript': '#3178c6',
      'python': '#3776ab', 'go': '#00add8', 'java': '#b07219', 'cpp': '#f34b7d', 'c': '#555', 'unknown': '#666'
    };
    let network = null;
    let allNodes = [];
    let allEdges = [];

    async function loadGraph() {
      document.getElementById('stats').textContent = 'Loading graph...';
      try {
        const resp = await fetch('/api/graph/data');
        const data = await resp.json();
        renderGraph(data);
      } catch (e) {
        document.getElementById('stats').textContent = 'Error: ' + e.message;
      }
    }

    function renderGraph(data) {
      allNodes = data.nodes || [];
      allEdges = data.edges || [];
      
      const types = [...new Set(allNodes.map(n => n.type).filter(Boolean))];
      const filterSelect = document.getElementById('filter-type');
      filterSelect.innerHTML = '<option value="">All Types</option>' + 
        types.map(t => `<option value="${t}">${t}</option>`).join('');

      const legend = document.getElementById('legend');
      legend.innerHTML = types.map(t => 
        `<span class="legend-item"><span class="legend-color" style="background:${typeColors[t] || '#666'}"></span>${t}</span>`
      ).join('');

      const nodes = new vis.DataSet(allNodes.map(n => ({
        id: n.id, label: n.label, title: n.label + '\n' + (n.imports || []).join('\n'),
        group: n.type, color: typeColors[n.type] || '#666', font: { color: '#eee' }
      })));
      const edges = new vis.DataSet(allEdges.map(e => ({
        from: e.source, to: e.target, label: e.type, arrows: 'to', color: '#555', font: { color: '#aaa', size: 10 }
      })));

      const container = document.getElementById('mynetwork');
      if (network) network.destroy();
      network = new vis.Network(container, { nodes, edges }, {
        physics: { barnesHut: { gravitationalConstant: -8000, springLength: 150 } },
        interaction: { hover: true, navigationButtons: true },
        nodes: { shape: 'dot', size: 8 },
        edges: { smooth: { type: 'continuous' } }
      });

      document.getElementById('search').addEventListener('input', e => {
        const q = e.target.value.toLowerCase();
        nodes.forEach(n => { n.hidden = !n.label.toLowerCase().includes(q); nodes.update(n); });
      });

      document.getElementById('filter-type').addEventListener('change', e => {
        const t = e.target.value;
        nodes.forEach(n => { n.hidden = t && n.group !== t; nodes.update(n); });
      });

      document.getElementById('reload').addEventListener('click', loadGraph);

      const stats = data.stats || {};
      document.getElementById('stats').textContent = 
        `Files: ${stats.files || allNodes.length} | Nodes: ${stats.nodes || allNodes.length} | Edges: ${stats.edges || allEdges.length}`;
    }

    loadGraph();
  </script>
</body>
</html>"#;
