use clap::builder::Str;

pub fn apply_new(
    files: &[crate::model::AnonymousVideo],
    api_key: crate::fetcher::YouTubeApiKey,
    music_root: &crate::music_file::MusicRoot,
    min_path: &crate::util::FilePath,
) {
    let files_video =
        match crate::music_file::get_videos_list_from_music_root(music_root) {
            Ok(f) => f,
            Err(e) => {
                e.display_prettier();
                return;
            }
        };

    //
}

pub fn apply_update() {
    //
}

pub fn apply_sync() {
    //
}

async fn foo(
    anonymity: &mut [crate::model::AnonymousVideo],
    api_key: crate::fetcher::YouTubeApiKey,
) -> Result<(), String> {
    let video_ids: Vec<crate::model::VideoId> = anonymity
        .iter()
        .map(|a| a.get_video_brief().get_video_id())
        .cloned()
        .collect();
    let res = crate::fetcher::YouTubeApi::new(api_key)
        .run(video_ids)
        .await
        .map_err(|e| format!("{e}\n"))?;
    let non_exist_ids = res.get_non_exists_video_ids();
    if !non_exist_ids.is_empty() {
        return Err(format!(
            "Non-exist video id(s) are specified: {}",
            non_exist_ids
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }

    // TODO ここから、ここから
    // `video_detail_fetch`見直し中

    todo!()
}
