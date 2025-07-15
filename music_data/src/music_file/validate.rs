/// 音楽情報ルートディレクトリ直下の年フォルダ群を検証し、各年の情報を返す
///
/// Arguments:
/// - `year_entries`: 音楽情報ルートディレクトリ直下のエントリ一覧
///
/// Errors:
/// - 引数`year_entries`が空の場合
/// - ディレクトリ構造が不正な場合
pub(super) fn collect_and_validate_year_directories(
    year_entries: &[std::fs::DirEntry],
) -> Result<Vec<super::MusicFilePath>, super::MusicFileErrors> {
    if year_entries.is_empty() {
        return Err(super::MusicFileError::YearFolderError(
            "No entries found in the music root directory".to_string(),
        )
        .into_errors());
    }

    let mut years: Vec<super::MusicFilePath> = Vec::new();
    let mut errors: Vec<super::MusicFileError> = Vec::new();

    // 1つのファイルの一部にだけエラーが無かったとしても他のファイルをすべてparseしたいので,
    // エラーが見つかるとそのファイルやフォルダ配下を飛ばしてparseを続行する
    for year_entry in year_entries {
        let year = match validate_year_directory(year_entry) {
            Ok(year) => year,
            Err(e) => {
                errors.push(e);
                continue;
            }
        };
        let monthly_entries = match super::fs_util::read_dir(&year_entry.path()) {
            Ok(entries) => entries,
            Err(e) => {
                errors.push(e);
                continue;
            }
        };

        let monthly_files = match collect_and_validate_month_files(&monthly_entries) {
            Ok(files) => files,
            Err(e) => {
                // 親フォルダのパスがエラー情報に欲しいのでErrの返却値はmsgのみで,
                // 呼び出し元(ここ)で親フォルダのパスを付与
                errors.push(super::MusicFileError::MonthFileError {
                    underlying: year_entry.path().display().to_string(),
                    msg: e,
                });
                continue;
            }
        };

        monthly_files.into_iter().for_each(|(file, month)| {
            years.push(super::MusicFilePath::new(year, month, file));
        });
    }

    if errors.is_empty() {
        Ok(years)
    } else {
        Err(super::MusicFileErrors { errs: errors })
    }
}

/// 年単位のフォルダであることを検証
///
/// そのフォルダ**のみ**を対象. 子ファイルなどは検証の対象外.
///
/// # Arguments:
/// - `entry`: 年ディレクトリエントリ
///
/// # Errors:
/// - ディレクトリでない場合
/// - ディレクトリ名が4桁の数字でない場合
fn validate_year_directory(
    entry: &std::fs::DirEntry,
) -> Result<usize, super::MusicFileError> {
    use super::MusicFileError;

    if !entry.file_type().map(|f| f.is_dir()).unwrap_or(false) {
        return Err(MusicFileError::YearFolderError(format!(
            "{} is not a directory",
            entry.path().display()
        )));
    }

    let dir_name = entry.file_name();
    let name = dir_name.to_str().ok_or_else(|| {
        MusicFileError::YearFolderError(format!(
            "Invalid UTF-8 in directory name: {}",
            dir_name.display()
        ))
    })?;

    if name.len() != 4 || name.chars().any(|c| !c.is_ascii_digit()) {
        return Err(MusicFileError::YearFolderError(format!(
            "Directory {} is not a valid year directory name",
            entry.path().display()
        )));
    }

    name.parse().map_err(|_| {
        MusicFileError::YearFolderError(format!(
            "invalid year directory name: {}",
            entry.path().display()
        ))
    })
}

/// 月単位のファイル名であることを検証し, 全ての月ファイルのパスを返す
///
/// - 年単位のフォルダ直下に存在
/// - 年単位のフォルダ直下にはちょうど12個の月ファイルが必須
/// - ファイル名はMM.json
///   - `01.json`, `02.json`, ..., `12.json` の12個
///
/// # Arguments:
/// - `entries`: 年単位のフォルダ直下のエントリ一覧
///
/// # Returns:
/// - `Ok(Vec<(FilePath, usize)>)`: 正常な場合. 月ファイルと対応した月の数字, の配列を返す
/// - `Err(String)`: 無効なファイルが含まれている場合. 適切なエラーメッセージを返す
fn collect_and_validate_month_files(
    entries: &[std::fs::DirEntry],
) -> Result<Vec<(crate::util::FilePath, usize)>, String> {
    const MONTH_FILE_EXT: &str = ".json";
    let mut monthly_files: Vec<(crate::util::FilePath, usize)> = Vec::new();

    if entries.len() != 12 {
        return Err(format!(
            "There must be exactly 12 month files, but found {}",
            entries.len()
        ));
    }

    for entry in entries {
        let name = entry.file_name();
        let name = name.to_str().ok_or("Invalid UTF-8 in file name")?;

        if name.len() != 7 || !name.ends_with(MONTH_FILE_EXT) {
            return Err(format!(
                "File {} is not a valid month file name",
                entry.path().display()
            ));
        }

        // 上でfile_base_nameを2文字であることを保証
        // また, ここで, ascii限定でパースしているので月の数字が重複することはない
        let num = name
            .get(..2)
            .map(|s| s.parse::<usize>())
            .and_then(Result::ok)
            .ok_or_else(|| format!("{name} is invalid file name"))?;

        let file_path = crate::util::FilePath::from_path_buf(entry.path())
            .map_err(|e| format!("Failed to create FilePath: {e}"))?;
        monthly_files.push((file_path, num));
    }
    Ok(monthly_files)
}

// MARK: Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_year_directory_ok() {
        let dir = tempfile::tempdir().unwrap();
        let year_dir = dir.path().join("2022");
        std::fs::create_dir(&year_dir).unwrap();
        let entry = std::fs::read_dir(dir.path())
            .unwrap()
            .next()
            .unwrap()
            .unwrap();
        let year = validate_year_directory(&entry).unwrap();
        assert_eq!(year, 2022);
    }

    #[test]
    fn test_validate_year_directory_invalid_name() {
        let dir = tempfile::tempdir().unwrap();
        let year_dir = dir.path().join("abcd");
        std::fs::create_dir(&year_dir).unwrap();
        let entry = std::fs::read_dir(dir.path())
            .unwrap()
            .next()
            .unwrap()
            .unwrap();
        let err = validate_year_directory(&entry).unwrap_err();
        match err {
            super::super::MusicFileError::YearFolderError(msg) => {
                assert!(
                    msg.contains("not a valid year directory name")
                        || msg.contains("invalid year directory name")
                );
            }
            _ => panic!("Expected YearFolderError, got: {err:?}"),
        }
    }

    #[test]
    fn test_collect_and_validate_month_files_ok() {
        let dir = tempfile::tempdir().unwrap();
        for i in 1..=12 {
            let fname = format!("{i:02}.json");
            std::fs::File::create(dir.path().join(fname)).unwrap();
        }
        let entries: Vec<_> = std::fs::read_dir(dir.path())
            .unwrap()
            .map(|e| e.unwrap())
            .collect();
        let result = collect_and_validate_month_files(&entries);
        assert!(result.is_ok());
        let files = result.unwrap();
        assert_eq!(files.len(), 12);
        for (fp, m) in files {
            assert!((1..=12).contains(&m));
            assert!(fp.as_path().ends_with(format!("{m:02}.json")));
        }
    }

    #[test]
    fn test_collect_and_validate_month_files_too_few() {
        let dir = tempfile::tempdir().unwrap();
        for i in 1..=11 {
            let fname = format!("{i:02}.json");
            std::fs::File::create(dir.path().join(fname)).unwrap();
        }
        let entries: Vec<_> = std::fs::read_dir(dir.path())
            .unwrap()
            .map(|e| e.unwrap())
            .collect();
        let err = collect_and_validate_month_files(&entries).unwrap_err();
        assert!(err.contains("12 month files"));
    }

    #[test]
    fn test_collect_and_validate_month_files_invalid_name() {
        let dir = tempfile::tempdir().unwrap();
        for i in 1..=11 {
            let fname = format!("{i:02}.json");
            std::fs::File::create(dir.path().join(fname)).unwrap();
        }
        std::fs::File::create(dir.path().join("ab.json")).unwrap();
        let entries: Vec<_> = std::fs::read_dir(dir.path())
            .unwrap()
            .map(|e| e.unwrap())
            .collect();
        let err = collect_and_validate_month_files(&entries).unwrap_err();
        assert!(
            err.contains("invalid file name")
                || err.contains("not a valid month file name")
        );
    }

    #[test]
    fn test_collect_and_validate_year_directories_ok() {
        let dir = tempfile::tempdir().unwrap();
        for y in [2021, 2022] {
            let year_dir = dir.path().join(format!("{y}"));
            std::fs::create_dir(&year_dir).unwrap();
            for m in 1..=12 {
                std::fs::File::create(year_dir.join(format!("{m:02}.json"))).unwrap();
            }
        }
        let year_entries: Vec<_> = std::fs::read_dir(dir.path())
            .unwrap()
            .map(|e| e.unwrap())
            .collect();
        let files = collect_and_validate_year_directories(&year_entries).unwrap();
        assert_eq!(files.len(), 24);
        for y in [2021, 2022] {
            for m in 1..=12 {
                assert!(
                    files
                        .iter()
                        .any(|f| f.get_year() == y && f.get_month() == m)
                );
            }
        }
    }

    #[test]
    fn test_collect_and_validate_year_directories_invalid() {
        let dir = tempfile::tempdir().unwrap();
        let year_dir = dir.path().join("2022");
        std::fs::create_dir(&year_dir).unwrap();
        for m in 1..=11 {
            std::fs::File::create(year_dir.join(format!("{m:02}.json"))).unwrap();
        }
        let year_entries: Vec<_> = std::fs::read_dir(dir.path())
            .unwrap()
            .map(|e| e.unwrap())
            .collect();
        let err = collect_and_validate_year_directories(&year_entries).unwrap_err();
        assert!(
            err.errs.iter().any(|e| matches!(
                e,
                super::super::MusicFileError::MonthFileError { .. }
            ))
        );
    }
}
