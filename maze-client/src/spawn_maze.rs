
use bevy::prelude::*;

use crate::{Maze, Player, PlayerCamera, PlayerLight, PlayerMinimap, PlayersList};


/// set up the 3D maze
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    maze: Res<Maze>,
    mut player_list: ResMut<PlayersList>,
    asset_server: Res<AssetServer>,
) {  

    // load a texture and retrieve its aspect ratio
    let texture_handle: Handle<Image> = asset_server.load("face.png");

    let striped_maze = maze.raw_grid.strip_prefix("maze_setup ").expect("Parsing maze went wrong");
    let str_rows : Vec<&str>= striped_maze.split(" ").collect();

    let height = str_rows.len() as isize;
    let width = str_rows[0].len() as isize;

    //UI body
    let ui_body = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        ..default()
    };

    let minimap_container = NodeBundle {
        style: Style {
            width: Val::Px(width as f32 * 10.),
            height: Val::Px((height as f32- 1.) * 10.),
            // border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        background_color: Color::srgb(0.65, 0.65, 0.65).into(),
        ..default()
    };

    let body_entity = commands.spawn(ui_body).id();
    let minimap_entity = commands.spawn(minimap_container).id();

    commands.entity(body_entity).push_children(&[minimap_entity]);


    // Warning : X and Y here are 2D coordinates.
    for (u_y, row) in str_rows.iter().enumerate() {
        let y = u_y as isize;
        for (u_x, c) in row.chars().enumerate() {
            let x = u_x as isize;
            match c {
                '#' => {
                    spawn_wall(x, y, &mut commands, &mut meshes, &mut materials);
                    let ui_wall = NodeBundle {
                        style: Style {
                            width: Val::Px(10.0),
                            height: Val::Px(10.0),
                            position_type: PositionType::Absolute,
                            left: Val::Px(x as f32 * 10.),
                            top: Val::Px(y as f32 * 10.),
                            ..default()
                        },
                        background_color: Color::srgb(0.4, 0.4, 1.).into(),
                        ..default()
                    };
                    let wall_entity = commands.spawn(ui_wall).id();
                    commands.entity(minimap_entity).push_children(&[wall_entity]);
    
                },
                '.' => {},
                _ => {
                    spawn_player(x, y, c, "", &mut commands, &mut meshes, &mut materials, &mut player_list, texture_handle.clone());
                },
                
            }

            // spawn les bords de terrain
             if y == 0 {
                 spawn_wall(x, y-1, &mut commands, &mut meshes, &mut materials);
             }
            if x == 0 {
                spawn_wall(x-1, y, &mut commands, &mut meshes, &mut materials);
            }
            if y == height-2 { // -2 parce qu'il y a surement une row vide à la fin du maze
                spawn_wall(x, y+1, &mut commands, &mut meshes, &mut materials);
            }
            if x == width-1 {
                spawn_wall(x+1, y, &mut commands, &mut meshes, &mut materials);
            }
        }
    }


    // circular floor
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(100.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });

    let ui_player = NodeBundle {
        style: Style {
            width: Val::Px(10.0),
            height: Val::Px(10.0),
            position_type: PositionType::Absolute,
            left: Val::Px(0.),
            top: Val::Px(0.),
            ..default()
        },
        background_color: Color::srgb(1.0, 0., 0.).into(),
        ..default()
    };
    commands.spawn(ui_player).insert(PlayerMinimap);

    // player light
    commands.spawn((
        PlayerLight,
        PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    }));
    // camera
    commands.spawn((PlayerCamera,
        Camera3dBundle {
            projection: PerspectiveProjection {
                // We must specify the FOV in radians.
                // Rust can convert degrees to radians for us.
                fov: 90.0_f32.to_radians(),
                ..default()
            }.into(),
        transform: Transform::from_xyz(width as f32, 100.0, height as f32).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }));

    println!("setup complete");
}

fn spawn_wall(
    x: isize, 
    y: isize, 
    commands: &mut Commands, 
    meshes: &mut ResMut<Assets<Mesh>>, 
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::srgb_u8(0, 0, 255)),
        transform: Transform::from_xyz(x as f32, 0.5, y as f32),
        ..default()
    });
}

pub fn spawn_player(
    x: isize, 
    y: isize,
    c: char,
    name: &str,
    commands: &mut Commands, 
    meshes: &mut ResMut<Assets<Mesh>>, 
    materials: &mut ResMut<Assets<StandardMaterial>>,
    list: &mut ResMut<PlayersList>,
    texture_handle: Handle<Image>,
) {

    let new_player = { 
        Player {
            name: name.to_string(),
            number: c,
            direction:"U".to_string(),
            position_x: x,
            position_y: y,
            score: 0,
        }
    };

    let red_material_handle = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        base_color_texture: Some(texture_handle),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    commands.spawn((
        new_player.clone(),
        PbrBundle {
            mesh: meshes.add(Sphere::new(0.15)),
            material: red_material_handle,
            transform: Transform::from_xyz(x as f32, 0.15, y as f32),
            ..default()
        },
    ));

    list.players.push(new_player);
}