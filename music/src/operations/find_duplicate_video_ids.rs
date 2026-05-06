pub(crate) fn find_duplicate_video_ids(
    library: &crate::music_file::MusicLibrary,
    ids: &[crate::model::VideoId],
) -> crate::model::VideoIds {
    use std::collections::HashSet;

    let library_ids: HashSet<crate::model::VideoId> =
        library.get_video_ids().into_iter().collect();

    ids.iter()
        .filter_map(|id| library_ids.get(id))
        .cloned()
        .collect()
}
