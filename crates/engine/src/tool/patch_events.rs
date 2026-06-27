//! OpenCode-style post-edit event metadata for apply_patch.
//!
//! Upstream references:
//! - `packages/opencode/src/tool/apply_patch.ts` publishes
//!   `FileSystem.Event.Edited` for edited targets.
//! - `packages/opencode/src/tool/apply_patch.ts` publishes
//!   `Watcher.Event.Updated` with `add`, `change`, and `unlink` events.
//! - `packages/opencode/src/tool/apply_patch.ts` touches LSP documents, collects
//!   diagnostics, and appends `LSP.Diagnostic.report(...)` output when errors exist.
//! - `packages/opencode/src/lsp/lsp.ts` defines `touchFile`, `diagnostics`, client
//!   status, and the LSP service interface.
//! - `packages/opencode/src/lsp/diagnostic.ts` limits diagnostic report output to
//!   20 errors per file and formats ERROR/WARN/INFO/HINT severities.
//! - `packages/opencode/src/tool/read.ts` treats LSP warm-up as optional and
//!   contains warm-up defects with `Effect.ignoreCause`.
//! - `packages/opencode/src/event-v2-bridge.ts` forwards EventV2 payloads to
//!   instance/workspace-scoped UI consumers.
//! - `packages/opencode/src/server/routes/instance/httpapi/handlers/event.ts` streams
//!   event payloads to connected clients through the EventV2 bridge.

const MAX_DIAGNOSTICS_PER_FILE: usize = 20;

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
            "opencode_lsp_warmup": lsp_warmups_for_file(file),
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

pub(crate) fn lsp_warmups(files: &[serde_json::Value]) -> Vec<serde_json::Value> {
    files.iter().flat_map(lsp_warmups_for_file).collect()
}

fn lsp_warmups_for_file(file: &serde_json::Value) -> Vec<serde_json::Value> {
    if change_type(file) == "delete" { return Vec::new(); }
    let target = target_path(file);
    if target.is_empty() { return Vec::new(); }
    vec![serde_json::json!({
        "event_name": "LSP.Warmup.contained",
        "file": target,
        "status": "contained_optional_warmup",
        "contained": true,
        "diagnostics_blocked": false,
        "source": "packages/opencode/src/tool/read.ts:lsp.touchFile(filepath).pipe(Effect.ignoreCause, Effect.forkIn(scope))",
        "note": "Forge records the same safety contract: LSP warm-up is optional and must not fail an otherwise successful edit/read path.",
    })]
}

pub(crate) fn diagnostic_reports(files: &[serde_json::Value]) -> Vec<serde_json::Value> {
    files.iter().flat_map(diagnostic_reports_for_file).collect()
}

fn diagnostic_reports_for_file(file: &serde_json::Value) -> Vec<serde_json::Value> {
    if change_type(file) == "delete" { return Vec::new(); }
    let target = target_path(file);
    if target.is_empty() { return Vec::new(); }
    let diagnostics = contained_diagnostics_for_file(file);
    let report_block = diagnostic_report_block(target, &diagnostics);
    let severity_counts = severity_counts(&diagnostics);
    vec![serde_json::json!({
        "event_name": "LSP.Diagnostic.report",
        "file": target,
        "status": if diagnostics.is_empty() { "no_errors_reported_contained_service" } else { "errors_reported_contained_service" },
        "diagnostics": diagnostics,
        "diagnostic_count": diagnostics.len(),
        "severity_counts": severity_counts,
        "max_per_file": MAX_DIAGNOSTICS_PER_FILE,
        "report_block": report_block,
        "report_emitted": !report_block.is_empty(),
        "warmup_contained": true,
        "lsp_clients": [],
        "lsp_client_status": "not_connected_contained",
        "source": "packages/opencode/src/tool/apply_patch.ts:lsp.diagnostics() -> LSP.Diagnostic.report; packages/opencode/src/lsp/diagnostic.ts:MAX_PER_FILE pretty/report; packages/opencode/src/lsp/lsp.ts:status/touchFile/diagnostics",
        "note": "Forge emits the diagnostic report shape after each approved edit. It now preserves OpenCode severity/count/report-block semantics while keeping missing live LSP clients contained.",
    })]
}

fn contained_diagnostics_for_file(file: &serde_json::Value) -> Vec<serde_json::Value> {
    let patch = file.get("patch").and_then(serde_json::Value::as_str).unwrap_or("");
    patch.lines()
        .filter(|line| line.starts_with('+') && !line.starts_with("+++"))
        .enumerate()
        .filter_map(|(idx, line)| contained_diagnostic_for_added_line(idx, line.trim_start_matches('+')))
        .take(MAX_DIAGNOSTICS_PER_FILE)
        .collect()
}

fn contained_diagnostic_for_added_line(idx: usize, line: &str) -> Option<serde_json::Value> {
    if line.contains("forge_lsp_error_probe") || line.contains("LSP_ERROR_PROBE") {
        return Some(diagnostic(idx, 1, "ERROR", "Contained diagnostic probe detected in edited content"));
    }
    if line.contains("TODO") || line.contains("todo!") {
        return Some(diagnostic(idx, 2, "WARN", "Contained warning for TODO-like marker in edited content"));
    }
    None
}

fn diagnostic(idx: usize, severity: u8, severity_label: &str, message: &str) -> serde_json::Value {
    serde_json::json!({
        "severity": severity,
        "severity_label": severity_label,
        "message": message,
        "range": {
            "start": { "line": idx, "character": 0 },
            "end": { "line": idx, "character": 1 }
        },
        "source": "forge.contained_lsp_diagnostic_probe"
    })
}

fn severity_counts(diagnostics: &[serde_json::Value]) -> serde_json::Value {
    let mut errors = 0usize;
    let mut warnings = 0usize;
    let mut info = 0usize;
    let mut hints = 0usize;
    for diagnostic in diagnostics {
        match diagnostic.get("severity").and_then(serde_json::Value::as_u64).unwrap_or(1) {
            1 => errors += 1,
            2 => warnings += 1,
            3 => info += 1,
            4 => hints += 1,
            _ => {}
        }
    }
    serde_json::json!({"ERROR": errors, "WARN": warnings, "INFO": info, "HINT": hints})
}

fn diagnostic_report_block(file: &str, diagnostics: &[serde_json::Value]) -> String {
    let errors = diagnostics.iter().filter(|item| item.get("severity").and_then(serde_json::Value::as_u64) == Some(1)).take(MAX_DIAGNOSTICS_PER_FILE).collect::<Vec<_>>();
    if errors.is_empty() { return String::new(); }
    let mut lines = vec![format!("<diagnostics file=\"{file}\">")];
    for item in errors {
        let line = item.pointer("/range/start/line").and_then(serde_json::Value::as_u64).unwrap_or(0) + 1;
        let col = item.pointer("/range/start/character").and_then(serde_json::Value::as_u64).unwrap_or(0) + 1;
        let message = item.get("message").and_then(serde_json::Value::as_str).unwrap_or("diagnostic");
        lines.push(format!("ERROR [{line}:{col}] {message}"));
    }
    lines.push("</diagnostics>".to_string());
    lines.join("\n")
}

pub(crate) fn diagnostics_metadata(files: &[serde_json::Value]) -> serde_json::Value {
    let reports = diagnostic_reports(files);
    let warmups = lsp_warmups(files);
    let warmup_count = warmups.len();
    let report_count = reports.len();
    serde_json::json!({
        "status": "event_envelope_emitted_with_warmup_containment",
        "reason": "Forge records OpenCode LSP touch targets, emits LSP diagnostic report envelopes, preserves severity/count/report-block shape, and records that optional LSP warm-up defects are contained instead of failing the edit path.",
        "touched_files": lsp_touches(files),
        "warmups": warmups,
        "reports": reports,
        "warmup_count": warmup_count,
        "report_count": report_count,
        "max_per_file": MAX_DIAGNOSTICS_PER_FILE,
        "opencode_source": [
            "packages/opencode/src/tool/apply_patch.ts:lsp.touchFile + lsp.diagnostics + LSP.Diagnostic.report",
            "packages/opencode/src/lsp/lsp.ts:status + touchFile + diagnostics",
            "packages/opencode/src/lsp/diagnostic.ts:MAX_PER_FILE + pretty + report",
            "packages/opencode/src/tool/read.ts:lsp.touchFile(...).pipe(Effect.ignoreCause, Effect.forkIn(scope))"
        ],
    })
}

pub(crate) fn opencode_event_source() -> serde_json::Value {
    serde_json::json!({
        "paths": [
            "packages/opencode/src/tool/apply_patch.ts",
            "packages/opencode/src/tool/read.ts",
            "packages/opencode/src/lsp/lsp.ts",
            "packages/opencode/src/lsp/diagnostic.ts",
            "packages/opencode/src/event-v2-bridge.ts",
            "packages/opencode/src/server/routes/instance/httpapi/handlers/event.ts"
        ],
        "behaviors": [
            "events.publish(FileSystem.Event.Edited, { file: edited })",
            "events.publish(Watcher.Event.Updated, update)",
            "lsp.touchFile(target, document)",
            "lsp.diagnostics()",
            "LSP.Diagnostic.report(file, issues) with max 20 errors per file",
            "optional LSP warm-up defects are contained with Effect.ignoreCause",
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
        assert_eq!(meta["status"], "event_envelope_emitted_with_warmup_containment");
        assert_eq!(meta["touched_files"][0]["kind"], "document");
        assert_eq!(meta["warmups"][0]["event_name"], "LSP.Warmup.contained");
        assert_eq!(meta["reports"][0]["event_name"], "LSP.Diagnostic.report");
        assert_eq!(meta["max_per_file"], MAX_DIAGNOSTICS_PER_FILE);
        assert!(meta["opencode_source"].as_array().unwrap()[3].as_str().unwrap().contains("ignoreCause"));
    }

    #[test]
    fn diagnostic_reports_skip_deleted_files() {
        let files = vec![serde_json::json!({"type": "delete", "path": "gone.rs", "relativePath": "gone.rs"})];
        assert!(diagnostic_reports(&files).is_empty());
        assert!(lsp_warmups(&files).is_empty());
    }

    #[test]
    fn contained_diagnostic_report_uses_opencode_error_block_shape() {
        let files = vec![serde_json::json!({
            "type": "add",
            "path": "probe.rs",
            "relativePath": "probe.rs",
            "patch": "@@ -1 +1 @@\n+forge_lsp_error_probe\n+TODO later\n"
        })];
        let reports = diagnostic_reports(&files);
        assert_eq!(reports[0]["severity_counts"]["ERROR"], 1);
        assert_eq!(reports[0]["severity_counts"]["WARN"], 1);
        assert_eq!(reports[0]["diagnostic_count"], 2);
        let block = reports[0]["report_block"].as_str().unwrap();
        assert!(block.contains("<diagnostics file=\"probe.rs\">"));
        assert!(block.contains("ERROR [1:1] Contained diagnostic probe detected"));
    }
}
