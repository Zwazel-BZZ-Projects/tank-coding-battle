use bevy::{color::palettes::css::WHITE, prelude::*};
use bevy_mod_billboard::BillboardText;
use shared::{
    game::player_handling::TankTransform,
    networking::messages::message_container::GameStartsTrigger,
};

use crate::game_handling::entity_mapping::MyEntityMapping;

pub fn update_player_positions(
    mut transforms: Query<(&mut Transform, &TankTransform), Changed<TankTransform>>,
) {
    for (mut transform, tank_transform) in transforms.iter_mut() {
        transform.translation = tank_transform.position;
        transform.rotation = tank_transform.rotation;
    }
}

pub fn create_player_visualisation(
    trigger: Trigger<GameStartsTrigger>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let game_start = trigger.event();
    let map_definition = &game_start.map_definition;
    let font = asset_server.load("fonts/FiraSans-Regular.ttf");

    for player in game_start.connected_clients.iter() {
        let team_color = game_start
            .team_configs
            .get(&player.client_team)
            .map(|config| Color::from(config.color.clone()))
            .unwrap_or(WHITE.into());

        let player_position = map_definition
            .get_spawn_point_position(&player.client_team, player.assigned_spawn_point)
            .unwrap()
            + Vec3::new(0.0, 0.5, 0.0);

        let entity = commands
            .spawn((
                Name::new(player.client_name.clone()),
                Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                MeshMaterial3d(materials.add(team_color)),
                Transform::from_translation(player_position),
                TankTransform {
                    position: player_position,
                    rotation: Quat::IDENTITY,
                },
            ))
            .with_children(|commands| {
                commands.spawn((
                    BillboardText::new(&player.client_name),
                    TextFont::from_font(font.clone()).with_font_size(60.0),
                    TextColor(Color::WHITE),
                    TextLayout::new_with_justify(JustifyText::Center),
                    Transform::from_translation(Vec3::new(0.0, 1.0, 0.0))
                        .with_scale(Vec3::splat(0.0085)),
                ));
            })
            .id();

        commands.entity(entity).insert(MyEntityMapping {
            server_entity: player.client_id,
            client_entity: entity,
        });
    }
}
