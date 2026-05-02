#![cfg(feature = "test-helpers")]

fn sample_tag_ids() -> (String, String) {
    let tag_ids = tagctl::model::LOADED_VIDEO_TAG_DATA
        .sorted_ids()
        .into_iter()
        .map(str::to_owned)
        .collect::<Vec<_>>();

    assert!(
        tag_ids.len() >= 2,
        "test tag data must contain at least two tags"
    );

    (tag_ids[0].clone(), tag_ids[1].clone())
}

fn sample_loaded_data() -> crate::build::load::LoadedData {
    let artist_id_1 = artistctl::model::LiverId::self_1().as_str().to_owned();
    let artist_id_2 = artistctl::model::LiverId::self_2().as_str().to_owned();
    let channel_id_1 = cmn_rs::yt::ChannelId::test_id_1().to_string();
    let channel_id_2 = cmn_rs::yt::ChannelId::test_id_2().to_string();
    let official_channel_id = cmn_rs::yt::ChannelId::test_id_3().to_string();
    let video_id_1 = musictl::model::VideoId::test_id_1().to_string();
    let video_id_2 = musictl::model::VideoId::test_id_2().to_string();
    let clip_uuid_1 = musictl::model::UuidVer4::self_1().to_string();
    let clip_uuid_2 = musictl::model::UuidVer4::self_2().to_string();
    let (tag_id_1, tag_id_2) = sample_tag_ids();

    crate::build::load::LoadedData {
        artists: vec![
            crate::build::load::LoadedArtist {
                artist_id: artist_id_2.clone(),
                channel_id: channel_id_2.clone(),
            },
            crate::build::load::LoadedArtist {
                artist_id: artist_id_1.clone(),
                channel_id: channel_id_1.clone(),
            },
        ],
        official_channels: vec![crate::build::load::LoadedOfficialChannel {
            channel_id: official_channel_id,
        }],
        tag_ids: vec![tag_id_2.clone(), tag_id_1.clone()],
        clips: vec![
            crate::build::load::LoadedClipRecord {
                clip_uuid: clip_uuid_2,
                video_id: video_id_2,
                published_at: 20,
                channel_id: channel_id_2,
                is_unlisted: true,
                embeddable: false,
                artist_ids: vec![artist_id_2, artist_id_1.clone()],
                tag_ids: vec![tag_id_2.clone(), tag_id_1.clone(), tag_id_1],
            },
            crate::build::load::LoadedClipRecord {
                clip_uuid: clip_uuid_1,
                video_id: video_id_1,
                published_at: 10,
                channel_id: channel_id_1,
                is_unlisted: false,
                embeddable: true,
                artist_ids: vec![artist_id_1],
                tag_ids: vec![tag_id_2],
            },
        ],
    }
}

#[test]
fn test_build_search_index_from_loaded_data() {
    let index = crate::build::assemble::build_search_index_from_loaded_data(
        sample_loaded_data(),
    )
    .unwrap();

    assert_eq!(index.meta.record_count, 2);
    assert_eq!(index.columns.is_unlisteds, vec![false, true]);
    assert_eq!(index.columns.embeddables, vec![true, false]);
    assert_eq!(index.columns.artist_id_lists.get(0), &[0]);
    assert_eq!(index.columns.artist_id_lists.get(1), &[0, 1]);
    assert_eq!(index.columns.tag_id_lists.get(1), &[0, 1]);
    assert_eq!(index.sort_indexes.published_at.doc_ids_asc(), &[0, 1]);
    assert_eq!(index.exact_indexes.is_unlisted_docs[0], vec![0]);
    assert_eq!(index.exact_indexes.is_unlisted_docs[1], vec![1]);
    assert_eq!(index.exact_indexes.embeddable_docs[0], vec![1]);
    assert_eq!(index.exact_indexes.embeddable_docs[1], vec![0]);
}

#[test]
fn test_build_search_index_from_loaded_data_rejects_unknown_channel() {
    let mut data = sample_loaded_data();
    data.clips[0].channel_id = "UCUNKNOWNUNKNOWNUNKNOWN1".to_string();

    let err =
        crate::build::assemble::build_search_index_from_loaded_data(data).unwrap_err();
    assert!(err.to_string().contains("unknown channel_id"));
}
