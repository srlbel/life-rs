use clap::Parser;
use cli::Args;
use rand::Rng;
use std::{thread, time::Duration};

mod cli;

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

#[derive(Clone)]
struct GameOfLife {
    grid: Vec<Vec<Cell>>,
    height: usize,
    width: usize,
}

impl GameOfLife {
    fn new(height: usize, width: usize) -> Self {
        let mut grid = vec![vec![Cell::Dead; width]; height];
        let mut rng = rand::thread_rng();

        for y in 0..height {
            for x in 0..height {
                grid[y][x] = if rng.gen_bool(0.3) {
                    Cell::Alive
                } else {
                    Cell::Dead
                };
            }
        }

        Self {
            grid,
            height,
            width,
        }
    }

    fn update(&mut self) {
        let mut new_grid = self.grid.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let alive_neighbors = self.get_neighbours(x, y);
                let current = self.grid[y][x];

                new_grid[y][x] = match (current, alive_neighbors) {
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Dead, 3) => Cell::Alive,
                    _ => Cell::Dead,
                };
            }
        }

        self.grid = new_grid.to_vec();
    }

    fn get_neighbours(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        let h = self.height as isize;
        let w = self.width as isize;

        for dy in [-1, 0, 1] {
            for dx in [-1, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx >= 0 && nx < w && ny >= 0 && ny < h {
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
    let args = Args::parse();
    let height = args.height;
    let width = args.width;

    let mut game = GameOfLife::new(height, width);

    loop {
        game.display();
        game.update();
        thread::sleep(Duration::from_millis(args.time));
    }
}
