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
    mut anonymity: Vec<crate::model::AnonymousVideo>,
    api_key: crate::fetcher::YouTubeApiKey,
) -> Result<(), String> {
    let video_ids: Vec<crate::model::VideoId> = anonymity
        .iter()
        .map(|a| a.get_video_id())
        .cloned()
        .collect();
    let res = crate::fetcher::YouTubeApi::new(api_key)
        .run(video_ids)
        .await
        .map_err(|e| format!("{e}\n"))?;

    let briefs = anonymity
        .iter()
        .map(|a| a.get_video_brief())
        .cloned()
        .collect::<Vec<_>>();

    let details = match res.try_into_video_detail(&briefs) {
        Ok(d) => d,
        // 一旦適当
        Err(non_exist_ids) => {
            return Err(format!(
                "Non-exist video id(s) are specified: {}",
                non_exist_ids
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
    };

    let detail_map: std::collections::HashMap<_, _> = details
        .into_iter()
        .map(|d| (d.get_video_id().clone(), d))
        .collect();

    let mut anonymity_detail = Vec::new();
    for an in anonymity {
        if let Some(detail) = detail_map.get(an.get_video_id()) {
            anonymity_detail.push((an, detail.clone()));
        } else {
            // なにか
        }
    }

    let mut verified_videos = Vec::new();

    for (an, detail) in anonymity_detail {
        verified_videos.push(crate::model::VerifiedVideo::from_anonymous_video(
            an, detail,
        ));
    }

    // TODO ここから、ここから
    // `video_detail_fetch`見直し中

    todo!()
}
