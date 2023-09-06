use serde::{Deserialize, Serialize};
use crate::channel::Channel;
use crate::hidden::PlaylistItem;
use crate::universal::Playlist;
use crate::video::Video;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommonImage {
    pub url: String,
    pub width: u32,
    pub height: u32,
}

/// Shared thumbnail object as specified in https://docs.invidious.io/api/common_types/
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CommonThumbnail {
    #[serde(default)]
    pub quality: String,
    pub url: String,
    pub width: u32,
    pub height: u32,
}

// https://docs.invidious.io/api/common_types/#videoobject

/// Shared image object as specified in https://docs.invidious.io/api/common_types/
/// Shared video object as specified in https://docs.invidious.io/api/common_types/
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CommonVideo {
    pub title: String,
    #[serde(rename = "videoId")]
    pub id: String,

    pub author: String,
    #[serde(rename = "authorId")]
    pub author_id: String,
    #[serde(rename = "authorUrl")]
    pub author_url: String,
    // #[serde(rename = "authorVerified")]
    // #[serde(default)]
    // pub author_verified: bool,
    #[serde(rename = "videoThumbnails")]
    pub thumbnails: Vec<CommonThumbnail>,

    pub description: String,
    #[serde(rename = "descriptionHtml")]
    pub description_html: String,

    #[serde(rename = "viewCount")]
    pub views: u64,
    // #[serde(rename = "viewCountText")]
    // pub views_text: String,
    #[serde(rename = "lengthSeconds")]
    pub length: u32,

    pub published: u64,
    #[serde(rename = "publishedText")]
    pub published_text: String,

    #[serde(rename = "premiereTimestamp")]
    #[serde(default)]
    pub premiere_timestamp: u64,

    #[serde(rename = "liveNow")]
    pub live: bool,
    pub premium: bool,
    #[serde(rename = "isUpcoming")]
    pub upcoming: bool,
}

/// Shared channel object as specified in https://docs.invidious.io/api/common_types/
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CommonChannel {
    #[serde(rename = "author")]
    pub name: String,
    #[serde(rename = "authorId")]
    pub id: String,
    #[serde(rename = "authorUrl")]
    pub url: String,
    #[serde(rename = "authorVerified")]
    #[serde(default)]
    pub verified: bool,
    #[serde(rename = "authorThumbnails")]
    pub thumbnails: Vec<CommonImage>,
    #[serde(rename = "autoGenerated")]
    #[serde(default)]
    pub auto_generated: bool,
    #[serde(rename = "subCount")]
    pub subscribers: u32,
    #[serde(rename = "videoCount")]
    pub video_count: u32,
    pub description: String,
    #[serde(rename = "descriptionHtml")]
    pub description_html: String,
}

/// Shared playlist object as specified in https://docs.invidious.io/api/common_types/
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CommonPlaylist {
    title: String,
    #[serde(rename = "playlistId")]
    id: String,
    #[serde(rename = "playlistThumbnail")]
    #[serde(default)]
    thumbnail: String,

    author: String,
    #[serde(rename = "authorId")]
    author_id: String,
    // #[serde(rename = "authorUrl")]
    // author_url: String,
    #[serde(rename = "authorVerified")]
    #[serde(default)]
    author_verified: bool,

    #[serde(rename = "videoCount")]
    video_count: u32,
    videos: Vec<CommonPlaylistVideo>,
}

/// Playlist video struct used in CommonPlaylist
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommonPlaylistVideo {
    title: String,
    #[serde(rename = "videoId")]
    id: String,
    #[serde(rename = "lengthSeconds")]
    length: u32,
    #[serde(rename = "videoThumbnails")]
    thumbnails: Vec<CommonThumbnail>,
}

impl From<Video> for CommonVideo {
    fn from(value: Video) -> Self {
        Self {
            title: value.title,
            id: value.id,
            author: value.author,
            author_id: value.author_id,
            author_url: value.author_url,
            thumbnails: value.thumbnails,
            description: value.description,
            description_html: value.description_html,
            views: value.views,
            length: value.length,
            published: value.published,
            published_text: value.published_text,
            premiere_timestamp: value.premiere_timestamp,
            live: value.live,
            premium: value.premium,
            upcoming: value.upcoming,
        }
    }
}

impl From<PlaylistItem> for CommonPlaylistVideo {
    fn from(value: PlaylistItem) -> Self {
        Self {
            title: value.title,
            id: value.id,
            length: value.length,
            thumbnails: value.thumbnails,
        }
    }
}

impl From<Playlist> for CommonPlaylist {
    fn from(value: Playlist) -> Self {
        Self {
            title: value.title,
            id: value.id,
            thumbnail: value.thumbnail,
            author: value.author,
            author_id: value.author_id,
            video_count: value.video_count,
            videos: value
                .videos
                .into_iter()
                .map(CommonPlaylistVideo::from)
                .collect(),
            ..Default::default()
        }
    }
}

impl From<Channel> for CommonChannel {
    fn from(value: Channel) -> Self {
        Self {
            name: value.name,
            id: value.id,
            url: value.url,
            thumbnails: value.thumbnails,
            auto_generated: value.auto_generated,
            subscribers: value.subscribers,
            description: value.description,
            description_html: value.description_html,
            ..Default::default()
        }
    }
}
