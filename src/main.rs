use std::{io, thread, time};

const ROW: usize = 9;
const COL: usize = 9;

#[derive(Copy, Clone, Debug)]
struct Position(i32, i32);
impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.0, self.1)
    }
}

#[derive(Copy, Clone, Debug)]
struct Cell {
    status: bool,
    position: Position,
}

enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    LeftUp,
}
impl Direction {
    const VALUES: [Self; 8] = [
        Self::Up,
        Self::UpRight,
        Self::Right,
        Self::DownRight,
        Self::Down,
        Self::DownLeft,
        Self::Left,
        Self::LeftUp,
    ];
}

impl Cell {
    fn get_neighboring_coordinate(&self, direction: Direction) -> Position {
        match direction {
            Direction::Up => {
                // println!("Up");
                Position(self.position.0 - 1, self.position.1)
            }
            Direction::UpRight => {
                // println!("UpRight");
                Position(self.position.0 - 1, self.position.1 + 1)
            }
            Direction::Right => {
                // println!("Right")
                Position(self.position.0, self.position.1 + 1)
            }
            Direction::DownRight => {
                // println!("DownRight")
                Position(self.position.0 + 1, self.position.1 + 1)
            }
            Direction::Down => {
                // println!("Down")
                Position(self.position.0 + 1, self.position.1)
            }
            Direction::DownLeft => {
                // println!("DownLeft")
                Position(self.position.0 + 1, self.position.1 - 1)
            }
            Direction::Left => {
                // println!("Left")
                Position(self.position.0, self.position.1 - 1)
            }
            Direction::LeftUp => {
                // println!("LeftUp")
                Position(self.position.0 - 1, self.position.1 - 1)
            }
        }
    }
}

fn get_valid_position(origin_position: Position, position: Position) -> Position {
    let mut x_pos = position.1;
    let mut y_pos = position.0;
    if position.0 < 0 || (position.0 >= ROW as i32) {
        y_pos = origin_position.0;
        x_pos = origin_position.1;
    } else if position.1 < 0 || (position.1 >= COL as i32) {
        x_pos = origin_position.1;
        y_pos = origin_position.0;
    }
    Position(y_pos, x_pos)
}

fn render(world: &[[Cell; COL]; ROW]) {
    for (_, rows) in world.iter().enumerate().take(ROW) {
        for (_, cell) in rows.iter().enumerate().take(COL) {
            if cell.status {
                print!(" 0 ");
            } else {
                print!(" . ");
            }
        }
        println!();
    }
}

// Any live cell with fewer than two live neighbors dies, as if by underpopulation.
// Any live cell with two or three live neighbors lives on to the next generation.
// Any live cell with more than three live neighbors dies, as if by overpopulation.
// Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
fn step(world: &mut [[Cell; COL]; ROW]) {
    let mut update_list : Vec<(Position, bool)> = Vec::new(); 
    for i in 0..ROW {
        for j in 0..COL {
            let mut active_neighbour_count = 0;
            // For each direction
            for direction in Direction::VALUES {
                let neighbour_position = get_valid_position(
                    world[i][j].position,
                    world[i][j].get_neighboring_coordinate(direction),
                );
                
                if !(neighbour_position.0 == i as i32 && neighbour_position.1 == j as i32)
                {
                    if world[neighbour_position.0 as usize][neighbour_position.1 as usize].status {
                        active_neighbour_count += 1;
                    }
                }
            }

            // println!("{}: COUNT", active_neighbour_count);
            // print!(" {} ", active_neighbour_count);
            if world[i][j].status {
                if active_neighbour_count > 1 && active_neighbour_count < 4 {
                    // world[i][j].status = true;
                    update_list.push((Position(i as i32, j as i32), true));
                } else if active_neighbour_count < 2 || active_neighbour_count > 3 {
                    // world[i][j].status = false;
                    update_list.push((Position(i as i32, j as i32), false));
                }
            } else {
                if active_neighbour_count == 3 {
                    // println!("GOES HERE");
                    // world[i][j].status = true;
                    update_list.push((Position(i as i32, j as i32), true));
                }
            }
        }
        // println!();
    }

    // Actually modify the world
    for (update_position, new_status) in &update_list {
        world[update_position.0 as usize][update_position.1 as usize].status = *new_status;
    }
}

fn main() {
    let mut world: [[Cell; COL]; ROW] = [[Cell {
        position: Position(0, 0),
        status: false,
    }; COL]; ROW];

    // Initialize the actual cell
    for i in 0..ROW {
        for j in 0..COL {
            world[i][j].position = Position(i as i32, j as i32);
        }
    }

    // Initial seeds
    world[1][1].status = true;
    world[2][1].status = true;
    world[1][2].status = true;
    world[0][1].status = true;
    world[3][4].status = true;
    world[3][5].status = true;
    world[4][5].status = true;

    loop {
        print!("{}[2J", 27 as char);
        render(&world);
        // let mut input: String = "".to_string(); // Uncomment this for pause every step
        // let _ = io::stdin().read_line(&mut input); // Uncomment this for pause every step

        thread::sleep(time::Duration::from_millis(250));
        step(&mut world);
    }
}
