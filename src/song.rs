use serde::{Serialize, Deserialize};

use crate::album::Album;
use crate::annotation::Referent;
use crate::user::{User, UserMetadata};
use crate::{Body, Date};

#[derive(Serialize, Deserialize, Debug)]
pub struct Song {
    /// Number of annotations on this song.
    pub annotation_count: u32,
    /// Path of the API.
    pub api_path: String,
    /// Id of the song in apple music.
    /// > Only in `get_song`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_music_id: Option<String>,
    /// URL of the song in apple music.
    /// > Only in `get_song`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_music_player_url: Option<String>,
    /// Number of comments on this song.
    /// > Only in `get_song` with `user-core` level token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_count: Option<u32>,
    /// Custom header image.
    /// > Only in `get_song` with `user-core` level token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_header_image_url: Option<String>,
    /// Custom song art image.
    /// > Only in `get_song` with `user-core` level token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_song_art_image_url: Option<String>,
    /// Description of the music.
    /// > Only in `get_song`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Body>,
    /// Preview of the description.
    /// > Only in `get_song` with `user-core` level token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_preview: Option<String>,
    /// HTML to embed the content in a web page.
    /// > Only in `get_song`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_content: Option<String>,
    /// Facebook share message without url.
    /// > Only in `get_song` with `user-core` level token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facebook_share_message_without_url: Option<String>,
    /// If has a video linked with this song.
    /// > Only in `get_song`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub featured_video: Option<bool>,
    /// Full song title is: "`name` by `author`".
    pub full_title: String,
    /// If has instagram reels annotations.
    /// > Only in `get_song` with `user-core` level token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_instagram_reel_annotations: Option<bool>,
    /// Header image with a smaller size.
    pub header_image_thumbnail_url: String,
    /// Header image.
    pub header_image_url: String,
    /// If this song is hidden?
    /// > Only in `get_song` with `user-core` level token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    /// Id of the song.
    pub id: u32,
    /// If is instrumental song.
    /// > Only in `get_song` with `user-core` level token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrumental: Option<bool>,
    /// If this song is a music lol
    /// > Only in `get_song` with `user-core` level token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_music: Option<bool>,
    /// Literally lyrics.
    /// > Only in `get_song` with `user-core` level token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lyrics: Option<Body>,
    /// Id of the user who requested the lyrics.
    pub lyrics_owner_id: u32,
    /// Lyrics state.
    pub lyrics_state: String,
    /// Lyrics updated timestamp.
    /// > Only in `get_song` with `user-core` level token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lyrics_updated_at: Option<u64>,
    /// Path where is in genius.com website.
    pub path: String,
    /// Pending lyrics edits count.
    /// > Only in `get_song` with `user-core` level token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_lyrics_edits_count: Option<u32>,
    /// If is published?
    /// > Only in `get_song` with `user-core` level token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>,
    /// > Only in `get_song` with `user-core` level token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pusher_channel: Option<String>,
    /// Release date of the album in struct format [`Date`].
    /// > Only in `get_song` with `user-core` level token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date_components: Option<Date>,
    /// Number of pyongs in this song.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pyongs_count: Option<u32>,
    /// The location of the recording.
    /// > Only in `get_song`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_location: Option<String>,
    /// Release date of this song in ISO 8601 date format.
    /// > Only in `get_song`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<String>,
    /// Release date in `{Month name} {day}, {year}` format.
    /// > Only in `get_song`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date_for_display: Option<String>,
    /// Share url.
    /// > Only in `get_song` with `user-core` level token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_url: Option<String>,
    /// Song art image with a smaller size.
    pub song_art_image_thumbnail_url: String,
    /// Song art image.
    pub song_art_image_url: String,
    /// Soundcloud url.
    /// > Only in `get_song` with `user-core` level token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soundcloud_url: Option<String>,
    /// Spotify uuid.
    /// > Only in `get_song` with `user-core` level token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spotify_uuid: Option<String>,
    /// Information about contribution and views.
    pub stats: SongStatus,
    /// Title of the song.
    pub title: String,
    /// Title but with the featured artist if it exists.
    pub title_with_featured: String,
    /// > Only in `get_song` with `user-core` level token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_paths: Option<TrackingPaths>,
    /// Twitter share message
    /// > Only in `get_song` with `user-core` level token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter_share_message: Option<String>,
    /// Twitter share message without url.
    /// > Only in `get_song` with `user-core` level token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter_share_message_without_url: Option<String>,
    /// Updated by a human timestamp.
    /// > Only in `get_song` with `user-core` level token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by_human_at: Option<u64>,
    /// Url of the song page.
    pub url: String,
    /// > Only in `get_song` with `user-core` level token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewable_by_roles: Option<Vec<String>>,
    /// Youtube start.
    /// > Only in `get_song` with `user-core` level token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub youtube_start: Option<String>,
    /// Youtube url.
    /// > Only in `get_song` with `user-core` level token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub youtube_url: Option<String>,
    /// User permissions and interactions.
    /// > Only in `get_song`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_user_metadata: Option<UserMetadata>,
    /// Author of the song.
    pub primary_artist: Artist,
    /// Album of the song.
    /// > Only in `get_song`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album: Option<Album>,
    /// All albums that this song appears, I don't know why but yes.
    /// > Only in `get_song` with `user-core` level token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub albums: Option<Vec<Album>>,
    /// People who worked in the music.
    /// > Only in `get_song`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_performances: Option<Vec<SongPerformance>>,
    /// Description annotation.
    /// > Only in `get_song`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_annotation: Option<Referent>,
    /// Artists who featured in the song.
    /// > Only in `get_song`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub featured_artists: Option<Vec<Artist>>,
    /// Music platforms that host this song and its url.
    /// > Only in `get_song`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Vec<SongMedia>>,
    /// Artists who produced this song.
    /// > Only in `get_song`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub producer_artists: Option<Vec<Artist>>,
    /// Songs that somehow relate to this.
    /// > Only in `get_song`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub song_relationships: Option<Vec<SongRelationship>>,
    /// All Verified Annotation contributors.
    /// > Only in `get_song`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_annotations_by: Option<Vec<User>>,
    /// All verified contributors
    /// > Only in `get_song`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_contributors: Option<Vec<SongContributor>>,
    /// All Verified lyrics contributors.
    /// > Only in `get_song`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_lyrics_by: Option<Vec<User>>,
    /// Composers
    /// > Only in `get_song`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub writer_artists: Option<Vec<Artist>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrackingPaths {
    pub aggregate: String,
    pub concurrent: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SongContributor {
    pub contributions: Vec<String>,
    pub artist: Artist,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SongRelationship {
    /// The type of relationship can be `samples`, `sampled_in`, `interpolates`, `interpolated_by`, `cover_of`, `covered_by`, `remix_of`, `remixed_by`, `live_version_of` and `performed_live_as`.
    pub relationship_type: String,
    /// Songs with this relationship type.
    pub songs: Vec<Option<Song>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SongPerformance {
    pub label: String,
    pub artists: Vec<Artist>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SongMedia {
    /// Spotify path of the song with `:` instead `/`, weird.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_uri: Option<String>,
    /// Soundcloud username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution: Option<String>,
    /// The song host provider `youtube`, `soundcloud` or `spotify`.
    pub provider: String,
    /// Youtube position of the video that starts the music.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<u32>,
    /// Media type `video` or `audio`
    #[serde(rename = "type")]
    pub media_type: String,
    /// The url of the song in the media host.
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SongStatus {
    /// Number of annotations accepted on this song.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_annotations: Option<u32>,
    /// Number of contributors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributors: Option<u32>,
    /// Number of users who have earned iq.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iq_earners: Option<u32>,
    /// Number of transcribers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcribers: Option<u32>,
    /// Number of verified annotations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_annotations: Option<u32>,
    /// Number of unreviewed annotations.
    pub unreviewed_annotations: u32,
    /// If it's hot be careful with your hands.
    pub hot: bool,
    /// Number of page views
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pageviews: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Artist {
    /// Path of the API.
    pub api_path: String,
    /// Artist header image.
    pub header_image_url: String,
    /// Artist id.
    pub id: u32,
    /// Artist image.
    pub image_url: String,
    /// First letter of the artist name.
    /// > Only with `user-core` level token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_character: Option<char>,
    /// Is this artist a meme?
    pub is_meme_verified: bool,
    /// If this artist is verified.
    pub is_verified: bool,
    /// Name of the artist.
    pub name: String,
    /// > Only with `user-core` level token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// Url of the artist page.
    pub url: String,
    /// How much iq this artist has.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iq: Option<u32>,
}
