use bevy::prelude::*;

use crate::{LaserList, PlayerMinimap, PlayersList};
use crate::{Player, PlayerLight, PlayerCamera, Laser};


    
pub fn update_player(
    mut player_query: Query<(&mut Transform, &Player)>,
    mut light_query: Query<&mut Transform, (With<PlayerLight>, Without<Player>)>,
    mut camera_query: Query<&mut Transform, (With<PlayerCamera>, Without<PlayerLight>, Without<Player>)>,
    mut ui_minimap_query: Query<&mut Style, With<PlayerMinimap>>,
    players_list: Res<PlayersList>,
) {

    for (mut player_transform, player) in player_query.iter_mut() {
        // Find the corresponding player in players_list using the `number`
        if let Some(player_data) = players_list.players.iter().find(|p| p.number == player.number) {
            // Update the player's transform with the position from players_list
            player_transform.translation.x = player_data.position_x as f32;
            player_transform.translation.z = player_data.position_y as f32;
            let mut p_rotation = Quat::from_rotation_y((0.0_f32).to_radians());
            match player_data.direction.as_str() {
                    "D" => p_rotation = Quat::from_rotation_y((270.0_f32).to_radians()),
                    "U" => p_rotation = Quat::from_rotation_y((90.0_f32).to_radians()),
                    "R" => p_rotation = Quat::from_rotation_y((0.0_f32).to_radians()),
                    "L" => p_rotation = Quat::from_rotation_y((180.0_f32).to_radians()),
                    _ => {
                        println!("Direction error")
                    }   
            }
            player_transform.rotation = p_rotation;

            let mut direction = Vec3::ZERO;
            direction.y = 0.3;
            if players_list.current_player == player_data.name { // player with the camera
                match player_data.direction.as_str(){
                    "D" => direction.z += 1.0,
                    "U" => direction.z -= 1.0,
                    "R" => direction.x += 1.0,
                    "L" => direction.x -= 1.0,
                    _ => {
                        println!("Direction error")
                    }                    
                }

                if 0.0 < direction.length() {
                    let mut camera_transform : Mut<Transform> = camera_query.single_mut();
                    camera_transform.translation = player_transform.translation;
                    camera_transform.translation.y = 0.5;
                    camera_transform.look_at(player_transform.translation + (1.0 * direction.normalize()), Vec3::Y);
                }

                let mut light_transform : Mut<Transform> = light_query.single_mut();
                light_transform.translation = player_transform.translation;  
                light_transform.translation.y = 2.0;

                let mut style = ui_minimap_query.single_mut();
                style.top = Val::Px( player_transform.translation.z * 10.);
                style.left = Val::Px( player_transform.translation.x * 10.);
            }
        }         
    }
}

pub fn update_laser(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(Entity, &mut Laser)>,
    mut laser_list: ResMut<LaserList>,
) { 

    // remove laser after duration end
    for (entity, mut timer) in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.finished() {
            commands.entity(entity).despawn();
            println!("remove laser")
        }
    }
    
    // spawn them from laser list
    for laser in laser_list.0.iter(){
        spawn_laser(&mut commands, &mut meshes, &mut materials, laser.0, laser.1, laser.2.clone())
    }
    // remove laser from laser list
    laser_list.0.clear();
    
}


fn spawn_laser(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position_x: isize,
    position_y: isize,
    direction: String,
) {

    let mut cylinder_rotation= Quat::from_rotation_z((0.0_f32).to_radians()) ;
    match direction.as_str(){
        "U" | "D" => cylinder_rotation = Quat::from_rotation_x((90.0_f32).to_radians()),
        "R" | "L" => cylinder_rotation = Quat::from_rotation_z((90.0_f32).to_radians()),
        _ => {
            println!("unknown laser direction");
        }
        
    }

    println!("laser {}, {}", position_x, position_y);
    commands.spawn((
        Laser(Timer::from_seconds(0.3, TimerMode::Once)),
        PbrBundle {
            mesh: meshes.add(Cylinder::new(0.05, 1.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb_u8(0, 255, 0),
                unlit: false,
                ..default()
            }),
            transform: Transform::from_xyz(position_y as f32, 0.15, position_x as f32)
            .with_rotation(cylinder_rotation),
            ..default()
        }       
    ));
}

