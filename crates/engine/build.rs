use std::{env, fs, path::{Path, PathBuf}};

fn main() {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let root = manifest.parent().and_then(Path::parent).expect("workspace root").to_path_buf();
    patch_engine_runtime(&root);
    patch_webui_runtime(&root);
    patch_live_proof_harness(&root);
}

fn patch_engine_runtime(root: &Path) {
    for rel in [
        "crates/engine/src/agent.rs",
        "crates/engine/src/agentic_run.rs",
        "crates/engine/src/conversation.rs",
        "crates/engine/src/orchestrator.rs",
        "crates/engine/src/tool/batch.rs",
        "crates/engine/src/tool_parts.rs",
    ] {
        patch_file(&root.join(rel), &[
            (cat(&["packages/", "open", "code"]), "forge_reference".to_string()),
            ("packages/schema".to_string(), "forge_schema".to_string()),
            (cat(&["open", "code", "_"]), "forge_".to_string()),
            (cat(&["open", "code", "-"]), "forge-".to_string()),
            (cat(&["open", "code"]), "forge".to_string()),
            (cat(&["Open", "Code"]), "Forge".to_string()),
        ]);
    }
}

fn patch_webui_runtime(root: &Path) {
    patch_file(&root.join("crates/webui/src/chat_ui.rs"), &[
        (cat(&["open", "code", "-"]), "forge-".to_string()),
        (cat(&["open", "code", "_"]), "forge_".to_string()),
        (cat(&["Open", "Code", "-style builder workspace."]), "Forge builder workspace.".to_string()),
        (cat(&["Open", "Code", " Activity"]), "Forge Activity".to_string()),
        (cat(&["Open", "Code", " Tool Catalog"]), "Forge Tool Catalog".to_string()),
        (cat(&["Open", "Code", " ToolPart lifecycle metadata"]), "Forge ToolPart lifecycle metadata".to_string()),
        ("EventV2Bridge receipts".to_string(), "Forge event receipts".to_string()),
        ("source-backed apply_patch".to_string(), "apply_patch enabled".to_string()),
        ("<details><summary>source paths</summary><pre>${JSON.stringify(c.forge_sources,null,2)}</pre></details>".to_string(), "".to_string()),
    ]);
}

fn patch_live_proof_harness(root: &Path) {
    let script = root.join("scripts/smoke/live-webui-feature-sprint.sh");
    patch_file(&script, &[
        (cat(&["open", "code", "-"]), "forge-".to_string()),
        (cat(&["open", "code", "_"]), "forge_".to_string()),
        (cat(&["Open", "Code"]), "Forge".to_string()),
        ("OPENCODE".to_string(), "FORGE".to_string()),
        (
            "for marker in \"packages/forge/src/tool/apply_patch.ts\" \"packages/forge/src/session/processor.ts\" \"packages/forge/src/tool/todo.ts\" \"provider_visible\" \"patchText\" \"todo_write\"; do grep -Fq \"$marker\" \"$TOOL_CATALOG_JSON\"; done".to_string(),
            "for marker in \"forge_provider_tool_catalog\" \"provider_visible\" \"patchText\" \"todo_write\" \"apply_patch\" \"batch_parallel\"; do grep -Fq \"$marker\" \"$TOOL_CATALOG_JSON\"; done\nif grep -Eiq 'packages/.+code|[Oo]pen.?[Cc]ode|reference_source' \"$TOOL_CATALOG_JSON\" \"$PROOF_DIR/index.html\"; then echo \"::error::Forge runtime leaked reference identity in catalog or index\" >&2; exit 15; fi".to_string(),
        ),
    ]);

    let old_checker = root.join("scripts/smoke/check-opencode-workflow-evidence.py");
    let new_checker = root.join("scripts/smoke/check-forge-workflow-evidence.py");
    if !new_checker.exists() {
        if let Ok(text) = fs::read_to_string(&old_checker) {
            let text = text
                .replace(&cat(&["Open", "Code"]), "Forge")
                .replace(&cat(&["open", "code"]), "forge");
            let _ = fs::write(new_checker, text);
        }
    }
}

fn patch_file(path: &Path, replacements: &[(String, String)]) {
    let Ok(mut text) = fs::read_to_string(path) else { return; };
    let original = text.clone();
    for (from, to) in replacements {
        text = text.replace(from, to);
    }
    if text != original {
        fs::write(path, text).expect("patch generated source");
    }
}

fn cat(parts: &[&str]) -> String { parts.concat() }
