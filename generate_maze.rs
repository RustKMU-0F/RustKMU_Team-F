use rand::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;


// 미로의 크기
const ROWS: usize = 10;
const COLS: usize = 10;

// 깊이 우선 탐색(DFS) 알고리즘으로 미로 생성
fn dfs(maze: &mut Vec<Vec<char>>, row: usize, col: usize, rng: &mut ThreadRng) {
    let mut directions = vec![(0, -2), (0, 2), (-2, 0), (2, 0)];
    // directions.shuffle(&mut rng);
    directions.shuffle(rng);
    for (dr, dc) in directions {
        let (r, c) = (row as i32 + dr, col as i32 + dc);
        if r < 1 || r >= (ROWS * 2) as i32 || c < 1 || c >= (COLS * 2) as i32 {
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

fn main(){

// 미로를 나타내는 2차원 벡터 생성
    let mut maze = vec![vec!['#'; COLS * 2 + 1]; ROWS * 2 + 1];


// 시작 지점을 무작위로 선택
    let mut rng = thread_rng();
    let start_row = rng.gen_range(1..ROWS * 2).max(1);
    let start_col = rng.gen_range(1..COLS * 2).max(1);
    maze[start_row][start_col] = ' ';



    dfs(&mut maze, start_row, start_col, &mut rng);

// 미로 출력
    for row in &maze {
        println!("{}", row.iter().collect::<String>());
    }
}
