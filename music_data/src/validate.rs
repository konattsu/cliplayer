#[derive(Debug, Clone)]
pub struct FileVideoId {
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
) -> Result<Vec<FileVideoId>, String> {
    let videos = crate::music_file::MusicRootContent::load(music_root)
        .map_err(|e| e.to_pretty_string())?
        .into_flattened_files();

    let video_ids_set = videos.into_vec();
    let video_ids_set: std::collections::HashMap<_, _> = video_ids_set
        .iter()
        .map(|v| (v.get_video_id(), v.get_published_at()))
        .collect();

    let duplicates: Vec<FileVideoId> = video_ids
        .iter()
        .filter_map(|id| {
            video_ids_set.get(id).copied().map(|p| FileVideoId {
                video_id: id.clone(),
                published_at: p.clone(),
            })
        })
        .collect();

    Ok(duplicates)
}

/// 新規の楽曲情報ファイルの形式を検証
pub fn validate_new_input(files: &[crate::util::FilePath]) -> Result<(), String> {
    for file in files {
        //
    }
    // applyと一部処理共通(anonymous作成)なのでそっち先作る
    todo!()
}

// TODO 通常と同じパースを適用している. 専用で処理するか要件等
/// 既存の楽曲情報に対する変更を検証
pub fn validate_update_input(
    music_root: &crate::music_file::MusicRoot,
) -> Result<(), String> {
    let _video = crate::music_file::MusicRootContent::load(music_root)
        .map_err(|e| e.to_pretty_string())?;

    Ok(())
}

fn deserialize_from_file(
    file: &crate::util::FilePath,
) -> Result<crate::model::AnonymousVideo, String> {
    // ref: src/music_file/fs.rs
    todo!()
}
