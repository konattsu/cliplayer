#[derive(Debug, Clone)]
pub struct FileVideoId {
    pub file: crate::util::FilePath,
    pub video_id: crate::model::VideoId,
}

/// 既存の楽曲情報ファイルから, 指定された動画IDが重複しているかどうか確認
///
/// Arguments:
/// - `music_root`: 楽曲情報のルートフォルダ
/// - `video_ids`: 重複を確認したい動画IDのリスト
///
/// Returns:
/// - Ok(a): 重複していた動画idとそれが含まれるファイルたち
/// - Err(e): 楽曲情報のファイルに関する何らかのエラー
pub fn duplicate_video_ids(
    music_root: &crate::music_file::MusicRoot,
    video_ids: &Vec<crate::model::VideoId>,
) -> Result<Vec<FileVideoId>, crate::music_file::ValidateError> {
    let mut duplicates: Vec<FileVideoId> = Vec::new();

    for file_video in crate::music_file::get_videos_list_from_music_root(music_root)? {
        for video_id in video_ids {
            if file_video.videos.has_video_id(video_id) {
                duplicates.push(FileVideoId {
                    file: file_video.file.clone(),
                    video_id: video_id.clone(),
                });
            }
        }
    }
    Ok(duplicates)
}

/// 新規の楽曲情報ファイルの形式を検証
pub fn validate_new_input(
    files: &[crate::util::FilePath],
) -> Result<(), crate::music_file::ValidateError> {
    let mut invalid_files: Vec<crate::music_file::ValidateErrorDeserialize> =
        Vec::new();

    for file in files {
        let res: Result<crate::model::AnonymousVideo, _> =
            crate::music_file::deserialize_from_file(file);

        match res {
            Ok(..) => continue,
            Err(crate::music_file::ValidateError::FileOpenError(inner)) => {
                // 1つでもファイルが開けなかったらすぐにエラーを返す
                return Err(crate::music_file::ValidateError::FileOpenError(inner));
            }
            Err(crate::music_file::ValidateError::DeserializeError(inner)) => {
                invalid_files.extend(inner);
            }
        }
    }

    if invalid_files.is_empty() {
        Ok(())
    } else {
        Err(crate::music_file::ValidateError::DeserializeError(
            invalid_files,
        ))
    }
}

/// 既存の楽曲情報に対する変更を検証
pub fn validate_update_input(
    files: &[crate::util::FilePath],
) -> Result<(), crate::music_file::ValidateError> {
    let mut invalid_files: Vec<crate::music_file::ValidateErrorDeserialize> =
        Vec::new();

    for file in files {
        let res: Result<crate::model::VerifiedVideos, _> =
            crate::music_file::deserialize_from_file(file);

        match res {
            Ok(..) => continue,
            Err(crate::music_file::ValidateError::FileOpenError(inner)) => {
                // 1つでもファイルが開けなかったらすぐにエラーを返す
                return Err(crate::music_file::ValidateError::FileOpenError(inner));
            }
            Err(crate::music_file::ValidateError::DeserializeError(inner)) => {
                invalid_files.extend(inner);
            }
        }
    }

    if invalid_files.is_empty() {
        Ok(())
    } else {
        Err(crate::music_file::ValidateError::DeserializeError(
            invalid_files,
        ))
    }
}

// なんやかんやこのファイル完成気味, まぁapplyの方が100倍大変なんですけどね
