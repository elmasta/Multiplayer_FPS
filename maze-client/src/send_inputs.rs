use bevy::prelude::*;

use crate::NetworkResource;

pub fn send_player_inputs(
    keys: Res<ButtonInput<KeyCode>>,
    network: ResMut<NetworkResource>,
) {

        if keys.pressed(KeyCode::KeyW) { // up
            network.udp_socket.send_to("move_U".as_bytes(),
                                        network.host_name.clone()).expect("Error on send \"U\"");
        }

        if keys.pressed(KeyCode::KeyA) { // left
            network.udp_socket.send_to("move_L".as_bytes(),
                                        network.host_name.clone()).expect("Error on send \"L\"");
        }

        if keys.pressed(KeyCode::KeyS) { // down
            network.udp_socket.send_to("move_D".as_bytes(),
                                        network.host_name.clone()).expect("Error on send \"D\"");
        }

        if keys.pressed(KeyCode::KeyD) { // right
            network.udp_socket.send_to("move_R".as_bytes(),
                                        network.host_name.clone()).expect("Error on send \"R\"");            
        }

        if keys.pressed(KeyCode::Space){
            network.udp_socket.send_to("shoot_S".as_bytes(),
                                        network.host_name.clone()).expect("Error on send \"S\"");
        }   

        
}