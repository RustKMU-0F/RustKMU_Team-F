use std::collections::VecDeque;
use rand::{Rng, thread_rng};
use std::io;
use std::io::prelude::*;
use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
// use rand::thread_rng;

const MAP_SIZE: usize = 10;
const WALL: char = '#';
const PLAYER: char = 'P';
const EXIT: char = 'E';
const BOMB: char = '*';

// 깊이 우선 탐색(DFS) 알고리즘으로 미로 생성
fn dfs(maze: &mut Vec<Vec<char>>, row: usize, col: usize, rng: &mut ThreadRng) {
    let mut directions = vec![(0, -2), (0, 2), (-2, 0), (2, 0)];
    directions.shuffle(rng);
    for (dr, dc) in directions {
        let (r, c) = (row as i32 + dr, col as i32 + dc);
        if r < 1 || r >= (MAP_SIZE) as i32 || c < 1 || c >= (MAP_SIZE) as i32 {
            continue;
        }
        let (r, c) = (r as usize, c as usize);
        if maze[r][c] == '#' {
            maze[(row as i32 + dr / 2) as usize][(col as i32 + dc / 2) as usize] = ' ';
            maze[r][c] = ' ';
            dfs(maze, r, c, rng);
        }
    }
}

fn is_reachable(maze: &Vec<Vec<char>>, start_row: usize, start_col: usize) -> bool {
    let mut visited = vec![vec![false; MAP_SIZE]; MAP_SIZE];
    let mut queue = VecDeque::new();
    queue.push_back((start_row, start_col));
    visited[start_row][start_col] = true;

    while let Some((row, col)) = queue.pop_front() {
        if row == MAP_SIZE - 1 && col == MAP_SIZE - 1 {
            return true;
        }
        for (dr, dc) in &[(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let (r, c) = ((row as i32 + dr) as usize, (col as i32 + dc) as usize);

            #[allow(unused_comparisons)]
            if r < 0 || r >= MAP_SIZE || c < 0 || c >= MAP_SIZE || visited[r][c] || maze[r][c] == '#' || maze[r][c] == '*'{
                continue;
            }

            visited[r][c] = true;
            queue.push_back((r, c));
        }
    }
    false
}

fn init_map() -> Vec<Vec<char>> {

    // 미로를 나타내는 2차원 벡터 생성
    let mut map = vec![vec!['#'; MAP_SIZE]; MAP_SIZE];

    // 시작 지점을 무작위로 선택
    let mut rng = thread_rng();

    let exit_row = MAP_SIZE - 1;
    let exit_col = MAP_SIZE - 1;
    map[exit_row][exit_col] = EXIT;
    // set player
    let mut player_row = rng.gen_range(0..MAP_SIZE).max(1);
    let mut player_col = rng.gen_range(0..MAP_SIZE).max(1);
    while map[player_row][player_col] != '#' {
        player_row = rng.gen_range(0..MAP_SIZE);
        player_col = rng.gen_range(0..MAP_SIZE);
    }
    map[player_row][player_col] = PLAYER;
    // set bomb
    let mut bomb_row = rng.gen_range(0..MAP_SIZE);
    let mut bomb_col = rng.gen_range(0..MAP_SIZE);
    while map[bomb_row][bomb_col] != '#' {
        bomb_row = rng.gen_range(0..MAP_SIZE);
        bomb_col = rng.gen_range(0..MAP_SIZE);
    }
    map[bomb_row][bomb_col] = BOMB;


    dfs(&mut map, player_row, player_col, &mut rng);

    if !is_reachable(&map, player_row, player_col) {
        map = init_map();
    }

    map
}


fn play_game() {
    let mut map = init_map();
    let mut player_row = 0;
    let mut player_col = 0;
    // find player and exit
    for i in 1..MAP_SIZE {
        for j in 1..MAP_SIZE {
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
                if new_player_row >= MAP_SIZE || map[new_player_row][new_player_col] == WALL {
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
                if new_player_col >= MAP_SIZE || map[new_player_row][new_player_col] == WALL {
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


