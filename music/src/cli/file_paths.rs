/// 主にコマンドライン引数からファイルパスを受け取るための型
#[derive(Debug, Clone)]
pub struct FilePathsFromCli(String);

impl FilePathsFromCli {
    /// コマンドライン引数から受け取ったファイルパスをVec<`PathBuf`>に変換
    ///
    /// - delimiter: ',', '\t', '\r', '\n'
    /// - delimiterだけで構成されたpathは無視
    /// - pathの文字列はtrimされる
    pub fn into_file_paths(self) -> Vec<std::path::PathBuf> {
        self.split_paths()
            .into_iter()
            .map(std::path::PathBuf::from)
            .collect()
    }

    fn split_paths(&self) -> Vec<&str> {
        // ' 'をdelimiterに追加してしまうとパスの一部の空白でもsplitしてしまうので追加しない
        const DELIM: [char; 4] = [',', '\t', '\r', '\n'];
        const IGNORE_ONLY_CHARS: &str = ",\t\r\n ";

        self.0
            .split(DELIM)
            .map(str::trim)
            .filter(|s| {
                !s.is_empty() && !s.chars().all(|c| IGNORE_ONLY_CHARS.contains(c))
            })
            .collect()
    }
}

impl std::fmt::Display for FilePathsFromCli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for FilePathsFromCli {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = s.trim().to_string();
        Ok(FilePathsFromCli(path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_path_from_cli_split_paths() {
        let cases = [
            ("path1,path2,path3", vec!["path1", "path2", "path3"]),
            ("path1, , path2 , , path3", vec!["path1", "path2", "path3"]),
            (",,,", vec![]),
            ("path1, , , , path2", vec!["path1", "path2"]),
            ("\n\n\n\n, \t\r, \n, \r\r\r,\n", vec![]),
            (" path1 , path2 , path3 ", vec!["path1", "path2", "path3"]),
            ("", vec![]),
        ];

        for (input, expected) in cases {
            let file_paths = FilePathsFromCli(input.to_string());
            assert_eq!(file_paths.split_paths(), expected);
        }
    }
}
