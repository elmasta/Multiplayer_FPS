use bevy::prelude::*;
use crate::{ LaserList, NetworkResource, PlayersList, spawn_player};
use std::str;
use std::str::FromStr;


pub fn handle_network_events(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    network: ResMut<NetworkResource>,
    mut player_list: ResMut<PlayersList>,
    mut laser_list: ResMut<LaserList>,
    asset_server: Res<AssetServer>,) {

    // load a texture and retrieve its aspect ratio
    let texture_handle: Handle<Image> = asset_server.load("face.png");

    let mut buf = [0; 2048];
    match network.udp_socket.recv_from(&mut buf) {
        Ok((amt, _src)) => {
            let received_data = str::from_utf8(&buf[..amt]).unwrap();
            // Traiter les données reçues

            // update player name, etc)
            if received_data.contains("LASER-"){
                let splited_data : Vec<&str> = received_data.split("LASEREND").collect();
                update_laser_from_string(&mut laser_list, splited_data[0]);

                update_players_from_string(&mut player_list, splited_data[1]);

            } else
            if received_data.contains("player_move"){
                update_players_from_string(&mut player_list, received_data)
            } else 
            if received_data.contains("JOINED-") {
                let data = received_data.strip_prefix("JOINED-").unwrap();
                let parts: Vec<&str> = data.split(", ").collect();
                let name = parts[0].strip_prefix("Name: ").unwrap().to_string();
                let number = parts[1].strip_prefix("Number: ").unwrap().chars().next().unwrap();
                let position_x = isize::from_str(parts[2].strip_prefix("PositionX: ").unwrap()).unwrap();
                let position_y = isize::from_str(parts[3].strip_prefix("PositionY: ").unwrap()).unwrap();
                spawn_player(position_x, position_y, number, &name, &mut commands, &mut meshes, &mut materials, &mut player_list, texture_handle.clone());
            }
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            // Pas de données disponibles, c'est normal dans un cas non-bloquant.
            // Pas d'action nécessaire, simplement sortir de la fonction.
        }
        Err(e) => {
            println!("Error receiving data: {:?}", e);
        }
    }
}

pub fn update_players_from_string(players_list: &mut PlayersList, str: &str) {
    let data = str.strip_prefix("player_move ").unwrap();
    for line in data.lines() {
        // Parse each line according to the expected format
        let parts: Vec<&str> = line.split(", ").collect();
        if parts.len() != 6 {
            continue;
        }

        let name = parts[0].strip_prefix("Name: ").unwrap().to_string();
        let number = parts[1].strip_prefix("Number: ").unwrap().chars().next().unwrap();
        let direction = parts[2].strip_prefix("Direction: ").unwrap().to_string();
        let position_x = isize::from_str(parts[3].strip_prefix("PositionX: ").unwrap()).unwrap();
        let position_y = isize::from_str(parts[4].strip_prefix("PositionY: ").unwrap()).unwrap();
        let points = usize::from_str(parts[5].strip_prefix("Points: ").unwrap()).unwrap();

        // Try to find the player in the list
        if let Some(player) = players_list.players.iter_mut().find(|p| p.number == number) {
            // Update existing player
            player.name = name;
            player.direction = direction;
            player.position_x = position_x;
            player.position_y = position_y;
            player.score = points;
        }
    }
}

pub fn update_laser_from_string(laser_list: &mut LaserList, str: &str) {
    let striped_str = str.strip_prefix("LASER-").unwrap();
    let parts: Vec<&str> = striped_str.split("| ").collect();
    
    // On extrait la direction
    let direction = parts[0].strip_prefix("Direction: ").unwrap().to_string();
    
    let coordinates_str = parts[1].strip_prefix("Coordinates: ").expect("Invalid format");
    // On supprime les crochets extérieurs uniquement
    let coordinates_short = coordinates_str.trim_start_matches('[').trim_end_matches(']');

    // On split sur "], [" pour séparer chaque paire de coordonnées
    for coord in coordinates_short.split("], [") {
        let values: Vec<&str> = coord.split(',').collect();

        // Vérification s'il y a bien deux valeurs pour x et y
        if values.len() == 2 {
            // On parse les valeurs x et y
            let x: isize = values[0].trim().parse().expect("Invalid number");
            let y: isize = values[1].trim().parse().expect("Invalid number");

            // On ajoute le laser avec la direction (qui doit être clonée pour chaque itération)
            laser_list.0.push((x, y, direction.clone()));
        } else {
            println!("Invalid coordinate pair: {}", coord);
        }
    }
}