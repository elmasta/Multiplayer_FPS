use crate::game::Player;

use rand::Rng;
use rand::prelude::SliceRandom;

pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<String>>,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = vec![vec!["#".to_string(); width]; height];
        Self { width, height, grid }
    }

    fn generate(&mut self) {
        let mut rng = rand::thread_rng();
        let start_x = rng.gen_range(0..(self.width / 2)) * 2;
        let start_y = rng.gen_range(0..(self.height / 2)) * 2;

        self.carve_passage_from(start_x, start_y);
    }

    fn carve_passage_from(&mut self, x: usize, y: usize) {
        let mut rng = rand::thread_rng();
        let mut directions = vec![(2, 0), (-2, 0), (0, 2), (0, -2)];
        directions.shuffle(&mut rng);

        self.grid[y][x] = ".".to_string();

        for &(dx, dy) in &directions {
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;

            if nx < self.width && ny < self.height && self.grid[ny][nx] == "#".to_string() {
                let between_x = (x as isize + dx / 2) as usize;
                let between_y = (y as isize + dy / 2) as usize;
                self.grid[between_y][between_x] = ".".to_string();
                self.grid[ny][nx] = ".".to_string();
                self.carve_passage_from(nx, ny);
            }
        }
    }

    fn open(&mut self, openning: u8) {
        let mut rng = rand::thread_rng();
        for x_parc in 1..self.width-1 {
            for y_parc in 1..self.height-1 {
                if rng.gen_range(1..101) <= openning {
                    if !((self.grid[y_parc-1][x_parc] == ".".to_string() && self.grid[y_parc][x_parc+1] == ".".to_string()) ||
                         (self.grid[y_parc][x_parc+1] == ".".to_string() && self.grid[y_parc+1][x_parc] == ".".to_string()) ||
                         (self.grid[y_parc+1][x_parc] == ".".to_string() && self.grid[y_parc][x_parc-1] == ".".to_string()) ||
                         (self.grid[y_parc][x_parc-1] == ".".to_string() && self.grid[y_parc-1][x_parc] == ".".to_string())) {
                        
                        self.grid[y_parc][x_parc] = ".".to_string()
                    }
                }
            }
        }
    }

    pub fn insert_player(&mut self, player: &mut Player) {
        let mut rng = rand::thread_rng();
        loop {
            let rand_y = rng.gen_range(0..self.height);
            let rand_x = rng.gen_range(0..self.width);
            if self.grid[rand_y][rand_x] == ".".to_string() {
                self.grid[rand_y][rand_x] = player.number.to_string().clone();
                player.position = [rand_x as u8, rand_y as u8];
                break
            }
        }
    }

    pub fn display(&self) {
        for row in &self.grid {
            for cell in row {
                print!("{}", cell);
            }
            println!();
        }
    }

    pub fn grid_to_string(&self) -> String {
        let mut result = String::from("maze_setup ");
        for row in &self.grid {
            for cell in row {
                result += cell;
            }
            result += " ";
        }
        result
    }
}

pub fn maze_creation(width: usize, height: usize, openning: u8) -> Maze {
    let mut maze = Maze::new(width, height);
    maze.generate();
    maze.open(openning);
    return maze;
}