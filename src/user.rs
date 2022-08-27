use serde::{Serialize, Deserialize};
use std::collections::BTreeMap as Map;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    /// User api path
    pub api_path: String,
    /// User profile photo
    pub avatar: Map<String, AvatarImage>,
    /// User header image
    pub header_image_url: String,
    /// User role human readable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_readable_role_for_display: Option<String>,
    /// User id.
    pub id: u32,
    /// User iq.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iq: Option<u32>,
    /// Username.
    pub login: String,
    /// User name.
    pub name: String,
    /// User role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_for_display: Option<String>,
    /// User page url.
    pub url: String,
    /// User permissions and interactions.
    pub current_user_metadata: UserMetadata,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AvatarImage {
    /// Image url.
    pub url: String,
    /// Width and height.
    pub bounding_box: Map<String, u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserMetadata {
    /// Permissions you have.
    pub permissions: Vec<String>,
    /// Permissions you don't have.
    pub excluded_permissions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions: Option<Interactions>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Interactions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cosign: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pyong: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vote: Option<u32>,
}
