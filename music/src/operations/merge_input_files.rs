pub(crate) struct MergeInputFilesResult {
    pub(crate) output_file: std::path::PathBuf,
}

pub(crate) fn merge_input_files(
    input_dir: &std::path::Path,
    output_dir: &std::path::Path,
    remove_source_files: bool,
) -> Result<MergeInputFilesResult, crate::operations::OperationError> {
    let source_files = collect_json_files_in_dir(input_dir)?;
    let anonymous_videos = crate::validate::try_load_anonymous_videos(&source_files)?;

    let output_file = output_dir.join(format!(
        "{}.json",
        chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S")
    ));

    if let Some(parent) = output_file.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let file = std::fs::File::create(&output_file)?;
    serde_json::to_writer_pretty(file, &anonymous_videos)?;

    if remove_source_files {
        for file in &source_files {
            if let Err(error) = std::fs::remove_file(file) {
                tracing::warn!(
                    "Failed to remove source file {}: {}",
                    file.display(),
                    error
                );
            }
        }
    }

    Ok(MergeInputFilesResult { output_file })
}

fn collect_json_files_in_dir(
    dir: &std::path::Path,
) -> Result<Vec<std::path::PathBuf>, std::io::Error> {
    let mut json_files = Vec::new();

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.eq_ignore_ascii_case("json"))
            .unwrap_or(false)
        {
            json_files.push(path);
        }
    }

    json_files.sort_unstable();
    Ok(json_files)
}
