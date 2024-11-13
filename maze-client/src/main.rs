
use std::net::UdpSocket;
use std::env;
use std::str;
use std::io;

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
};



mod spawn_maze;
use spawn_maze::*;
mod structs;
use structs::*;
mod handle_network_events;
use handle_network_events::*;
mod send_inputs;
use send_inputs::*;
mod update_maze;
use update_maze::*;
mod fps_count;
use fps_count::*;



fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage {} hostname", args[0]);
        std::process::exit(1);
    }
    let hostname = args[1].clone();

    let socket = UdpSocket::bind("[::]:0").expect("socket binding failed"); // for UDP4/6
    
    socket.connect(hostname.to_string() + &":8080").expect("couldn't connect to address");
    //ask to connect to the server
    socket.send_to("ask_connection".as_bytes(), hostname.to_string() + &":8080")
	    .expect("Error on send");

    //wait for connexion confirmation
    let mut buf = [0; 2048];
	if let Ok((amt, _src)) = socket.recv_from(&mut buf){ // TODO : verifier que ce soit le bon message
        let received_data = str::from_utf8(&buf[..amt]).unwrap();
        println!("{:?}", received_data);    
    }

    //ask for name
    println!("Enter your username :");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed to read line");
    let username = username.trim();

    socket.send_to(("enter_name ".to_owned() + username).as_bytes(), hostname.to_string() + &":8080")
	    .expect("Error on send");

    //wait for username confirmation / maze setup
    let mut maze_setup = String::new();
    if let Ok((amt, _src)) = socket.recv_from(&mut buf){ 
        let received_data = str::from_utf8(&buf[..amt]).unwrap();
        println!("{:?} has joined the game", received_data);

        maze_setup = received_data.to_string();
    }

    let new_players_list: Vec<Player> =  Vec::new();
    let new_laser_list: LaserList =  structs::LaserList(Vec::new());

    socket.set_nonblocking(true).unwrap();
    App::new()
        .insert_resource(NetworkResource {
            udp_socket: socket,
            host_name: hostname + &":8080",
        })
        .insert_resource(Maze {
            raw_grid : maze_setup,
        })
        .insert_resource(PlayersList {
            players : new_players_list,
            current_player : username.to_string(), 
        })
        .insert_resource( new_laser_list )
        .add_plugins(DefaultPlugins)    
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, handle_network_events)
        .add_systems(Update, send_player_inputs)
        .add_systems(Update, update_player)
        .add_systems(Update, update_laser)
        .add_systems(Startup, infotext_system)
        .add_systems(Update, change_text_system)

        
        .run();

}


