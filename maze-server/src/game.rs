use crate::maze::Maze;

use core::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, PartialEq, Debug)]
pub struct Cooldown {
    pub duration: u128, // cooldown duration in millisec
    pub last_used: Option<u128>, // unix timestamp in millisec
}

impl Cooldown {
    // create the cooldown
    pub fn new(duration: u128) -> Cooldown {
        Cooldown {
            duration,
            last_used: None,
        }
    }

    fn current_time_unix_ms() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time is going backward!")
            .as_millis()
    }

    //check if cooldown is over
    fn is_ready(&self) -> bool {
        match self.last_used {
            Some(last) => Cooldown::current_time_unix_ms() >= last + self.duration,
            None => true, // if first use, it's ready
        }
    }

    //start the cooldown, return true if it was started and so if the action could be done
    fn trigger(&mut self) -> bool {
        if self.is_ready() {
            self.last_used = Some(Cooldown::current_time_unix_ms());
            true
        } else {
            false
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Player {
    pub source: SocketAddr,
    pub name: String,
    pub number: char,
    pub direction: char,
    pub points: u16,
    pub position: [u8; 2],
    pub move_cooldown: Cooldown,
    pub shoot_cooldown: Cooldown,
}

pub struct ShootInfo {
    pub laser_direction: char,
    pub laser_coordinates: Vec<Vec<usize>>
}

pub fn player_to_string(players: &Vec<Player>) -> String {

    let mut result = String::from("player_move ");
    result += 
    players.into_iter()
        .map(|player| {
            format!(
                "Name: {}, Number: {}, Direction: {}, PositionX: {:?}, PositionY: {:?}, Points: {}",
                player.name, player.number, player.direction, player.position[0], player.position[1], player.points
            )
        })
        .collect::<Vec<String>>()
        .join("\n").as_str()
    ;
    result
}

pub fn move_player(players: &mut Vec<Player>, playing_maze: &mut Maze, message: &str, src: SocketAddr) {
    for player in players {
        if player.source == src {
            if message.contains("move_U") | message.contains("move_D") {
                match player.direction {
                    'U' => {
                        if message.contains("move_U") {
                            if let Some(result) = player.position[1].checked_sub(1) {
                                if playing_maze.grid[result as usize][player.position[0] as usize] == "." && /*check cooldown*/ player.move_cooldown.trigger() {
                                    playing_maze.grid[result as usize][player.position[0] as usize] = player.number.to_string();
                                    playing_maze.grid[player.position[1] as usize][player.position[0] as usize] = ".".to_string();
                                    player.position[1] -= 1;
                                }
                            }
                        } else {
                            if !(player.position[1]+1 >= playing_maze.height as u8) {
                                if playing_maze.grid[(player.position[1]+1) as usize][player.position[0] as usize] == "." && /*check cooldown*/ player.move_cooldown.trigger() {
                                    playing_maze.grid[(player.position[1]+1) as usize][player.position[0] as usize] = player.number.to_string();
                                    playing_maze.grid[player.position[1] as usize][player.position[0] as usize] = ".".to_string();
                                    player.position[1] += 1;
                                }
                            }
                        }
                    },
                    'D' => {
                        if message.contains("move_U") {
                            if !(player.position[1]+1 >= playing_maze.height as u8) {
                                if playing_maze.grid[(player.position[1]+1) as usize][player.position[0] as usize] == "." && /*check cooldown*/ player.move_cooldown.trigger() {
                                    playing_maze.grid[(player.position[1]+1) as usize][player.position[0] as usize] = player.number.to_string();
                                    playing_maze.grid[player.position[1] as usize][player.position[0] as usize] = ".".to_string();
                                    player.position[1] += 1;
                                }
                            }
                        } else {
                            if let Some(result) = player.position[1].checked_sub(1) {
                                if playing_maze.grid[result as usize][player.position[0] as usize] == "." && /*check cooldown*/ player.move_cooldown.trigger() {
                                    playing_maze.grid[result as usize][player.position[0] as usize] = player.number.to_string();
                                    playing_maze.grid[player.position[1] as usize][player.position[0] as usize] = ".".to_string();
                                    player.position[1] -= 1;
                                }
                            }
                        }
                    },
                    'L' => {
                        if message.contains("move_U") {
                            if let Some(result) = player.position[0].checked_sub(1) {
                                if playing_maze.grid[player.position[1] as usize][result as usize] == "." && /*check cooldown*/ player.move_cooldown.trigger() {
                                    playing_maze.grid[player.position[1] as usize][result as usize] = player.number.to_string();
                                    playing_maze.grid[player.position[1] as usize][player.position[0] as usize] = ".".to_string();
                                    player.position[0] -= 1;
                                }
                            }
                        } else {
                            if !(player.position[0]+1 >= playing_maze.width as u8) {
                                if playing_maze.grid[player.position[1] as usize][(player.position[0]+1) as usize] == "." && /*check cooldown*/ player.move_cooldown.trigger() {
                                    playing_maze.grid[player.position[1] as usize][(player.position[0]+1) as usize] = player.number.to_string();
                                    playing_maze.grid[player.position[1] as usize][player.position[0] as usize] = ".".to_string();
                                    player.position[0] += 1;
                                }
                            }
                        }
                    },
                    'R' => {
                        if message.contains("move_U") {
                            if !(player.position[0]+1 >= playing_maze.width as u8) {
                                if playing_maze.grid[player.position[1] as usize][(player.position[0]+1) as usize] == "." && /*check cooldown*/ player.move_cooldown.trigger() {
                                    playing_maze.grid[player.position[1] as usize][(player.position[0]+1) as usize] = player.number.to_string();
                                    playing_maze.grid[player.position[1] as usize][player.position[0] as usize] = ".".to_string();
                                    player.position[0] += 1;
                                }
                            }
                        } else {
                            if let Some(result) = player.position[0].checked_sub(1) {
                                if playing_maze.grid[player.position[1] as usize][result as usize] == "." && /*check cooldown*/ player.move_cooldown.trigger() {
                                    playing_maze.grid[player.position[1] as usize][result as usize] = player.number.to_string();
                                    playing_maze.grid[player.position[1] as usize][player.position[0] as usize] = ".".to_string();
                                    player.position[0] -= 1;
                                }
                            }
                        }
                    },
                    _ => {},
                }
            } else if message.contains("move_L") && /*check cooldown*/ player.move_cooldown.trigger() {
                match player.direction {
                    'U' => {
                        player.direction = 'L';
                    },
                    'R' => {
                        player.direction = 'U';
                    },
                    'D' => {
                        player.direction = 'R';
                    },
                    'L' => {
                        player.direction = 'D';
                    },
                    _ => {},
                }
            } else if message.contains("move_R") && /*check cooldown*/ player.move_cooldown.trigger() {
                match player.direction {
                    'U' => {
                        player.direction = 'R';
                    },
                    'R' => {
                        player.direction = 'D';
                    },
                    'D' => {
                        player.direction = 'L';
                    },
                    'L' => {
                        player.direction = 'U';
                    },
                    _ => {},
                }
            }
        }
    }
}

pub fn shoot_player(players: &mut Vec<Player>, maze: &mut Maze, src: SocketAddr) -> ShootInfo {
    let mut player_hit: Vec<String> = Vec::new();
    let mut shooter_index = 0;
    let mut laser_coordinates = Vec::new();
    if let Some(index) = players.iter().position(|player| player.source == src) {
        if players[index].shoot_cooldown.trigger() {
            let mut laser_position = players[index].position;
            println!("{:?}", players[index]);
            println!("{:?}", laser_position);
            shooter_index = index;
            match players[index].direction {
                'U' => {
                    while let Some(result) = laser_position[1].checked_sub(1) {
                        println!("{}, {}", result, laser_position[0]);
                        if maze.grid[result as usize][laser_position[0] as usize] != "#" {
                            laser_coordinates.push(vec![result as usize, laser_position[0] as usize]);
                            if maze.grid[result as usize][laser_position[0] as usize] != "." {
                                // a player is hit
                                players[index].points += 1;
                                player_hit.push(maze.grid[result as usize][laser_position[0] as usize].clone());
                                maze.grid[result as usize][laser_position[0] as usize] = ".".to_string();
                            }
                            laser_position[1] = result;
                        } else {
                            break;
                        }
                    }
                },
                'D' => {
                    laser_position[1] += 1;
                    while laser_position[1] < maze.height as u8 {
                        println!("{}, {}", laser_position[1], laser_position[0]);
                        if maze.grid[laser_position[1] as usize][laser_position[0] as usize] != "#" {
                            laser_coordinates.push(vec![laser_position[1] as usize, laser_position[0] as usize]);
                            if maze.grid[laser_position[1] as usize][laser_position[0] as usize] != "." {
                                // a player is hit
                                players[index].points += 1;
                                player_hit.push(maze.grid[laser_position[1] as usize][laser_position[0] as usize].clone());
                                maze.grid[laser_position[1] as usize][laser_position[0] as usize] = ".".to_string();
                            }
                            laser_position[1] += 1;
                        } else {
                            break;
                        }
                    }
                },
                'L' => {
                    while let Some(result) = laser_position[0].checked_sub(1) {
                        println!("{}, {}", laser_position[1], result);
                        if maze.grid[laser_position[1] as usize][result as usize] != "#" {
                            laser_coordinates.push(vec![laser_position[1] as usize, result as usize]);
                            if maze.grid[laser_position[1] as usize][result as usize] != "." {
                                // a player is hit
                                players[index].points += 1;
                                player_hit.push(maze.grid[laser_position[1] as usize][result as usize].clone());
                                maze.grid[laser_position[1] as usize][result as usize] = ".".to_string();
                            }
                            laser_position[0] = result;
                        } else {
                            break;
                        }
                    }
                },
                'R' => {
                    laser_position[0] += 1;
                    while laser_position[0] < maze.width as u8 {
                        println!("{}, {}", laser_position[1], laser_position[0]);
                        if maze.grid[laser_position[1] as usize][laser_position[0] as usize] != "#" {
                            laser_coordinates.push(vec![laser_position[1] as usize, laser_position[0] as usize]);
                            if maze.grid[laser_position[1] as usize][laser_position[0] as usize] != "." {
                                // a player is hit
                                players[index].points += 1;
                                player_hit.push(maze.grid[laser_position[1] as usize][laser_position[0] as usize].clone());
                                maze.grid[laser_position[1] as usize][laser_position[0] as usize] = ".".to_string();
                            }
                            laser_position[0] += 1;
                        } else {
                            break;
                        }
                    }
                },
                _ => {},
            }
        }
    }
    println!("{:?}", laser_coordinates);
    for touched in player_hit {
        for player in players.iter_mut() {
            if touched == player.number.to_string() {
                //respawn player
                maze.insert_player(player);
            }
        }
    }
    return ShootInfo{
        laser_direction: players[shooter_index].direction,
        laser_coordinates: laser_coordinates,
    }
}
