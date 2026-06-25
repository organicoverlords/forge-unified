use anyhow::Result;
use std::path::Path;

pub struct FileFinder {
    include_patterns: Vec<String>,
    exclude_patterns: Vec<String>,
    max_depth: u32,
    follow_symlinks: bool,
}

impl FileFinder {
    pub fn new() -> Result<Self> {
        let include_patterns: Vec<String> = vec![
            "*.rs", "*.py", "*.ts", "*.js", "*.jsx", "*.tsx",
            "*.go", "*.java", "*.cpp", "*.cc", "*.cxx", "*.c",
            "*.h", "*.hpp", "*.hh", "*.cs", "*.kt", "*.scala",
            "*.php", "*.swift", "*.lua", "*.luau", "*.zig",
            "*.ps1", "*.psm1", "*.rb", "*.d", "*.dart",
        ].into_iter().map(String::from).collect();

        let exclude_patterns: Vec<String> = vec![
            "target/", "debug/", "build/", "dist/",
            "node_modules/", "__pycache__/", ".pytest_cache/",
            ".git/", ".svn/", ".hg/",
            "tests/", "test/", "spec/", "fixtures/",
            "*.min.js", "*.bundle.js", "*.map",
            "venv/", ".venv/", "env/", ".env",
            "*.class", "*.pyc", "*.pyo", "*.pyd",
            "*.o", "*.obj", "*.exe", "*.dll",
            "*.png", "*.jpg", "*.jpeg", "*.gif", "*.svg",
            "*.pdf", "*.doc", "*.docx", "*.xls", "*.xlsx",
        ].into_iter().map(String::from).collect();

        Ok(Self {
            include_patterns,
            exclude_patterns,
            max_depth: 10,
            follow_symlinks: false,
        })
    }

    pub async fn find_files(&self, patterns: &[String]) -> Result<Vec<String>> {
        let mut found_files = Vec::new();

        for pattern in patterns {
            let path = Path::new(pattern);

            if path.exists() {
                if path.is_dir() {
                    let files = self.find_files_in_directory(&path, 0).await?;
                    found_files.extend(files);
                } else if self.should_include_file(path) {
                    found_files.push(path.to_string_lossy().to_string());
                }
            }
        }

        found_files.dedup();
        Ok(found_files)
    }

    async fn find_files_in_directory(
        &self,
        dir: &Path,
        current_depth: u32,
    ) -> Result<Vec<String>> {
        Box::pin(async move {
            if current_depth >= self.max_depth {
                return Ok(Vec::new());
            }

            let mut files = Vec::new();

            match tokio::fs::read_dir(dir).await {
                Ok(mut entries) => {
                    while let Some(entry) = entries.next_entry().await? {
                        let path = entry.path();

                        if path.is_dir() {
                            if self.follow_symlinks || !path.is_symlink() {
                                let sub_files = self.find_files_in_directory(&path, current_depth + 1).await?;
                                files.extend(sub_files);
                            }
                        } else if self.should_include_file(&path) {
                            files.push(path.to_string_lossy().to_string());
                        }
                    }
                }
                Err(_) => {}
            }

            Ok(files)
        }).await
    }

    fn should_include_file(&self, path: &Path) -> bool {
        if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
            let matches_include = self.include_patterns.iter().any(|pattern| {
                if pattern.starts_with('*') {
                    let suffix = &pattern[1..];
                    extension.ends_with(suffix)
                } else {
                    extension == pattern
                }
            });

            if !matches_include {
                return false;
            }
        } else {
            return false;
        }

        let path_str = path.to_string_lossy();

        for exclude in &self.exclude_patterns {
            if path_str.contains(exclude) {
                return false;
            }
        }

        true
    }
}
