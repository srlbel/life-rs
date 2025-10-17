use rand::Rng;
use std::{thread, time::Duration};

const W: usize = 200;
const H: usize = 100;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Alive,
    Dead,
}

impl Cell {
    fn to_char(self) -> char {
        match self {
            Cell::Alive => '█',
            Cell::Dead => ' ',
        }
    }
}

struct GameOfLife {
    grid: [[Cell; W]; H],
}

impl GameOfLife {
    fn new() -> Self {
        let mut grid = [[Cell::Dead; W]; H];
        let mut rng = rand::thread_rng();

        for y in 0..H {
            for x in 0..W {
                grid[y][x] = if rng.gen_bool(0.3) {
                    Cell::Alive
                } else {
                    Cell::Dead
                };
            }
        }

        Self { grid }
    }

    fn update(&mut self) {
        let mut new_grid = self.grid;

        for y in 0..H {
            for x in 0..W {
                let alive_neighbors = self.get_neighbours(x, y);
                let current = self.grid[y][x];

                new_grid[y][x] = match (current, alive_neighbors) {
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Dead, 3) => Cell::Alive,
                    _ => Cell::Dead,
                };
            }
        }

        self.grid = new_grid;
    }

    fn get_neighbours(&self, x: usize, y: usize) -> usize {
        let mut count = 0;

        for dy in [-1, 0, 1] {
            for dx in [-1, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx >= 0 && nx < W as isize && ny >= 0 && ny < H as isize {
                    if self.grid[ny as usize][nx as usize] == Cell::Alive {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    fn display(&self) {
        println!("\x1B[2J\x1B[1;1H");
        for row in &self.grid {
            for &cell in row {
                print!("{}", cell.to_char());
            }
            println!();
        }
    }
}

fn main() {
    let mut game = GameOfLife::new();

    loop {
        game.display();
        game.update();
        thread::sleep(Duration::from_millis(100));
    }
}
