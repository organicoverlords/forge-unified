//! OpenCode-compatible apply_patch filesystem mutation helpers.

use crate::tool::patch_ops::{validate_relative_patch_path, PatchHunk, UpdateChunk};
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tokio::fs;

const UTF8_BOM: &[u8] = b"\xEF\xBB\xBF";

#[derive(Debug, Clone)]
pub(crate) struct FileChange {
    pub(crate) path: String,
    full_path: PathBuf,
    pub(crate) move_path: Option<String>,
    move_full_path: Option<PathBuf>,
    old_content: String,
    new_content: String,
    change_type: &'static str,
    diff: String,
    additions: usize,
    deletions: usize,
    bom: bool,
}

#[derive(Debug, Clone)]
struct TextFile {
    text: String,
    bom: bool,
}

pub(crate) async fn prepare_file_changes(hunks: &[PatchHunk], workspace_root: &str) -> Result<Vec<FileChange>> {
    let mut changes = Vec::new();
    for hunk in hunks {
        match hunk {
            PatchHunk::Add { path, contents } => {
                let full_path = workspace_path(workspace_root, path)?;
                let next = split_text_value(contents);
                let new_content = ensure_trailing_newline(&next.text);
                let diff = generate_unified_diff("", &new_content);
                let (additions, deletions) = count_diff_lines("", &new_content);
                changes.push(FileChange {
                    path: path.clone(), full_path, move_path: None, move_full_path: None,
                    old_content: String::new(), new_content, change_type: "add", diff, additions, deletions, bom: next.bom,
                });
            }
            PatchHunk::Delete { path } => {
                let full_path = workspace_path(workspace_root, path)?;
                let old_file = read_text_file(&full_path).await
                    .with_context(|| format!("apply_patch verification failed: Failed to read file to delete: {}", full_path.display()))?;
                let diff = generate_unified_diff(&old_file.text, "");
                let deletions = line_count(&old_file.text);
                changes.push(FileChange {
                    path: path.clone(), full_path, move_path: None, move_full_path: None,
                    old_content: old_file.text, new_content: String::new(), change_type: "delete", diff,
                    additions: 0, deletions, bom: old_file.bom,
                });
            }
            PatchHunk::Update { path, move_path, chunks } => {
                let full_path = workspace_path(workspace_root, path)?;
                let metadata = fs::metadata(&full_path).await.ok();
                if metadata.as_ref().map_or(true, |meta| meta.is_dir()) {
                    anyhow::bail!("apply_patch verification failed: Failed to read file to update: {}", full_path.display());
                }
                let old_file = read_text_file(&full_path).await
                    .with_context(|| format!("apply_patch verification failed: Failed to read file to update: {}", full_path.display()))?;
                let new_content = derive_new_contents_from_chunks(path, chunks, &old_file.text)?;
                let move_full_path = move_path.as_deref().map(|target| workspace_path(workspace_root, target)).transpose()?;
                let diff = generate_unified_diff(&old_file.text, &new_content);
                let (additions, deletions) = count_diff_lines(&old_file.text, &new_content);
                changes.push(FileChange {
                    path: path.clone(), full_path, move_path: move_path.clone(), move_full_path,
                    old_content: old_file.text, new_content, change_type: if move_path.is_some() { "move" } else { "update" },
                    diff, additions, deletions, bom: old_file.bom,
                });
            }
        }
    }
    Ok(changes)
}

pub(crate) async fn apply_file_changes(changes: &[FileChange]) -> Result<()> {
    for change in changes {
        match change.change_type {
            "add" | "update" => write_with_dirs(&change.full_path, &change.new_content, change.bom).await?,
            "move" => {
                let Some(target) = change.move_full_path.as_deref() else {
                    anyhow::bail!("apply_patch verification failed: missing move target");
                };
                write_with_dirs(target, &change.new_content, change.bom).await?;
                fs::remove_file(&change.full_path).await
                    .with_context(|| format!("Failed to remove moved source: {}", change.full_path.display()))?;
            }
            "delete" => {
                fs::remove_file(&change.full_path).await
                    .with_context(|| format!("Failed to delete file: {}", change.full_path.display()))?;
            }
            other => anyhow::bail!("apply_patch verification failed: unknown change type {other}"),
        }
    }
    Ok(())
}

pub(crate) fn file_change_metadata(change: &FileChange) -> serde_json::Value {
    serde_json::json!({
        "path": &change.path,
        "relativePath": change.move_path.as_deref().unwrap_or(&change.path),
        "type": change.change_type,
        "movePath": change.move_path.as_deref(),
        "patch": &change.diff,
        "additions": change.additions,
        "deletions": change.deletions,
        "oldBytes": change.old_content.len(),
        "newBytes": change.new_content.len(),
        "bom": change.bom,
    })
}

pub(crate) fn file_change_summary_line(change: &FileChange) -> String {
    match change.change_type {
        "add" => format!("A {}", change.path),
        "delete" => format!("D {}", change.path),
        _ => format!("M {}", change.move_path.as_deref().unwrap_or(&change.path)),
    }
}

pub(crate) fn total_diff(changes: &[FileChange]) -> String {
    changes.iter().map(|change| change.diff.as_str()).collect::<Vec<_>>().join("\n")
}

async fn write_with_dirs(path: &Path, content: &str, bom: bool) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await?;
    }
    fs::write(path, join_bom(content, bom)).await
        .with_context(|| format!("Failed to write file: {}", path.display()))
}

async fn read_text_file(path: &Path) -> Result<TextFile> {
    let bytes = fs::read(path).await?;
    split_text_bytes(path, &bytes)
}

fn split_text_bytes(path: &Path, bytes: &[u8]) -> Result<TextFile> {
    let bom = bytes.starts_with(UTF8_BOM);
    let text_bytes = if bom { &bytes[UTF8_BOM.len()..] } else { bytes };
    let text = String::from_utf8(text_bytes.to_vec())
        .with_context(|| format!("File is not valid UTF-8: {}", path.display()))?;
    Ok(TextFile { text, bom })
}

fn split_text_value(input: &str) -> TextFile {
    if let Some(text) = input.strip_prefix('\u{feff}') {
        TextFile { text: text.to_string(), bom: true }
    } else {
        TextFile { text: input.to_string(), bom: false }
    }
}

fn join_bom(content: &str, bom: bool) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(content.len() + if bom { UTF8_BOM.len() } else { 0 });
    if bom {
        bytes.extend_from_slice(UTF8_BOM);
    }
    bytes.extend_from_slice(content.as_bytes());
    bytes
}

fn workspace_path(workspace_root: &str, raw_path: &str) -> Result<PathBuf> {
    validate_relative_patch_path(raw_path)?;
    Ok(Path::new(workspace_root).join(raw_path))
}

fn derive_new_contents_from_chunks(file_path: &str, chunks: &[UpdateChunk], original: &str) -> Result<String> {
    let mut original_lines: Vec<String> = original.split('\n').map(str::to_string).collect();
    if original_lines.last().is_some_and(|line| line.is_empty()) {
        original_lines.pop();
    }

    let replacements = compute_replacements(&original_lines, file_path, chunks)?;
    let mut new_lines = apply_replacements(&original_lines, &replacements);
    if new_lines.last().map_or(true, |line| !line.is_empty()) {
        new_lines.push(String::new());
    }
    Ok(new_lines.join("\n"))
}

fn compute_replacements(lines: &[String], file_path: &str, chunks: &[UpdateChunk]) -> Result<Vec<(usize, usize, Vec<String>)>> {
    let mut replacements = Vec::new();
    let mut line_index = 0;

    for chunk in chunks {
        if let Some(context) = &chunk.change_context {
            let Some(context_idx) = seek_sequence(lines, &[context.clone()], line_index, false) else {
                anyhow::bail!("Failed to find context '{}' in {}", context, file_path);
            };
            line_index = context_idx + 1;
        }

        if chunk.old_lines.is_empty() {
            let insertion_idx = if lines.last().is_some_and(|line| line.is_empty()) { lines.len() - 1 } else { lines.len() };
            replacements.push((insertion_idx, 0, chunk.new_lines.clone()));
            continue;
        }

        let mut pattern = chunk.old_lines.clone();
        let mut new_segment = chunk.new_lines.clone();
        let mut found = seek_sequence(lines, &pattern, line_index, chunk.is_end_of_file);
        if found.is_none() && pattern.last().is_some_and(|line| line.is_empty()) {
            pattern.pop();
            if new_segment.last().is_some_and(|line| line.is_empty()) {
                new_segment.pop();
            }
            found = seek_sequence(lines, &pattern, line_index, chunk.is_end_of_file);
        }

        let Some(found_idx) = found else {
            anyhow::bail!("Failed to find expected lines in {}:\n{}", file_path, chunk.old_lines.join("\n"));
        };
        replacements.push((found_idx, pattern.len(), new_segment));
        line_index = found_idx + pattern.len();
    }

    replacements.sort_by_key(|replacement| replacement.0);
    Ok(replacements)
}

fn apply_replacements(lines: &[String], replacements: &[(usize, usize, Vec<String>)]) -> Vec<String> {
    let mut result = lines.to_vec();
    for (start_idx, old_len, new_segment) in replacements.iter().rev() {
        result.splice(*start_idx..(*start_idx + *old_len), new_segment.clone());
    }
    result
}

fn seek_sequence(lines: &[String], pattern: &[String], start_index: usize, eof: bool) -> Option<usize> {
    if pattern.is_empty() || pattern.len() > lines.len() {
        return None;
    }
    try_match(lines, pattern, start_index, |a, b| a == b, eof)
        .or_else(|| try_match(lines, pattern, start_index, |a, b| a.trim_end() == b.trim_end(), eof))
        .or_else(|| try_match(lines, pattern, start_index, |a, b| a.trim() == b.trim(), eof))
        .or_else(|| try_match(lines, pattern, start_index, |a, b| normalize_unicode(a.trim()) == normalize_unicode(b.trim()), eof))
}

fn try_match<F>(lines: &[String], pattern: &[String], start_index: usize, compare: F, eof: bool) -> Option<usize>
where
    F: Fn(&str, &str) -> bool,
{
    if eof {
        let from_end = lines.len().checked_sub(pattern.len())?;
        if from_end >= start_index && pattern.iter().enumerate().all(|(idx, value)| compare(&lines[from_end + idx], value)) {
            return Some(from_end);
        }
    }

    let max_start = lines.len().checked_sub(pattern.len())?;
    for i in start_index..=max_start {
        if pattern.iter().enumerate().all(|(idx, value)| compare(&lines[i + idx], value)) {
            return Some(i);
        }
    }
    None
}

fn normalize_unicode(input: &str) -> String {
    let mut output = String::new();
    for ch in input.chars() {
        match ch {
            '‘' | '’' | '‚' | '‛' => output.push('\''),
            '“' | '”' | '„' | '‟' => output.push('"'),
            '‐' | '‑' | '‒' | '–' | '—' | '―' => output.push('-'),
            '…' => output.push_str("..."),
            '\u{00a0}' => output.push(' '),
            _ => output.push(ch),
        }
    }
    output
}

fn ensure_trailing_newline(input: &str) -> String {
    if input.is_empty() || input.ends_with('\n') { input.to_string() } else { format!("{input}\n") }
}

fn line_count(input: &str) -> usize {
    if input.is_empty() { 0 } else { input.lines().count() }
}

fn count_diff_lines(old_content: &str, new_content: &str) -> (usize, usize) {
    let additions = new_content.lines().filter(|line| !old_content.lines().any(|old| old == *line)).count();
    let deletions = old_content.lines().filter(|line| !new_content.lines().any(|new| new == *line)).count();
    (additions, deletions)
}

fn generate_unified_diff(old_content: &str, new_content: &str) -> String {
    let old_lines: Vec<&str> = old_content.split('\n').collect();
    let new_lines: Vec<&str> = new_content.split('\n').collect();
    let mut diff = "@@ -1 +1 @@\n".to_string();
    let mut changed = false;
    for i in 0..old_lines.len().max(new_lines.len()) {
        let old_line = old_lines.get(i).copied().unwrap_or("");
        let new_line = new_lines.get(i).copied().unwrap_or("");
        if old_line != new_line {
            if !old_line.is_empty() { diff.push_str(&format!("-{old_line}\n")); }
            if !new_line.is_empty() { diff.push_str(&format!("+{new_line}\n")); }
            changed = true;
        } else if !old_line.is_empty() {
            diff.push_str(&format!(" {old_line}\n"));
        }
    }
    if changed { diff } else { String::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derives_new_contents_with_trim_and_unicode_fallbacks() {
        let chunk = UpdateChunk {
            old_lines: vec!["let name = “old”;".to_string()],
            new_lines: vec!["let name = \"new\";".to_string()],
            change_context: None,
            is_end_of_file: false,
        };
        let next = derive_new_contents_from_chunks("src/main.rs", &[chunk], "fn main() {\n  let name = \"old\";\n}\n").unwrap();
        assert_eq!(next, "fn main() {\nlet name = \"new\";\n}\n");
    }

    #[test]
    fn roundtrips_utf8_bom_bytes() {
        let bytes = [UTF8_BOM, b"hello\n"].concat();
        let split = split_text_bytes(Path::new("example.txt"), &bytes).unwrap();
        assert!(split.bom);
        assert_eq!(split.text, "hello\n");
        assert_eq!(join_bom(&split.text, split.bom), bytes);
    }

    #[test]
    fn detects_bom_character_in_added_content() {
        let split = split_text_value("\u{feff}hello");
        assert!(split.bom);
        assert_eq!(split.text, "hello");
    }
}
