use crate::state::AppState;
use axum::{
    extract::State,
    response::{sse::{Event, KeepAlive, Sse}, Html},
    Json,
};
use futures_util::stream::{self, Stream, StreamExt};
use std::{convert::Infallible, time::Duration};

pub async fn recent_events(State(state): State<AppState>) -> Json<serde_json::Value> {
    let events = state.agent.recent_change_events();
    Json(serde_json::json!({"event_bus": "change_bus", "count": events.len(), "events": events}))
}

pub async fn stream_events(State(state): State<AppState>) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let initial = serde_json::json!({"type":"server.connected","properties":{}});
    let connected = stream::once(async move { Ok(Event::default().event("message").data(initial.to_string())) });
    let rx = state.agent.subscribe_change_events();
    let live = stream::unfold(rx, |mut rx| async move {
        loop {
            match rx.recv().await {
                Ok(item) => {
                    let payload = serde_json::json!({"type": item.event_type, "properties": item, "id": item.seq.to_string()});
                    return Some((Ok(Event::default().event("message").id(item.seq.to_string()).data(payload.to_string())), rx));
                }
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                Err(tokio::sync::broadcast::error::RecvError::Closed) => return None,
            }
        }
    });
    Sse::new(connected.chain(live)).keep_alive(KeepAlive::new().interval(Duration::from_secs(10)).text("server.heartbeat"))
}

pub async fn events_page() -> Html<&'static str> { Html(EVENTS_HTML) }

const EVENTS_HTML: &str = r##"<!doctype html>
<html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Forge Activity</title>
<style>
:root{color-scheme:dark;--bg:#070708;--panel:#18181b;--panel2:#202024;--text:#f4f4f5;--muted:#a1a1aa;--border:rgba(255,255,255,.12);--accent:#d6a84f;--ok:#8fd18f;--mono:ui-monospace,SFMono-Regular,Menlo,Consolas,monospace}
*{box-sizing:border-box}body{margin:0;min-height:100vh;background:radial-gradient(circle at 50% -15%,rgba(214,168,79,.18),transparent 34rem),linear-gradient(180deg,#0d0d10,#050506);color:var(--text);font:15px/1.45 Inter,ui-sans-serif,system-ui,sans-serif}.wrap{max-width:1180px;margin:0 auto;padding:1.2rem}.top{display:flex;justify-content:space-between;gap:1rem;align-items:center;margin-bottom:1rem}.pill{border:1px solid var(--border);border-radius:999px;padding:.28rem .7rem;color:var(--accent);background:rgba(214,168,79,.12)}a{color:var(--accent);text-decoration:none}.grid{display:grid;grid-template-columns:minmax(0,1fr) 340px;gap:1rem}.panel{border:1px solid var(--border);background:rgba(24,24,27,.76);border-radius:22px;padding:1rem;box-shadow:0 20px 60px rgba(0,0,0,.25)}h1{font-size:1.2rem;margin:.1rem 0}.sub{color:var(--muted);font-size:.9rem}.events{display:flex;flex-direction:column;gap:.65rem}.event{border:1px solid var(--border);background:rgba(255,255,255,.035);border-radius:16px;padding:.75rem}.event[data-kind*="watcher"]{border-color:rgba(214,168,79,.38)}.event[data-kind*="filesystem"]{border-color:rgba(143,209,143,.38)}.head{display:flex;justify-content:space-between;gap:1rem;font-weight:750}.kind{color:var(--accent)}.file{font-family:var(--mono);margin-top:.35rem;overflow-wrap:anywhere}.meta{color:var(--muted);font-size:.82rem;margin-top:.35rem}pre{white-space:pre-wrap;max-height:13rem;overflow:auto;border:1px solid var(--border);background:#050506;border-radius:14px;padding:.7rem;font:12px var(--mono)}button{border:1px solid var(--border);background:var(--panel2);color:var(--text);border-radius:14px;padding:.65rem .85rem;cursor:pointer}@media(max-width:880px){.grid{grid-template-columns:1fr}}
</style></head><body data-proof="opencode-event-rail">
<div class="wrap"><div class="top"><div><h1>Forge Activity ✦</h1><div class="sub">OpenCode-style EventV2Bridge activity rail backed by the Forge change bus.</div></div><div><a class="pill" href="/">Back to chat</a></div></div>
<div class="grid"><section class="panel"><div class="head"><span>Live event rail</span><span id="status" class="pill">connecting</span></div><div class="sub">Mirrors OpenCode event streaming shape: server.connected, heartbeat, and message events.</div><div id="events" class="events" style="margin-top:1rem"></div></section><aside class="panel"><h1>Sources copied</h1><div class="sub">packages/opencode/src/event-v2-bridge.ts<br>packages/opencode/src/server/routes/instance/httpapi/handlers/event.ts<br>packages/opencode/src/tool/apply_patch.ts</div><button id="refresh" style="margin-top:1rem">Load recent events</button><pre id="raw">[]</pre></aside></div></div>
<script>
const list=document.getElementById('events'),raw=document.getElementById('raw'),status=document.getElementById('status');
function eventFile(e){const p=e.properties?.payload||e.payload||{};return p.file||p.path||p.properties?.payload?.file||'no file'}
function addEvent(e){const row=document.createElement('div');const kind=e.type||e.event_type||'event';row.className='event';row.dataset.kind=kind;row.innerHTML='<div class="head"><span class="kind"></span><span></span></div><div class="file"></div><div class="meta"></div>';row.querySelector('.kind').textContent=kind;row.querySelector('.head span:last-child').textContent=e.id||e.seq||'';row.querySelector('.file').textContent=eventFile(e);row.querySelector('.meta').textContent=JSON.stringify(e.properties?.payload||e.payload||e.properties||{});list.prepend(row);while(list.children.length>50)list.lastChild.remove()}
async function loadRecent(){const r=await fetch('/api/events/recent');const data=await r.json();raw.textContent=JSON.stringify(data,null,2);for(const e of data.events||[])addEvent({type:e.event_type,id:e.seq,properties:e});status.textContent=`recent ${data.count}`}
document.getElementById('refresh').onclick=loadRecent;loadRecent().catch(e=>status.textContent=e.message);const es=new EventSource('/api/events/stream');es.onopen=()=>status.textContent='live';es.onmessage=(m)=>{const data=JSON.parse(m.data);addEvent(data);status.textContent='live'};es.onerror=()=>status.textContent='reconnecting';
</script></body></html>"##;
