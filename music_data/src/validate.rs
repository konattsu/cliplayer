#[derive(Debug, Clone)]
pub struct VideoIdPublishedAt {
    pub video_id: crate::model::VideoId,
    pub published_at: crate::model::VideoPublishedAt,
}

/// 既存の楽曲情報ファイルから, 指定された動画IDが重複しているかどうか確認
///
/// Arguments:
/// - `music_root`: 楽曲情報のルートフォルダ
/// - `video_ids`: 重複を確認したい動画IDのリスト
///
/// Returns:
/// - Ok(a): 重複していた動画idとその動画が公開された日のリスト
/// - Err(e): 何らかのエラー. 整形済みの文字列
pub fn duplicate_video_ids(
    music_root: &crate::music_file::MusicRoot,
    video_ids: &[crate::model::VideoId],
) -> Result<Vec<VideoIdPublishedAt>, String> {
    let videos = crate::music_file::MusicRootContent::load(music_root)
        .map_err(|e| e.to_pretty_string())?
        .into_flattened_files();

    let video_ids_set = videos.into_inner();
    let video_ids_set: std::collections::HashMap<_, _> = video_ids_set
        .iter()
        .map(|v| (v.get_video_id(), v.get_published_at()))
        .collect();

    let duplicates: Vec<VideoIdPublishedAt> = video_ids
        .iter()
        .filter_map(|id| {
            video_ids_set.get(id).copied().map(|p| VideoIdPublishedAt {
                video_id: id.clone(),
                published_at: p.clone(),
            })
        })
        .collect();

    Ok(duplicates)
}

pub struct AnonymousVideoValidateErrors {
    errs: Vec<AnonymousVideoValidateError>,
}

pub enum AnonymousVideoValidateError {
    FileReadError {
        path: crate::util::FilePath,
        msg: String,
    },
    InvalidFileContent {
        path: crate::util::FilePath,
        msg: String,
    },
}

impl From<AnonymousVideoValidateErrors> for Vec<AnonymousVideoValidateError> {
    fn from(value: AnonymousVideoValidateErrors) -> Self {
        value.errs
    }
}

impl AnonymousVideoValidateErrors {
    pub fn to_pretty_string(&self) -> String {
        format!(
            "{}\n",
            self.errs
                .iter()
                .map(|e| e.to_pretty_string())
                .collect::<String>()
        )
    }
}

impl AnonymousVideoValidateError {
    /// エラーメッセージを整形して返す
    ///
    /// 文字列の最後に`\n`が付与される
    pub fn to_pretty_string(&self) -> String {
        match self {
            Self::FileReadError { path, msg } => {
                format!("Failed to read file {path}: {msg}\n",)
            }
            Self::InvalidFileContent { path, msg } => {
                format!("Invalid content in file {path}: {msg}\n",)
            }
        }
    }
}

/// 新規の楽曲情報ファイルの形式を検証
///
/// Error時はエラーメッセージを成形して表示
pub fn validate_new_input(files: &[crate::util::FilePath]) -> Result<(), String> {
    // deserializeできたらok
    let _videos = try_load_anonymous_videos(files).map_err(|e| e.to_pretty_string())?;
    Ok(())
}

pub fn try_load_anonymous_videos(
    files: &[crate::util::FilePath],
) -> Result<crate::model::AnonymousVideos, AnonymousVideoValidateErrors> {
    let mut errs = Vec::new();
    let mut videos: Vec<crate::model::AnonymousVideo> = Vec::new();

    for file in files {
        match deserialize_from_file(file) {
            Ok(v) => videos.extend(v.into_inner()),
            Err(e) => errs.push(e),
        }
    }

    if errs.is_empty() {
        Ok(crate::model::AnonymousVideos::new(videos))
    } else {
        Err(AnonymousVideoValidateErrors { errs })
    }
}

// TODO 通常と同じパースを適用している. 専用で処理するか要件等
/// 既存の楽曲情報に対する変更を検証
pub fn validate_update_input(
    music_root: &crate::music_file::MusicRoot,
) -> Result<(), String> {
    // deserializeできたらok
    let _videos = crate::music_file::MusicRootContent::load(music_root)
        .map_err(|e| e.to_pretty_string())?;

    Ok(())
}

fn deserialize_from_file(
    file: &crate::util::FilePath,
) -> Result<crate::model::AnonymousVideos, AnonymousVideoValidateError> {
    let file_handle = std::fs::File::open(file.as_path()).map_err(|e| {
        AnonymousVideoValidateError::FileReadError {
            path: file.clone(),
            msg: e.to_string(),
        }
    })?;

    let reader = std::io::BufReader::new(file_handle);

    serde_json::from_reader(reader).map_err(|e| {
        AnonymousVideoValidateError::InvalidFileContent {
            path: file.clone(),
            msg: e.to_string(),
        }
    })
}
