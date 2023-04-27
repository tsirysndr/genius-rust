use serde::{Serialize, Deserialize};
use std::collections::BTreeMap as Map;

use crate::user::{User, UserMetadata};
use crate::Body;

#[derive(Serialize, Deserialize, Debug)]
pub struct Referent {
    pub _type: String,
    pub annotator_id: u32,
    pub annotator_login: String,
    pub api_path: String,
    pub classification: String,
    pub fragment: String,
    pub id: u32,
    /// > Only with `user-core` level token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ios_app_url: Option<String>,
    pub is_description: bool,
    /// > Only with `user-core` level token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_image: Option<bool>,
    pub path: String,
    pub range: Map<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub song_id: Option<u32>,
    pub url: String,
    /// > Only with `user-core` level token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_user_metadata: Option<UserMetadata>,
    pub annotations: Vec<Annotation>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Annotation {
    pub api_path: String,
    pub body: Body,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_count: Option<u32>,
    pub community: bool,
    pub has_voters: bool,
    pub id: u32,
    pub pinned: bool,
    pub share_url: String,
    pub state: String,
    pub url: String,
    pub verified: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub votes_total: Option<i32>,
    pub current_user_metadata: UserMetadata,
    pub authors: Vec<AnnotationAuthor>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnnotationAuthor {
    pub attribution: f32,
    pub user: User,
}
