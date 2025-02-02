use serde::{Serialize, Deserialize};

use crate::annotation::Referent;
use crate::song::{Artist, SongPerformance};
use crate::user::UserMetadata;
use crate::Date;

#[derive(Serialize, Deserialize, Debug)]
pub struct Album {
    /// Path of the API.
    pub api_path: String,
    /// Number of comments.
    /// > Only in `get_album`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_count: Option<u32>,
    /// Cover art thumbnail if the album.
    /// > Only with `user-core` level token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_art_thumbnail_url: Option<String>,
    /// Cover art of the album.
    pub cover_art_url: String,
    /// Custom header image.
    /// > Only in `get_album`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_header_image_url: Option<String>,
    /// Full album title is: "`name` by `author`".
    pub full_title: String,
    /// Header image.
    /// > Only in `get_album`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_image_url: Option<String>,
    /// Id of the album.
    pub id: u32,
    /// > Only in `get_album`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_state: Option<String>,
    /// Name of the album.
    pub name: String,
    /// Name with artist in `{album name} (artist: {artist name})`
    /// > Only with `user-core` level token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_with_artist: Option<String>,
    /// Number of pyongs in this album.
    /// > Only in `get_album`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pyongs_count: Option<u32>,
    /// Release date of the album in ISO 8601 date format.
    /// > Only in `get_album`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<String>,
    /// Release date of the album in struct format [`Date`].
    /// > Only in `get_album` or with `user-core` level token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date_components: Option<Date>,
    /// Url of the album page.
    pub url: String,
    /// Current user metadata have all the permissions of the user to vote, edit etc. and some others stuffs.
    /// > Only in `get_album`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_user_metadata: Option<UserMetadata>,
    /// Number of views in the album page.
    /// > Only in `get_album`
    #[serde(rename = "song_pageviews", skip_serializing_if = "Option::is_none")]
    pub album_pageviews: Option<u32>,
    /// Album artist.
    pub artist: Artist,
    /// All cover arts of the album.
    /// > Only in `get_album`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_arts: Option<Vec<CoverArt>>,
    /// Annotations in this album.
    /// > Only in `get_album`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_annotation: Option<Referent>,
    /// All the people who worked on this album.
    /// > Only in `get_album`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub song_performances: Option<Vec<SongPerformance>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoverArt {
    /// If this art have annotations.
    pub annotated: bool,
    /// Path of the API
    pub api_path: String,
    /// Id of cover art.
    pub id: u32,
    /// Image of the art.
    pub image_url: String,
    /// Thumbnail image of the art.
    pub thumbnail_image_url: String,
    /// Page of the art.
    pub url: String,
}
