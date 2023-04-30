
use ggez::{Context, ContextBuilder, GameError, GameResult, input::keyboard::KeyCode};
use ggez::graphics::{self, Color, Rect};
use ggez::event::{self, EventHandler};
use oorandom::Rand32;
use rand::{Rng, thread_rng};
use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
use std::collections::VecDeque;
use ggez::input::keyboard::KeyInput;
use std::net::{TcpListener, TcpStream}; // line 10..12 socket, thread 관련 lib 추가
use std::io::{BufRead, BufReader, Write};
use std::thread;

const MAP_SIZE: usize = 30;
const WALL: char = '#';
const PLAYER: char = 'P';
const EXIT: char = 'E';
const BOMB: char = '*';
static mut player_row: usize = 0;
static mut player_col: usize = 0;
static mut bomb_row: usize = 0;
static mut bomb_col: usize = 0;
static mut exit_row: usize = 0;
static mut exit_col: usize = 0;

const DESIRED_FPS: u32 = 8;

unsafe fn init_map() -> Vec<Vec<char>> {

    // 미로를 나타내는 2차원 벡터 생성
    let mut map = vec![vec!['#'; MAP_SIZE]; MAP_SIZE];

    // 시작 지점을 무작위로 선택
    let mut rng = thread_rng();

    exit_row = MAP_SIZE - 1;
    exit_col = MAP_SIZE - 1;
    map[exit_row][exit_col] = EXIT;
    // set player
    player_row = rng.gen_range(0..MAP_SIZE).max(1);
    player_col = rng.gen_range(0..MAP_SIZE).max(1);
    while map[player_row][player_col] != '#' {
        player_row = rng.gen_range(0..MAP_SIZE);
        player_col = rng.gen_range(0..MAP_SIZE);
    }
    map[player_row][player_col] = PLAYER;
    // set bomb
    bomb_row = rng.gen_range(0..MAP_SIZE);
    bomb_col = rng.gen_range(0..MAP_SIZE);
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
            if r < 0 || r >= MAP_SIZE || c < 0 || c >= MAP_SIZE || visited[r][c] || maze[r][c] == '#' || maze[r][c] == '*' {
                continue;
            }

            visited[r][c] = true;
            queue.push_back((r, c));
        }
    }
    false
}

struct MyGame {
    wall: Wall,
    map: Vec<Vec<char>>,
    player: Player,
    bomb: Bomb,
    exit: Exit,
    draw_menu: Menu,
    solo:bool,
    mulit_player:Player,
}

impl MyGame {
    pub unsafe fn new(x: &mut Context) -> Self {
        let wall_pos = GridPosition { x: 0, y: 0 };
        MyGame {
            wall: Wall::new(wall_pos, false),
            map: init_map(),
            player: Player::new(GridPosition { x: (player_row) as i16, y: (player_col) as i16 }),
            bomb: Bomb::new(GridPosition { x: (bomb_row) as i16, y: (bomb_col) as i16 }),
            exit: Exit::new(GridPosition { x: (exit_row  ) as i16, y: (exit_col  )as i16 }),
            draw_menu: Menu::new(0, vec!["Solo".to_string(), "Multi".to_string(), "Join".to_string(), "Exit".to_string()]),
            solo: true,
            mulit_player: Player::new(GridPosition { x: (player_row) as i16, y: (player_col) as i16 }),
        }
    }
    fn update_multi(&mut self,solo:bool){
        self.solo = solo;
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
    Return,
}

impl Direction {
    pub fn inverse(&self) -> Self {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Return => Direction::Return,
        }
    }

    pub fn from_keycode(key: KeyCode) -> Option<Direction> {
        match key {
            KeyCode::Up => Some(Direction::Up),
            KeyCode::Down => Some(Direction::Down),
            KeyCode::Left => Some(Direction::Left),
            KeyCode::Right => Some(Direction::Right),
            KeyCode::Return => Some(Direction::Return),
            _ => None,
        }
    }
}

struct Menu{
    select: i32,
    pos: [f32;2],
    list: Vec<String>,
    in_menu: bool,
    solo: bool,
}
impl Menu{
    pub fn new(select:i32, list: Vec<String>) -> Self{
        Menu{select, pos: [910.0, 500.0], list, in_menu: true, solo: true}
    }
    fn draw(&self, canvas: &mut graphics::Canvas){
        if !self.in_menu{
            return;
        }
        let x = 950.0;
        let mut y = 500.0;
        for i in 0..4{
            let text = graphics::Text::new(self.list[i].clone());
            let coord = [x,y] ;
            canvas.draw(
                &text,
                graphics::DrawParam::new()
                    .dest(coord)
                    .color(Color::BLACK)
            );
            y += 30.0;
        }
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(Rect::new(
                    self.pos[0],
                    self.pos[1],
                    20.0,
                    20.0,
                ))
                .color(Color::BLACK)
        );
    }
    fn go(&mut self, dir: Direction) {
        match dir {
            Direction::Up => if self.pos[1] != 500.0 {self.pos[1] -= 30.0;self.select -= 1;},
            Direction::Down => if self.pos[1] != 590.0 {self.pos[1] += 30.0;self.select+=1; },
            Direction::Return => Menu::action(self),
            _ => {}
        }
    }
    fn action(&mut self){
        if self.select == 0{
            self.in_menu = false;
        }else if self.select == 1 {
            self.in_menu = false;
            self.solo = false;
            //add multi action
        }else if self.select == 2 {
            self.in_menu = false;
            //add join action
        }else if self.select == 3 {
            std::process::exit(0);
        }
    }
}

struct Exit {
    pos: GridPosition,
    can:bool,
}

impl Exit {
    pub fn new(pos: GridPosition) -> Self {
        Exit { pos, can: false }
    }
    fn draw(&self, canvas: &mut graphics::Canvas, solo:bool) {
        if self.can{
            if solo{
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(Rect::new(
                            ((self.pos.x * 40) + 350) as f32,
                            (self.pos.y * 40)  as f32,
                            40.0,
                            40.0,
                        ))
                        .color(Color::YELLOW), );
            }else{
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(Rect::new(
                            ((self.pos.x * 30)) as f32,
                            (self.pos.y * 40)  as f32,
                            30.0,
                            40.0,
                        ))
                        .color(Color::YELLOW), );
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(Rect::new(
                            ((self.pos.x * 30) + 1000) as f32,
                            (self.pos.y * 40)  as f32,
                            30.0,
                            40.0,
                        ))
                        .color(Color::YELLOW), );
            }

        }
    }
    fn update(&mut self, can : bool){
        self.can = can;
    }
}

struct Bomb {
    pos: GridPosition,
    timer: f32,
    can:bool,
}

impl Bomb {
    pub fn new(pos: GridPosition) -> Self {
        Bomb { pos, timer: 0.0 , can: false}
    }
    fn draw(&self, canvas: &mut graphics::Canvas, solo:bool) {
        if self.can{
            if solo{
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(Rect::new(
                            ((self.pos.x*40) + 350) as f32,
                            (self.pos.y * 40 ) as f32,
                            40.0,
                            40.0,
                        ))
                        .color(Color::RED), );
            }else{
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(Rect::new(
                            (self.pos.x * 30) as f32,
                            (self.pos.y * 40 ) as f32,
                            30.0,
                            40.0,
                        ))
                        .color(Color::RED), );
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(Rect::new(
                            ((self.pos.x * 30) + 1000) as f32,
                            (self.pos.y * 40) as f32,
                            30.0,
                            40.0,
                        ))
                        .color(Color::RED), );
            }
        }

    }
    fn boom() {
        //TODO
    }
    fn update(&mut self, can : bool){
        self.can = can;
    }
}

struct Player {
    pos: GridPosition,
    can:bool,
}

impl Player {
    pub fn new(pos: GridPosition) -> Self {
        Player { pos, can: false}
    }
    fn draw(&self, canvas: &mut graphics::Canvas, solo:bool,multi:bool) {
        if self.can{
            if solo{
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(Rect::new(
                            ((self.pos.x * 40) + 350) as f32,
                            (self.pos.y * 40)  as f32,
                            40.0,
                            40.0,
                        ))
                        .color(Color::GREEN), );
            }else if multi{
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(Rect::new(
                            ((self.pos.x * 30) +1000) as f32,
                            (self.pos.y * 40) as f32,
                            30.0,
                            40.0,
                        ))
                        .color(Color::GREEN), );
            }else{
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(Rect::new(
                            (self.pos.x * 30) as f32,
                            (self.pos.y * 40)  as f32,
                            30.0,
                            40.0,
                        ))
                        .color(Color::GREEN), );
            }

        }

    }
    fn go(&mut self, dir: Direction, solo:bool) {
        if solo{
            match dir {
                Direction::Up => self.pos.y -= 1,
                Direction::Down => self.pos.y += 1,
                Direction::Left => self.pos.x -= 1,
                Direction::Right => self.pos.x += 1,
                _ => {}
            }
        }else{
            match dir {
                Direction::Up => self.pos.y -= 1,
                Direction::Down => self.pos.y += 1,
                Direction::Left => self.pos.x -= 1,
                Direction::Right => self.pos.x += 1,
                _ => {}
            }
        }

    }
    fn update(&mut self, can : bool){
        self.can = can;
    }
}

struct Wall {
    pos: GridPosition,
    can:bool,
}

struct GridPosition {
    x: i16,
    y: i16,
}

struct Entity<'a> {
    pos: &'a GridPosition,
}

impl Into<GridPosition> for (i16, i16) {
    fn into(self) -> GridPosition {
        GridPosition { x: self.0, y: self.1 }
    }
}

impl GridPosition {
    pub fn new(x: i16, y: i16) -> Self {
        GridPosition { x, y }
    }
}

impl Wall {
    pub fn new(pos: GridPosition,can:bool) -> Self {
        Wall { pos, can }
    }
    fn update(&mut self, can : bool){
        self.can = can;
    }
    fn draw(&self, canvas: &mut graphics::Canvas, map: &Vec<Vec<char>>,solo:bool) {
        if self.can{
            if solo{
                for i in 0..MAP_SIZE {
                    for j in 0..MAP_SIZE {
                        if map[i][j] == '#' {
                            canvas.draw(
                                &graphics::Quad,
                                graphics::DrawParam::new()
                                    .dest_rect(Rect::new(
                                        ((i * 40) + 350) as f32,
                                        (j * 40) as f32,
                                        40.0,
                                        40.0,
                                    ))
                                    .color(Color::BLACK), );
                        }

                    }
                }
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(Rect::new(
                            0.0 as f32,
                            0.0  as f32,
                            350.0,
                            1200.0,
                        ))
                        .color(Color::BLACK), );
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(Rect::new(
                            1550.0 as f32,
                            0.0  as f32,
                            350.0,
                            1200.0,
                        ))
                        .color(Color::BLACK), );
            }else{
                for i in 0..MAP_SIZE {
                    for j in 0..MAP_SIZE {
                        if map[i][j] == '#' {
                            canvas.draw(
                                &graphics::Quad,
                                graphics::DrawParam::new()
                                    .dest_rect(Rect::new(
                                        (i * 30) as f32,
                                        (j * 40) as f32,
                                        30.0,
                                        40.0,
                                    ))
                                    .color(Color::BLACK), );
                        }

                    }
                }
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(Rect::new(
                            900.0 as f32,
                            0.0  as f32,
                            100.0,
                            1200.0,
                        ))
                        .color(Color::BLACK), );
                for i in 0..MAP_SIZE {
                    for j in 0..MAP_SIZE {
                        if map[i][j] == '#' {
                            canvas.draw(
                                &graphics::Quad,
                                graphics::DrawParam::new()
                                    .dest_rect(Rect::new(
                                        ((i * 30) + 1000) as f32,
                                        (j * 40) as f32,
                                        30.0,
                                        40.0,
                                    ))
                                    .color(Color::BLACK), );
                        }

                    }
                }
            }

        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        while _ctx.time.check_update_time(DESIRED_FPS) {
            if !self.draw_menu.in_menu {
                self.solo = self.draw_menu.solo;
                if !self.solo{
                    self.mulit_player.update(true);
                }
                self.wall.update(true);
                self.player.update(true);
                self.exit.update(true);
                self.bomb.update(true);
            }
        }


        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        self.wall.draw(&mut canvas, &self.map, self.solo);
        self.player.draw(&mut canvas, self.solo, false);
        self.mulit_player.draw(&mut canvas, self.solo, true);
        self.exit.draw(&mut canvas, self.solo);
        self.bomb.draw(&mut canvas, self.solo);
        self.draw_menu.draw(&mut canvas);
        canvas.finish(ctx)?;

        Ok(())
    }
    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeated: bool) -> Result<(), GameError> {
        if let Some(dir) = input.keycode.and_then(Direction::from_keycode) {
            if !self.draw_menu.in_menu {
                self.player.go(dir, self.solo);
            }else{
                self.draw_menu.go(dir);
            }

        }
        Ok(())
    }
}
// ----------------  TCP socket  --------------
struct TcpServer {
    listener: TcpListener,
}

impl TcpServer {
    fn new(addr: &str) -> Result<Self, std::io::Error> {
        let listener = TcpListener::bind(addr)?;
        Ok(Self { listener })
    }

    fn run(&self) -> std::io::Result<()> {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.handle_client(stream);
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        Ok(())
    }

    fn handle_client(&self, stream: TcpStream) {
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut writer = stream.try_clone().unwrap();

        loop {
            let mut input = String::new();
            reader.read_line(&mut input).unwrap();

            let trimmed = input.trim();
            println!("Received: {}", trimmed);

            if trimmed == "exit" {
                writer.write_all(b"Goodbye!\n").unwrap();
                break;
            }

            writer.write_all(trimmed.as_bytes()).unwrap();
            writer.write_all(b"\n").unwrap();
        }
    }
}


fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .window_mode(ggez::conf::WindowMode::default().dimensions(1900.0, 1200.0))
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = unsafe { MyGame::new(&mut ctx) };

    // Run!
    event::run(ctx, event_loop, my_game);

    // 로컬호스트 사용해서 테스트
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let server = TcpServer { listener: listener.try_clone().unwrap() };
                thread::spawn(move || {
                    server.handle_client(stream);
                });
                println!("Listening OK")
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

}