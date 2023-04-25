use rand::Rng;
use std::io;
use std::io::prelude::*;

const MAP_SIZE: usize = 10;
const WALL: char = '#';
const PLAYER: char = 'P';
const EXIT: char = 'E';
const BOMB: char = '*';

fn init_map() -> Vec<Vec<char>> {
    let mut map = vec![vec![' '; MAP_SIZE]; MAP_SIZE];
    // set walls
    for i in 0..MAP_SIZE {
        map[0][i] = WALL;
        map[MAP_SIZE - 1][i] = WALL;
        map[i][0] = WALL;
        map[i][MAP_SIZE - 1] = WALL;
    }
    // set exit
    let mut rng = rand::thread_rng();
    let exit_row = rng.gen_range(1..MAP_SIZE - 1);
    let exit_col = rng.gen_range(1..MAP_SIZE - 1);
    while map[exit_row][exit_col] != ' ' {
        let exit_row = rng.gen_range(1..MAP_SIZE - 1);
        let exit_col = rng.gen_range(1..MAP_SIZE - 1);
    }
    map[exit_row][exit_col] = EXIT;
    // set player
    let mut player_row = rng.gen_range(1..MAP_SIZE - 1);
    let mut player_col = rng.gen_range(1..MAP_SIZE - 1);
    while map[player_row][player_col] != ' ' {
        player_row = rng.gen_range(1..MAP_SIZE - 1);
        player_col = rng.gen_range(1..MAP_SIZE - 1);
    }
    map[player_row][player_col] = PLAYER;
    // set bomb
    let mut bomb_row = rng.gen_range(1..MAP_SIZE - 1);
    let mut bomb_col = rng.gen_range(1..MAP_SIZE - 1);
    while map[bomb_row][bomb_col] != ' ' {
        bomb_row = rng.gen_range(1..MAP_SIZE - 1);
        bomb_col = rng.gen_range(1..MAP_SIZE - 1);
    }
    map[bomb_row][bomb_col] = BOMB;
    map
}


fn play_game() {
    let mut map = init_map();
    let mut player_row = 0;
    let mut player_col = 0;
    // find player and exit
    for i in 1..MAP_SIZE - 1 {
        for j in 1..MAP_SIZE - 1 {
            if map[i][j] == PLAYER {
                player_row = i;
                player_col = j;
            }
        }
    }
    loop {
        // print map
        for row in &map {
            for cell in row {
                print!("{}", cell);
            }
            println!();
        }
        // get user input
        let mut input = String::new();
        print!("Enter a direction (up, down, left, right): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        // process input
        let mut new_player_row = player_row;
        let mut new_player_col = player_col;
        match input.as_str() {
            "up" => {
                new_player_row -= 1;
                if new_player_row < 1 || map[new_player_row][new_player_col] == WALL {
                    continue;
                }
                if map[new_player_row][new_player_col] == BOMB {
                    println!("You hit a bomb! Game over!");
                    return;
                }
                map[player_row][player_col] = ' ';
                player_row = new_player_row;
                if map[player_row][player_col] == EXIT {
                    println!("Congratulations! You escaped the maze!");
                    return;
                }
                map[player_row][player_col] = PLAYER;
            }
            "down" => {
                new_player_row += 1;
                if new_player_row >= MAP_SIZE - 1 || map[new_player_row][new_player_col] == WALL {
                    continue;
                }
                if map[new_player_row][new_player_col] == BOMB {
                    println!("You hit a bomb! Game over!");
                    return;
                }
                map[player_row][player_col] = ' ';
                player_row = new_player_row;
                if map[player_row][player_col] == EXIT {
                    println!("Congratulations! You escaped the maze!");
                    return;
                }
                map[player_row][player_col] = PLAYER;
            }
            "left" => {
                new_player_col -= 1;
                if new_player_col < 1 || map[new_player_row][new_player_col] == WALL {
                    continue;
                }
                if map[new_player_row][new_player_col] == BOMB {
                    println!("You hit a bomb! Game over!");
                    return;
                }
                map[player_row][player_col] = ' ';
                player_col = new_player_col;
                if map[player_row][player_col] == EXIT {
                    println!("Congratulations! You escaped the maze!");
                    return;
                }
                map[player_row][player_col] = PLAYER;
            }
            "right" => {
                new_player_col += 1;
                if new_player_col >= MAP_SIZE - 1 || map[new_player_row][new_player_col] == WALL {
                    continue;
                }
                if map[new_player_row][new_player_col] == BOMB {
                    println!("You hit a bomb! Game over!");
                    return;
                }
                map[player_row][player_col] = ' ';
                player_col = new_player_col;
                if map[player_row][player_col] == EXIT {
                    println!("Congratulations! You escaped the maze!");
                    return;
                }
                map[player_row][player_col] = PLAYER;
            }
            _ => {
                println!("Invalid input. Try again.");
                continue;
            }
        }
    }
}

fn main() {
    play_game();
}


