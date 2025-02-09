use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FirstContactData {
    pub bot_name: String,
    pub lobby_id: String,
    pub map_name: Option<String>,
    pub team_name: String,
}
