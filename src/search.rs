use crate::song::Song;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Hit {
    pub index: String,
    #[serde(rename = "type")]
    pub hit_type: String,
    pub result: Song,
}
