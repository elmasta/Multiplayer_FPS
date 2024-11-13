mod maze;
mod game;

use std::net::UdpSocket;
//use std::time::Duration;
use core::net::SocketAddr;

fn main() -> std::io::Result<()> {
    // Créer et lier le socket UDP
    let socket = UdpSocket::bind("[::]:8080")?;  // pour UDP4/6
    let mut next_nmb = '0';
    let mut buf = [0; 2048];
    let mut sources: Vec<SocketAddr> = Vec::new();
    let mut players: Vec<game::Player> = Vec::new();
    let mut new_maze: maze::Maze = maze::maze_creation(31, 15, 30);

    loop {
        let (_amt, src) = socket.recv_from(&mut buf).unwrap();
        let message = std::str::from_utf8(&buf[.._amt]).unwrap(); 
        if message.contains("ask_connection") {
            if !sources.iter().any(|source| *source == src) {
                sources.push(src);
                socket.send_to("Succesfully connected to the server".as_bytes(), src).unwrap();
            } else {
                socket.send_to("Already connected".as_bytes(), src).unwrap();
            }

        } else if message.contains("move_U") || message.contains("move_R") || message.contains("move_D") || message.contains("move_L") {
            game::move_player(&mut players, &mut new_maze, message, src);
            for player in &players {
                socket.send_to(game::player_to_string(&players).as_bytes(), player.source).unwrap();
            }
            new_maze.display(); //debug
            println!();

        } else if message.contains("shoot_S") {
            let tmp = game::shoot_player(&mut players, &mut new_maze, src);
            for player in &players {
                //socket.send_to(game::player_to_string(&players).as_bytes(), player.source).unwrap();
                socket.send_to(format!("LASER-Direction: {}| Coordinates: {:?}LASEREND{}", tmp.laser_direction, tmp.laser_coordinates, game::player_to_string(&players)).as_bytes(), player.source).unwrap();
            }
            new_maze.display(); //debug
            println!();

        } else if message.contains("enter_name") {
            let mut new_player_name = message.strip_prefix("enter_name ").expect("failed to parse new player name").to_string();
            new_player_name = new_player_name.chars().take(6).collect();
            //check if player exist
            let mut exist = false;
            for player in &players {
                if player.name == new_player_name {
                    exist = true;
                    socket.send_to("PlayerExist".as_bytes(), src).unwrap();
                    break;
                }
            }
            if !exist {
                let mut new_player = game::Player{source: src, name: new_player_name.clone(), number: next_nmb, direction: 'D', points: 0, position: [0, 0], move_cooldown: game::Cooldown::new(200), shoot_cooldown: game::Cooldown::new(1000)};
                let mut inserted = false;
                for source in &sources {
                    if *source == src { // player who just entered their name 
                        if next_nmb == '9' {
                            next_nmb = 'A'
                        } else {
                            next_nmb = (next_nmb as u8 + 1) as char;
                        }
                        new_maze.insert_player(&mut new_player);
                        players.push(new_player.clone());
                        socket.send_to(new_maze.grid_to_string().as_bytes(), src).unwrap();
                        inserted = true;
                    }
                }
                if inserted {
                    let formatted_string = format!("JOINED-Name: {}, Number: {}, PositionX: {:?}, PositionY: {:?}, ", new_player_name.clone(), new_player.number, new_player.position[0], new_player.position[1]);
                    for source in &sources {
                        if !(*source == src) { // every other player
                            socket.send_to(formatted_string.as_bytes(), source).unwrap();
                        }
                    }
                    new_maze.display(); //debug
                    println!();
                }
            }
        }

        //std::thread::sleep(Duration::from_millis(1));
    }
}
