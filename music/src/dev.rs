/// 既存の楽曲情報ファイルから, 指定された動画IDが重複しているかどうか確認
///
/// Arguments:
/// - `music_root`: 楽曲情報のルートフォルダ
/// - `video_ids`: 重複を確認したい動画IDのリスト
///
/// Returns:
/// - Ok(a): 重複していた動画id
pub fn find_video_ids(
    music_lib: &crate::music_file::MusicLibrary,
    video_ids: &[crate::model::VideoId],
) -> crate::model::VideoIds {
    let video_ids_set: std::collections::HashSet<_> =
        music_lib.get_video_ids().into_iter().collect();
    video_ids
        .iter()
        .filter_map(|id| video_ids_set.get(id))
        .cloned()
        .collect()
}

/// いわゆる`input`の楽曲情報をmergeして一つのファイルにする
///
/// `files`が提供された場合は, `dir`は無視される
///
/// # Returns
/// - Ok(path): マージされたファイルのパスのリスト
/// - Err(e): エラーが発生した場合のエラーメッセージ
pub fn merge_files(
    files: Option<Vec<crate::util::FilePath>>,
    dir: &crate::util::DirPath,
    output_dir: crate::util::DirPath,
) -> Result<Vec<crate::util::FilePath>, String> {
    let files: Vec<crate::util::FilePath> = match (files, dir) {
        (Some(files), dir) => {
            println!("dir(`{dir}`) is ignored because `files` is provided.");
            files
        }
        (None, dir) => collect_json_paths_from_dir(dir)?,
    };

    let videos = crate::validate::try_load_anonymous_videos(&files)
        .map_err(|e| e.to_pretty_string())?;

    let file = output_dir.into_path_buf().join(format!(
        "{}.json",
        chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S")
    ));
    let file = std::fs::File::create(&file)
        .map_err(|e| format!("Failed to access file`{}`: {e}", file.display()))?;
    serde_json::to_writer_pretty(file, &videos).map_err(|e| e.to_string())?;
    Ok(files)
}

fn collect_json_paths_from_dir(
    dir: &crate::util::DirPath,
) -> Result<Vec<crate::util::FilePath>, String> {
    let mut paths = Vec::new();
    for entry in std::fs::read_dir(dir.as_path()).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
            paths.push(crate::util::FilePath::from_path_buf(entry.path())?);
        }
    }
    Ok(paths)
}
