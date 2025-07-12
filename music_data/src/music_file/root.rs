/// 音楽情報のルートフォルダであることを保証する
#[derive(Debug, Clone)]
pub struct MusicRoot {
    root: crate::util::DirPath,
    files: Vec<MusicRootYear>,
}

#[derive(Debug, Clone)]
pub struct MusicRootYear {
    year: usize,
    monthly_files: Vec<crate::util::FilePath>,
}

impl MusicRoot {
    /// MusicRootを作成
    ///
    /// Argument:
    /// - `path`: 音楽情報のルートディレクトリパス
    ///
    /// Errors
    /// - ディレクトリ構造が不正な場合
    pub fn new(path: &crate::util::DirPath) -> Result<Self, String> {
        let entries: Vec<std::fs::DirEntry> = read_dir(path.as_path())?;
        collect_and_validate_year_directories(&entries).map(|files| Self {
            root: path.clone(),
            files,
        })
    }

    pub fn get_file_paths(&self) -> Vec<crate::util::FilePath> {
        self.files
            .iter()
            .flat_map(|y| y.monthly_files.iter())
            .cloned()
            .collect()
    }
}

/// 音楽情報ルートディレクトリ直下の年フォルダ群を検証し、各年の情報を返す
///
/// Arguments:
/// - `entries`: 音楽情報ルートディレクトリ直下のエントリ一覧
///
/// Errors:
/// - 1つでも年単位のフォルダでないものがある場合
/// - Vecが空の場合
fn collect_and_validate_year_directories(
    entries: &[std::fs::DirEntry],
) -> Result<Vec<MusicRootYear>, String> {
    if entries.is_empty() {
        return Err("No entries found in the music root directory".to_string());
    }

    let mut years: Vec<MusicRootYear> = Vec::new();

    for entry in entries {
        let year = validate_year_directory(entry)?;
        let inner_entries = read_dir(&entry.path())?;
        let monthly_files = collect_and_validate_month_files(&inner_entries)
            .map_err(|e| format!("Under the {}, {}", entry.path().display(), e))?;
        years.push(MusicRootYear {
            year,
            monthly_files,
        });
    }

    Ok(years)
}

/// 年単位のフォルダであることを検証
///
/// そのフォルダ**のみ**を対象. 子ファイルなどは検証の対象外.
///
/// Argument:
/// - `entry`: 年ディレクトリエントリ
///
/// Errors:
/// - ディレクトリでない場合
/// - ディレクトリ名が4桁の数字でない場合
fn validate_year_directory(entry: &std::fs::DirEntry) -> Result<usize, String> {
    if !entry.file_type().map(|f| f.is_dir()).unwrap_or(false) {
        return Err(format!(
            "Entry {} is not a directory",
            entry.path().display()
        ));
    }

    let file_name = entry.file_name();
    let name = file_name.to_str().ok_or_else(|| {
        format!("Invalid UTF-8 in directory name: {:?}", file_name.display())
    })?;
    if name.len() != 4 || name.chars().any(|c| !c.is_ascii_digit()) {
        return Err(format!(
            "Directory {} is not a valid year directory name",
            entry.path().display()
        ));
    }

    name.parse()
        .map_err(|_| format!("Failed to parse year from directory name: {name}"))
}

/// 月単位のファイル名であることを検証し, 全ての月ファイルのパスを返す
///
/// - 年単位のフォルダ直下に存在
/// - 年単位のフォルダ直下にはちょうど12個の月ファイルが必須
/// - ファイル名はMM.json
///   - `01.json`, `02.json`, ..., `12.json` の12個
///
/// Arguments:
/// - `entries`: 年単位のフォルダ直下のエントリ一覧
///
/// # Returns:
/// - `Ok(Vec<FilePath>)`: 正常な場合. 全ての月ファイルのパスが含まれる
/// - `Err(String)`: 無効なファイルが含まれている場合
fn collect_and_validate_month_files(
    entries: &[std::fs::DirEntry],
) -> Result<Vec<crate::util::FilePath>, String> {
    const MONTH_FILE_EXT: &str = ".json";
    let mut exists_month_num = [false; 12];
    let mut monthly_files: Vec<crate::util::FilePath> = Vec::new();

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

        let num = name
            .get(..2)
            .map(|s| s.parse::<usize>())
            .and_then(Result::ok)
            .and_then(|s| exists_month_num.get_mut(s - 1))
            .ok_or_else(|| format!("{name} is invalid file name"))?;

        if *num {
            return Err(format!(
                "Duplicate month file found: {}",
                entry.path().display()
            ));
        } else {
            *num = true;
        }

        monthly_files.push(
            crate::util::FilePath::from_path_buf(entry.path())
                .map_err(|e| format!("Failed to create FilePath: {e}"))?,
        );
    }

    if !exists_month_num.iter().all(|&exists| exists) {
        return Err("Not all month files are present".to_string());
    }
    Ok(monthly_files)
}

fn read_dir(path: &std::path::Path) -> Result<Vec<std::fs::DirEntry>, String> {
    std::fs::read_dir(path)
        .map_err(|e| format!("Failed to read directory {}: {}", path.display(), e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to read directory entries: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_and_validate_month_files_ok() {
        let dir = tempfile::tempdir().unwrap();
        for i in 1..=12 {
            let fname = format!("{i:02}.json");
            std::fs::File::create(dir.path().join(fname)).unwrap();
        }
        let entries = read_dir(dir.path()).unwrap();
        let result = collect_and_validate_month_files(&entries);
        assert!(result.is_ok());
        let files = result.unwrap();
        assert_eq!(files.len(), 12);
    }

    #[test]
    fn test_collect_and_validate_month_files_too_few() {
        let dir = tempfile::tempdir().unwrap();
        for i in 1..=11 {
            let fname = format!("{i:02}.json");
            std::fs::File::create(dir.path().join(fname)).unwrap();
        }
        let entries = read_dir(dir.path()).unwrap();
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
        let entries = read_dir(dir.path()).unwrap();
        let err = collect_and_validate_month_files(&entries).unwrap_err();
        assert!(
            err.contains("invalid file name")
                || err.contains("not a valid month file name")
        );
    }

    #[test]
    fn test_read_dir_ok() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::File::create(dir.path().join("foo.txt")).unwrap();
        let entries = read_dir(dir.path()).unwrap();
        assert_eq!(entries.len(), 1);
    }

    #[test]
    fn test_read_dir_not_exist() {
        let dir = tempfile::tempdir().unwrap();
        let not_exist = dir.path().join("nope");
        let err = read_dir(&not_exist).unwrap_err();
        assert!(err.contains("Failed to read directory"));
    }

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
        assert!(err.contains("not a valid year directory name"));
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
        let entries = read_dir(dir.path()).unwrap();
        let years = collect_and_validate_year_directories(&entries).unwrap();
        assert_eq!(years.len(), 2);
        assert!(years.iter().any(|y| y.year == 2021));
        assert!(years.iter().any(|y| y.year == 2022));
        for y in years {
            assert_eq!(y.monthly_files.len(), 12);
        }
    }

    #[test]
    fn test_collect_and_validate_year_directories_invalid() {
        let dir = tempfile::tempdir().unwrap();
        let year_dir = dir.path().join("2022");
        std::fs::create_dir(&year_dir).unwrap();
        // 11個しか作らない
        for m in 1..=11 {
            std::fs::File::create(year_dir.join(format!("{m:02}.json"))).unwrap();
        }
        let entries = read_dir(dir.path()).unwrap();
        let err = collect_and_validate_year_directories(&entries).unwrap_err();
        assert!(err.contains("12 month files"));
    }
}
