use bevy::{ecs::system::SystemParam, prelude::*, utils::HashMap};
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{
        config::{ConfigureLoadingState, LoadingStateConfig},
        LoadingStateAppExt,
    },
    mapped::AssetFileStem,
};
use bevy_common_assets::ron::RonAssetPlugin;
use serde::{Deserialize, Serialize};

use crate::main_state::MyMainState;

pub struct MyMapPlugin;

impl Plugin for MyMapPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TeamConfig>()
            .register_type::<MapConfig>()
            .register_type::<MapDefinition>()
            .register_type::<TileDefinition>()
            .register_type::<LayerDefinition>()
            .register_type::<LayerType>()
            .register_type::<MarkerDefinition>()
            .register_type::<MarkerType>()
            .configure_loading_state(
                LoadingStateConfig::new(MyMainState::SettingUp).load_collection::<AllMapsAsset>(),
            )
            .add_plugins(RonAssetPlugin::<MapConfig>::new(&["map.ron"]));
    }
}

#[derive(Debug, Default, Clone, AssetCollection, Resource)]
pub struct AllMapsAsset {
    #[asset(path = "maps", collection(mapped, typed))]
    pub maps: HashMap<AssetFileStem, Handle<MapConfig>>,
}

#[derive(Debug, Default, Reflect, Clone, Asset, Deserialize, PartialEq)]
pub struct MapConfig {
    pub teams: HashMap<String, TeamConfig>,
    pub map: MapDefinition,
}

impl MapConfig {
    pub fn insert_player_into_team(&mut self, team_name: &str, player: Entity) -> bool {
        match self.teams.get_mut(team_name) {
            Some(team) => team.players.push(player),
            None => return false,
        }
        true
    }

    pub fn remove_player_from_team(&mut self, player: Entity) {
        for team in self.teams.values_mut() {
            team.players.retain(|&x| x != player);
        }
    }

    pub fn get_team(&self, team_name: &str) -> Option<&TeamConfig> {
        self.teams.get(team_name)
    }

    pub fn get_team_of_player(&self, player: Entity) -> Option<(String, &TeamConfig)> {
        for (team_name, team) in self.teams.iter() {
            if team.players.contains(&player) {
                return Some((team_name.clone(), team));
            }
        }
        None
    }
}

#[derive(Debug, Clone, Reflect, Default, Deserialize, PartialEq)]
pub struct TeamConfig {
    pub color: Color,
    pub max_players: usize,

    #[serde(skip)]
    pub players: Vec<Entity>,
}

#[derive(Debug, Clone, Reflect, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MapDefinition {
    pub width: usize,
    pub height: usize,

    /// A 2D array of height values—row by row.
    /// For a grid of size `height x width`, we'll have `height` sub-vectors,
    /// each containing `width` floats.
    pub tiles: Vec<Vec<f32>>,

    pub layers: Vec<LayerDefinition>,

    pub markers: Vec<MarkerDefinition>,
}

#[derive(Debug, Clone, Reflect, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TileDefinition {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Reflect, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LayerDefinition {
    pub kind: LayerType,
    /// A cost modifier for pathfinding or movement / Maybe also use this to slow down?
    pub cost_modifier: f32,
    // TODO: Add a hide modifier?
    /// A list of (x, y) coordinates for cells that belong to this layer
    pub tiles: Vec<TileDefinition>,
}

#[derive(Debug, Clone, Reflect, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LayerType {
    #[default]
    Forest,
}

#[derive(Debug, Clone, Reflect, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MarkerDefinition {
    pub tile: TileDefinition,
    /// The group this marker belongs to. for example a team
    pub group: String,

    pub kind: MarkerType,
}

#[derive(Debug, Clone, Reflect, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase", tag = "type")]
pub enum MarkerType {
    #[serde(rename_all = "camelCase")]
    Spawn {
        spawn_number: usize,
    },
    Flag,
}

impl Default for MarkerType {
    fn default() -> Self {
        MarkerType::Spawn { spawn_number: 0 }
    }
}

#[derive(SystemParam)]
pub struct MapConfigSystemParam<'w> {
    maps_asset: Res<'w, AllMapsAsset>,
    map_configs: Res<'w, Assets<MapConfig>>,
}

impl<'w> MapConfigSystemParam<'w> {
    pub fn get_map_config_from_name(&self, map_name: &str) -> Option<&MapConfig> {
        let map_name = if map_name.ends_with(".map") {
            map_name.to_string()
        } else {
            format!("{}.map", map_name)
        };

        self.maps_asset
            .maps
            .iter()
            .find(|(stem, _)| stem.as_ref() == map_name)
            .and_then(|(_, handle)| self.map_configs.get(handle))
    }

    pub fn get_map_config_from_asset_id(&self, asset_id: AssetId<MapConfig>) -> Option<&MapConfig> {
        self.map_configs.get(asset_id)
    }

    pub fn list_map_names(&self) -> Vec<String> {
        self.maps_asset
            .maps
            .iter()
            .map(|(stem, _)| stem.as_ref().to_string().replace(".map", ""))
            .collect()
    }
}
