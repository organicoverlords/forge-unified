//! OpenCode-style post-edit event metadata for apply_patch.
//!
//! Upstream references:
//! - `packages/opencode/src/tool/apply_patch.ts` publishes
//!   `FileSystem.Event.Edited` for edited targets.
//! - `packages/opencode/src/tool/apply_patch.ts` publishes
//!   `Watcher.Event.Updated` with `add`, `change`, and `unlink` events.
//! - `packages/opencode/src/tool/apply_patch.ts` touches LSP documents and then
//!   collects diagnostics.
//! - `packages/opencode/src/server/routes/instance/httpapi/handlers/event.ts` streams
//!   event payloads to connected clients through the EventV2 bridge.

fn file_path(file: &serde_json::Value) -> &str {
    file.get("path").and_then(serde_json::Value::as_str).unwrap_or("")
}

fn target_path(file: &serde_json::Value) -> &str {
    file.get("relativePath").and_then(serde_json::Value::as_str).or_else(|| file.get("path").and_then(serde_json::Value::as_str)).unwrap_or("")
}

fn change_type(file: &serde_json::Value) -> &str {
    file.get("type").and_then(serde_json::Value::as_str).unwrap_or("update")
}

pub(crate) fn file_change_events(files: &[serde_json::Value]) -> Vec<serde_json::Value> {
    files.iter().map(|file| {
        let kind = change_type(file);
        let path = target_path(file);
        serde_json::json!({
            "type": match kind { "add" => "file.added", "delete" => "file.deleted", "move" => "file.moved", _ => "file.edited" },
            "path": path,
            "previousPath": file.get("path").filter(|_| kind == "move"),
            "source": "apply_patch",
            "additions": file.get("additions").cloned().unwrap_or_else(|| serde_json::json!(0)),
            "deletions": file.get("deletions").cloned().unwrap_or_else(|| serde_json::json!(0)),
            "bom": file.get("bom").cloned().unwrap_or_else(|| serde_json::json!(false)),
            "opencode_watcher_update": watcher_updates_for_file(file),
            "opencode_filesystem_edited": filesystem_edits_for_file(file),
            "opencode_lsp_diagnostics": diagnostic_reports_for_file(file),
        })
    }).collect()
}

pub(crate) fn watcher_updates(files: &[serde_json::Value]) -> Vec<serde_json::Value> {
    files.iter().flat_map(watcher_updates_for_file).collect()
}

fn watcher_updates_for_file(file: &serde_json::Value) -> Vec<serde_json::Value> {
    let kind = change_type(file);
    let path = file_path(file);
    let target = target_path(file);
    let mut updates = match kind {
        "add" => vec![watcher_update(target, "add")],
        "delete" => vec![watcher_update(path, "unlink")],
        "move" => vec![watcher_update(path, "unlink"), watcher_update(target, "add")],
        _ => vec![watcher_update(target, "change")],
    };
    updates.retain(|value| value.get("file").and_then(serde_json::Value::as_str).is_some_and(|file| !file.is_empty()));
    updates
}

fn watcher_update(file: &str, event: &str) -> serde_json::Value {
    serde_json::json!({
        "event_name": "Watcher.Event.Updated",
        "file": file,
        "event": event,
        "source": "packages/opencode/src/tool/apply_patch.ts:events.publish(Watcher.Event.Updated, update)",
    })
}

pub(crate) fn filesystem_edits(files: &[serde_json::Value]) -> Vec<serde_json::Value> {
    files.iter().flat_map(filesystem_edits_for_file).collect()
}

fn filesystem_edits_for_file(file: &serde_json::Value) -> Vec<serde_json::Value> {
    if change_type(file) == "delete" { return Vec::new(); }
    let target = target_path(file);
    if target.is_empty() { return Vec::new(); }
    vec![serde_json::json!({
        "event_name": "FileSystem.Event.Edited",
        "file": target,
        "source": "packages/opencode/src/tool/apply_patch.ts:events.publish(FileSystem.Event.Edited, { file: edited })",
    })]
}

pub(crate) fn lsp_touches(files: &[serde_json::Value]) -> Vec<serde_json::Value> {
    files.iter().filter(|file| change_type(file) != "delete").filter_map(|file| {
        let target = target_path(file);
        (!target.is_empty()).then(|| serde_json::json!({
            "file": target,
            "kind": "document",
            "source": "packages/opencode/src/tool/apply_patch.ts:lsp.touchFile(target, document)",
        }))
    }).collect()
}

pub(crate) fn diagnostic_reports(files: &[serde_json::Value]) -> Vec<serde_json::Value> {
    files.iter().flat_map(diagnostic_reports_for_file).collect()
}

fn diagnostic_reports_for_file(file: &serde_json::Value) -> Vec<serde_json::Value> {
    if change_type(file) == "delete" { return Vec::new(); }
    let target = target_path(file);
    if target.is_empty() { return Vec::new(); }
    vec![serde_json::json!({
        "event_name": "LSP.Diagnostic.report",
        "file": target,
        "status": "pending_service",
        "diagnostics": [],
        "source": "packages/opencode/src/tool/apply_patch.ts:lsp.diagnostics() -> LSP.Diagnostic.report",
        "note": "Forge now emits the live diagnostics event envelope after each approved edit; attaching a real LSP backend remains a separate parity step.",
    })]
}

pub(crate) fn diagnostics_metadata(files: &[serde_json::Value]) -> serde_json::Value {
    let reports = diagnostic_reports(files);
    serde_json::json!({
        "status": "event_envelope_emitted",
        "reason": "Forge records OpenCode LSP touch targets and emits LSP diagnostic report envelopes; live language-server collection is not wired yet.",
        "touched_files": lsp_touches(files),
        "reports": reports,
        "report_count": reports.len(),
        "opencode_source": "packages/opencode/src/tool/apply_patch.ts:lsp.touchFile + lsp.diagnostics + LSP.Diagnostic.report",
    })
}

pub(crate) fn opencode_event_source() -> serde_json::Value {
    serde_json::json!({
        "paths": [
            "packages/opencode/src/tool/apply_patch.ts",
            "packages/opencode/src/event-v2-bridge.ts",
            "packages/opencode/src/server/routes/instance/httpapi/handlers/event.ts"
        ],
        "behaviors": [
            "events.publish(FileSystem.Event.Edited, { file: edited })",
            "events.publish(Watcher.Event.Updated, update)",
            "lsp.touchFile(target, document)",
            "lsp.diagnostics()",
            "EventV2Bridge forwards activity payloads to the HTTP event stream"
        ]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_add_update_delete_and_move_to_watcher_updates() {
        let files = vec![
            serde_json::json!({"type": "add", "path": "a.txt", "relativePath": "a.txt"}),
            serde_json::json!({"type": "update", "path": "b.txt", "relativePath": "b.txt"}),
            serde_json::json!({"type": "delete", "path": "c.txt", "relativePath": "c.txt"}),
            serde_json::json!({"type": "move", "path": "old.txt", "relativePath": "new.txt"}),
        ];
        let updates = watcher_updates(&files);
        let events: Vec<_> = updates.iter().map(|v| v["event"].as_str().unwrap()).collect();
        assert_eq!(events, vec!["add", "change", "unlink", "unlink", "add"]);
        assert_eq!(updates[3]["file"], "old.txt");
        assert_eq!(updates[4]["file"], "new.txt");
    }

    #[test]
    fn filesystem_edit_events_skip_deletes() {
        let files = vec![
            serde_json::json!({"type": "delete", "path": "gone.txt", "relativePath": "gone.txt"}),
            serde_json::json!({"type": "move", "path": "old.txt", "relativePath": "new.txt"}),
        ];
        let edits = filesystem_edits(&files);
        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0]["event_name"], "FileSystem.Event.Edited");
        assert_eq!(edits[0]["file"], "new.txt");
    }

    #[test]
    fn diagnostics_metadata_records_lsp_touch_targets() {
        let files = vec![serde_json::json!({"type": "add", "path": "a.rs", "relativePath": "a.rs"})];
        let meta = diagnostics_metadata(&files);
        assert_eq!(meta["status"], "event_envelope_emitted");
        assert_eq!(meta["touched_files"][0]["kind"], "document");
        assert_eq!(meta["reports"][0]["event_name"], "LSP.Diagnostic.report");
        assert!(meta["opencode_source"].as_str().unwrap().contains("lsp.diagnostics"));
    }

    #[test]
    fn diagnostic_reports_skip_deleted_files() {
        let files = vec![serde_json::json!({"type": "delete", "path": "gone.rs", "relativePath": "gone.rs"})];
        assert!(diagnostic_reports(&files).is_empty());
    }
}
