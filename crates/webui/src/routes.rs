//! REST API routes.

use axum::{extract::{Path, State}, Json};
use crate::state::AppState;
use forge_engine::types::*;
use serde::{Deserialize, Serialize};

pub async fn health(State(_s): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({"status": "ok", "version": env!("CARGO_PKG_VERSION")}))
}

#[derive(Serialize)]
pub struct ConversationListEntry { id: String, title: String, message_count: usize, mode: String, updated_at: String }

pub async fn list_conversations(State(s): State<AppState>) -> Json<Vec<ConversationListEntry>> {
    let convs = s.agent.list_conversations().await;
    Json(convs.into_iter().map(|c| ConversationListEntry {
        id: c.id.0.to_string(), title: c.title, message_count: c.message_count,
        mode: format!("{:?}", c.mode), updated_at: c.updated_at.to_rfc3339(),
    }).collect())
}

#[derive(Deserialize)]
pub struct CreateConversationRequest { title: String }

#[derive(Serialize)]
pub struct CreateConversationResponse { id: String }

pub async fn create_conversation(State(s): State<AppState>, Json(req): Json<CreateConversationRequest>) -> Json<CreateConversationResponse> {
    let id = s.agent.new_conversation(req.title).await;
    Json(CreateConversationResponse { id: id.0.to_string() })
}

pub async fn get_conversation(State(s): State<AppState>, Path(id): Path<String>) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let conv_id = ConversationId(id.parse().map_err(|_| axum::http::StatusCode::BAD_REQUEST)?);
    let conv = s.agent.get_conversation(&conv_id).await.ok_or(axum::http::StatusCode::NOT_FOUND)?;
    Ok(Json(serde_json::to_value(conv).unwrap_or_default()))
}

pub async fn delete_conversation(State(s): State<AppState>, Path(id): Path<String>) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let conv_id = ConversationId(id.parse().map_err(|_| axum::http::StatusCode::BAD_REQUEST)?);
    s.agent.delete_conversation(&conv_id).await.ok_or(axum::http::StatusCode::NOT_FOUND)?;
    Ok(Json(serde_json::json!({"deleted": true})))
}

#[derive(Deserialize)]
pub struct ChatRequest { message: String, #[allow(dead_code)] max_rounds: Option<u32> }

pub async fn chat(State(s): State<AppState>, Path(id): Path<String>, Json(req): Json<ChatRequest>) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let conv_id = ConversationId(id.parse().map_err(|_| axum::http::StatusCode::BAD_REQUEST)?);
    let record = s.agent.chat(&conv_id, req.message).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(serde_json::to_value(record).unwrap_or_default()))
}

pub async fn cancel(State(s): State<AppState>, Path(id): Path<String>) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let conv_id = ConversationId(id.parse().map_err(|_| axum::http::StatusCode::BAD_REQUEST)?);
    s.agent.cancel(&conv_id).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(serde_json::json!({"cancelled": true})))
}

pub async fn pause(State(s): State<AppState>, Path(id): Path<String>) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let conv_id = ConversationId(id.parse().map_err(|_| axum::http::StatusCode::BAD_REQUEST)?);
    s.agent.pause(&conv_id).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(serde_json::json!({"paused": true})))
}

pub async fn resume(State(s): State<AppState>, Path(id): Path<String>) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let conv_id = ConversationId(id.parse().map_err(|_| axum::http::StatusCode::BAD_REQUEST)?);
    s.agent.resume(&conv_id).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(serde_json::json!({"resumed": true})))
}

pub async fn save_snapshot(State(s): State<AppState>, Path(id): Path<String>) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let conv_id = ConversationId(id.parse().map_err(|_| axum::http::StatusCode::BAD_REQUEST)?);
    let snapshot = s.agent.save_snapshot_with_part(&conv_id).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(serde_json::json!({"snapshot_saved": true, "snapshot": snapshot})))
}

#[derive(Deserialize)]
pub struct BrowserProofApiRequest { pub url: String, pub width: Option<u32>, pub height: Option<u32>, pub capture_dom: Option<bool> }

pub async fn browser_proof(State(s): State<AppState>, Json(req): Json<BrowserProofApiRequest>) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let result = s.agent.browser_proof(&req.url, req.width.unwrap_or(1280), req.height.unwrap_or(720), req.capture_dom.unwrap_or(true)).await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(serde_json::to_value(result).unwrap_or_default()))
}

#[derive(Deserialize)]
pub struct VisionReviewApiRequest { pub image_base64: String, pub prompt: Option<String>, pub provider_id: Option<String>, pub model_id: Option<String> }

pub async fn vision_review(State(s): State<AppState>, Json(req): Json<VisionReviewApiRequest>) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let provider_id = req.provider_id.map(ProviderId);
    let model_id = req.model_id.map(ModelId);
    let result = s.agent.vision_review(&req.image_base64, req.prompt.as_deref(), provider_id, model_id).await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(serde_json::to_value(result).unwrap_or_default()))
}

pub async fn benchmark(State(_s): State<AppState>) -> Json<serde_json::Value> {
    let config = forge_engine::config::Config::default();
    let adapter = forge_engine::benchmark::BenchmarkAdapter::from_config(&config);
    let report: Vec<_> = adapter.report().into_iter().map(|(k, v)| (k.to_string(), v)).collect();
    Json(serde_json::json!({"score": adapter.score(), "capabilities": report}))
}

pub async fn graph_visualization(State(_s): State<AppState>) -> axum::response::Html<String> {
    axum::response::Html(GRAPH_HTML.to_string())
}

pub async fn graph_data(State(s): State<AppState>) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let graph_json = s.agent.clone().graph_build(None).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(graph_json))
}

const GRAPH_HTML: &str = r#"<!DOCTYPE html>
<html><head><meta charset="utf-8"><title>Code Graph Visualization</title><script src="https://cdn.jsdelivr.net/npm/vis-network@9.1.2/dist/vis-network.min.js"></script><style>body{margin:0;font-family:system-ui;background:#1a1a2e;color:#eee}#mynetwork{width:100vw;height:100vh}#controls{position:fixed;top:10px;left:10px;z-index:10;background:rgba(0,0,0,.8);padding:15px;border-radius:8px;border:1px solid #333}#stats{position:fixed;bottom:10px;left:10px;z-index:10;background:rgba(0,0,0,.8);padding:10px;border-radius:8px;font-size:12px}input,select{padding:8px;margin:5px;border:1px solid #444;background:#222;color:#eee;border-radius:4px}button{padding:8px 12px;margin:5px;background:#00bcd4;color:#000;border:0;border-radius:4px;cursor:pointer}.legend{display:flex;flex-wrap:wrap;gap:8px;margin-top:10px;font-size:11px}.legend-item{display:flex;align-items:center;gap:4px}.legend-color{width:12px;height:12px;border-radius:3px}</style></head><body><div id="controls"><div><input type="text" id="search" placeholder="Search nodes..." style="width:200px"><select id="filter-type"><option value="">All Types</option></select><button id="reload">Reload Graph</button></div><div class="legend" id="legend"></div></div><div id="stats">Loading...</div><div id="mynetwork"></div><script>const typeColors={function:'#4caf50',struct:'#ff9800',enum:'#ffeb3b',trait:'#9c27b0',import:'#9e9e9e',constant:'#00bcd4',module:'#009688',type_alias:'#e91e63',unknown:'#666'};let network=null,allNodes=[],allEdges=[];async function loadGraph(){document.getElementById('stats').textContent='Loading graph...';try{const resp=await fetch('/api/graph/data');const data=await resp.json();renderGraph(data)}catch(e){document.getElementById('stats').textContent='Error: '+e.message}}function renderGraph(data){allNodes=data.nodes||[];allEdges=data.edges||[];const types=[...new Set(allNodes.map(n=>n.type).filter(Boolean))];document.getElementById('filter-type').innerHTML='<option value="">All Types</option>'+types.map(t=>`<option value="${t}">${t}</option>`).join('');document.getElementById('legend').innerHTML=types.map(t=>`<span class="legend-item"><span class="legend-color" style="background:${typeColors[t]||'#666'}"></span>${t}</span>`).join('');const nodes=new vis.DataSet(allNodes.map(n=>({id:n.id,label:n.label||n.name||n.id,title:JSON.stringify(n,null,2),color:typeColors[n.type]||'#666',font:{color:'#eee'}})));const edges=new vis.DataSet(allEdges.map((e,i)=>({id:i,from:e.from,to:e.to,label:e.type||'',arrows:'to',color:'#666',font:{color:'#aaa'}})));if(network)network.destroy();network=new vis.Network(document.getElementById('mynetwork'),{nodes,edges},{physics:{stabilization:false},nodes:{shape:'dot',size:16},edges:{smooth:true}});document.getElementById('stats').textContent=`${allNodes.length} nodes, ${allEdges.length} edges`}}document.getElementById('reload').onclick=loadGraph;document.getElementById('search').oninput=e=>{const q=e.target.value.toLowerCase();network.selectNodes(allNodes.filter(n=>(n.label||n.name||n.id||'').toLowerCase().includes(q)).map(n=>n.id))};loadGraph();</script></body></html>"#;
