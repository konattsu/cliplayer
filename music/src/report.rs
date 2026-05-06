pub(crate) fn anonymous_videos_to_markdown(
    videos: &crate::model::AnonymousVideos,
) -> String {
    let mut markdown = String::new();

    for video in videos.sorted_videos() {
        markdown.push_str(&anonymous_video_to_markdown(video));
        markdown.push_str("\n\n");
    }

    markdown
}

fn anonymous_video_to_markdown(video: &crate::model::AnonymousVideo) -> String {
    let video_id = video.get_video_id();
    let uploader_name = video
        .local_info()
        .get_uploader_name()
        .map(ToString::to_string)
        .unwrap_or_else(|| "(None)".to_string());
    let video_tags = video.local_info().get_video_tags().to_vec().join(", ");

    format!(
        r#"
<details>
<summary>{video_id} | Clips: {} | {video_tags}</summary>

- [Watch on YouTube](https://youtu.be/{video_id})
- Uploader Name: {uploader_name}

Clips list:

{}
</details>
"#,
        video.clips().len(),
        anonymous_clips_to_markdown(video.clips()),
    )
}

fn anonymous_clips_to_markdown(clips: &[crate::model::AnonymousClip]) -> String {
    let mut markdown =
        "| # | Song Title | Clip Range | Artists | Other |\n|:---|:---|:---|:---|:---|\n"
            .to_string();

    for (index, clip) in clips.iter().enumerate() {
        let clip_range = format!(
            "{} - {}",
            clip.get_start_time().to_short_str(),
            clip.get_end_time().to_short_str()
        );
        let artists = clip.get_liver_ids().get_artists_ja_name().join("<br />");
        let other_info = clip_markdown_other_info(clip);

        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            index + 1,
            clip.get_song_title(),
            clip_range,
            artists,
            other_info
        ));
    }

    markdown
}

fn clip_markdown_other_info(clip: &crate::model::AnonymousClip) -> String {
    let mut parts = Vec::new();

    if clip.get_start_time().as_secs() + 390 < clip.get_end_time().as_secs() {
        parts.push("clip range too long?".to_string());
    }
    if let Some(external_artists_name) = clip.get_external_artists_name() {
        parts.push(format!(
            "external artists: {}",
            external_artists_name.to_vec().join(", ")
        ));
    }
    if let Some(clipped_video_id) = clip.get_clipped_video_id() {
        parts.push(format!(
            "clipped video exists [here](https://youtu.be/{clipped_video_id})",
        ));
    }
    parts.join("<br />")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clip_markdown_other_info() {
        let clip = crate::model::AnonymousClip::self_a_3();
        let other_info = clip_markdown_other_info(&clip);
        assert!(other_info.contains("external artists"));

        let clip = crate::model::AnonymousClip::self_a_2();
        let other_info = clip_markdown_other_info(&clip);
        assert!(other_info.contains("clipped video exists"));

        let clip = crate::model::AnonymousClip::self_b_1();
        let other_info = clip_markdown_other_info(&clip);
        assert!(other_info.contains("external artists"));
        assert!(other_info.contains("clipped video exists"));
    }

    #[test]
    fn test_anonymous_clips_to_markdown() {
        let clips = vec![
            crate::model::AnonymousClip::self_a_1(),
            crate::model::AnonymousClip::self_a_2(),
        ];

        let markdown = anonymous_clips_to_markdown(&clips);

        assert!(markdown.contains("| # | Song Title | Clip Range | Artists | Other |"));
        assert!(markdown.contains("Test Song A1"));
        assert!(markdown.contains("Test Song A2"));
    }
}
