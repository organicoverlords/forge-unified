use crate::state::AppState;
use axum::{extract::State, Json};

pub async fn recent_events(State(state): State<AppState>) -> Json<serde_json::Value> {
    let events = state.agent.recent_change_events();
    Json(serde_json::json!({
        "event_bus": "change_bus",
        "count": events.len(),
        "events": events,
    }))
}
