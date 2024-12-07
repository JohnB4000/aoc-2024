use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use super::Day;

#[derive(Clone, Debug)]
pub enum Positions {
    Empty,
    Obstacle,
    Guard(Direction, Vec<Direction>),
    Visited(Vec<Direction>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Day06;

impl Day for Day06 {
    type ParsedData = Vec<Vec<Positions>>;

    type Part1Result = i32;

    type Part2Result = i32;

    fn parse(input_directory: String) -> Result<Self::ParsedData, String> {
        let mut grid = Vec::new();

        BufReader::new(File::open(input_directory + "day06.txt").expect("File Error"))
            .lines()
            .for_each(|line| {
                let mut row = Vec::new();
                line.unwrap().chars().for_each(|c| match c {
                    '#' => row.push(Positions::Obstacle),
                    '.' => row.push(Positions::Empty),
                    '^' => row.push(Positions::Guard(Direction::Up, Vec::new())),
                    character => todo!("Encountered character: {}", character),
                });
                grid.push(row);
            });

        Ok(grid)
    }

    fn part_1(input: &Self::ParsedData) -> Self::Part1Result {
        let mut grid: Vec<Vec<Positions>> = input.clone();
        let (mut i, mut j) = find_guard(&input);

        loop {
            if let Positions::Guard(direction, _) = grid[i][j].clone() {
                grid[i][j] = Positions::Visited(Vec::new());

                let offset = match direction {
                    Direction::Up => (-1, 0),
                    Direction::Down => (1, 0),
                    Direction::Left => (0, -1),
                    Direction::Right => (0, 1),
                };

                if i as i32 + offset.0 < 0
                    || i as i32 + offset.0 >= grid.len() as i32
                    || j as i32 + offset.1 < 0
                    || j as i32 + offset.1 >= grid[0].len() as i32
                {
                    break;
                }

                let newi = (i as i32 + offset.0) as usize;

                let newj = (j as i32 + offset.1) as usize;
                match grid[newi][newj] {
                    Positions::Obstacle => {
                        let new_direction = match direction {
                            Direction::Up => Direction::Right,
                            Direction::Down => Direction::Left,
                            Direction::Left => Direction::Up,
                            Direction::Right => Direction::Down,
                        };
                        grid[i][j] = Positions::Guard(new_direction, Vec::new());
                    }
                    _ => {
                        grid[newi][newj] = Positions::Guard(direction, Vec::new());
                        i = newi;
                        j = newj;
                    }
                }
            }
        }

        grid.iter()
            .map(|row| {
                row.iter()
                    .map(|element| match element {
                        Positions::Visited(_) => 1,
                        _ => 0,
                    })
                    .sum::<i32>()
            })
            .sum()
    }

    fn part_2(input: &Self::ParsedData) -> Self::Part2Result {
        let mut grid: Vec<Vec<Positions>> = input.clone();
        let (mut i, mut j) = find_guard(&input);

        loop {
            if let Positions::Guard(direction, _) = grid[i][j].clone() {
                grid[i][j] = Positions::Visited(Vec::new());

                let offset = match direction {
                    Direction::Up => (-1, 0),
                    Direction::Down => (1, 0),
                    Direction::Left => (0, -1),
                    Direction::Right => (0, 1),
                };

                if i as i32 + offset.0 < 0
                    || i as i32 + offset.0 >= grid.len() as i32
                    || j as i32 + offset.1 < 0
                    || j as i32 + offset.1 >= grid[0].len() as i32
                {
                    break;
                }

                let newi = (i as i32 + offset.0) as usize;

                let newj = (j as i32 + offset.1) as usize;
                match grid[newi][newj] {
                    Positions::Obstacle => {
                        let new_direction = match direction {
                            Direction::Up => Direction::Right,
                            Direction::Down => Direction::Left,
                            Direction::Left => Direction::Up,
                            Direction::Right => Direction::Down,
                        };
                        grid[i][j] = Positions::Guard(new_direction, Vec::new());
                    }
                    _ => {
                        grid[newi][newj] = Positions::Guard(direction, Vec::new());
                        i = newi;
                        j = newj;
                    }
                }
            }
        }

        let guard_pos = find_guard(&input);

        let mut possible_indexes = Vec::new();
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if (i, j) == guard_pos {
                    continue;
                }
                if let Positions::Visited(_) = grid[i][j] {
                    possible_indexes.push((i, j));
                }
            }
        }

        possible_indexes
            .into_iter()
            .filter(|coords| place_obstacle(&input, *coords))
            .collect::<Vec<(usize, usize)>>()
            .len() as i32
    }
}

fn find_guard(input: &Vec<Vec<Positions>>) -> (usize, usize) {
    for i in 0..input.len() {
        for j in 0..input.len() {
            if let Positions::Guard(_, _) = input[i][j] {
                return (i, j);
            }
        }
    }

    (0, 0)
}

// fn print_grid(input: &Vec<Vec<Positions>>) {
//     for row in input {
//         for element in row {
//             match element {
//                 Positions::Empty => print!("."),
//                 Positions::Obstacle => print!("#"),
//                 Positions::Guard(direction, _) => match direction {
//                     Direction::Up => print!("^"),
//                     Direction::Down => print!("v"),
//                     Direction::Left => print!("<"),
//                     Direction::Right => print!(">"),
//                 },
//                 Positions::Visited(_) => print!("X"),
//             }
//         }
//         println!("");
//     }
//     println!("")
// }

fn place_obstacle(input: &Vec<Vec<Positions>>, coords: (usize, usize)) -> bool {
    let mut grid = input.clone();

    grid[coords.0][coords.1] = Positions::Obstacle;

    let (mut i, mut j) = find_guard(&input);

    loop {
        if let Positions::Guard(direction, directions) = grid[i][j].clone() {
            if directions.contains(&direction) {
                return true;
            }

            let mut directions = directions.clone();
            directions.push(direction.clone());
            grid[i][j] = Positions::Visited(directions.clone());

            let offset = match direction {
                Direction::Up => (-1, 0),
                Direction::Down => (1, 0),
                Direction::Left => (0, -1),
                Direction::Right => (0, 1),
            };

            if i as i32 + offset.0 < 0
                || i as i32 + offset.0 >= grid.len() as i32
                || j as i32 + offset.1 < 0
                || j as i32 + offset.1 >= grid[0].len() as i32
            {
                return false;
            }

            let newi = (i as i32 + offset.0) as usize;
            let newj = (j as i32 + offset.1) as usize;

            match &grid[newi][newj] {
                Positions::Obstacle => {
                    let new_direction = match direction {
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                    };
                    grid[i][j] = Positions::Guard(new_direction, directions);
                }
                _ => {
                    if let Positions::Visited(directions) = &grid[newi][newj] {
                        grid[newi][newj] = Positions::Guard(direction, directions.to_vec());
                    } else {
                        grid[newi][newj] = Positions::Guard(direction, Vec::new());
                    }
                    i = newi;
                    j = newj;
                }
            }
        }
    }
}
