/// 特定のディレクトリ内の特定の拡張子のファイルを再帰的に探索
///
/// Arguments
/// - `dir`: 探索するディレクトリのパス
/// - `extension`: 探索するファイルの拡張子.
///   - 先頭のdot`.`は不問
///   - case-insensitive
pub fn find_files_by_extension(
    dir: &crate::util::DirPath,
    extension: &str,
) -> Vec<crate::util::FilePath> {
    let mut files = Vec::new();
    let walker = walkdir::WalkDir::new(dir.clone().into_path_buf()).into_iter();
    let extension = extension.trim_start_matches('.').to_lowercase();

    for entry in walker
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        if let Some(ext) = entry.path().extension().and_then(|s| s.to_str()) {
            if ext.eq_ignore_ascii_case(&extension) {
                if let Ok(file_path) = crate::util::FilePath::new(entry.path()) {
                    files.push(file_path);
                }
            }
        }
    }
    files
}
