use bevy::prelude::*;

use std::net::UdpSocket;


#[derive(Resource)]
pub struct NetworkResource {
    pub udp_socket: UdpSocket,
    pub host_name: String,
}

#[derive(Resource)]
pub struct Maze {
    pub raw_grid: String,
}

#[derive(Resource)]
pub struct PlayersList {
    pub players: Vec<Player>,
    pub current_player: String,
}
#[derive(Resource)]
pub struct LaserList (pub Vec<(isize, isize, String)>);



#[derive(Component)]
pub struct Laser (pub Timer);

#[derive(Component, Clone)]
pub struct Player{ // Component used to manipulate the 3D representation of the player
    pub name: String,
    pub number: char,
    pub direction: String,
    pub position_x: isize,
    pub position_y: isize,
    pub score: usize,
}

#[derive(Component)]
pub struct PlayerLight;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct PlayerMinimap;
