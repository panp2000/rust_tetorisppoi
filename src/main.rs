#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use sdl2::event::Event;
use sdl2::keyboard::{KeyboardState, Keycode, Scancode};
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;

use rand::Rng;
use std::fmt::Debug;
use std::time::Duration;

const BLOCK_SRC_SIZE: u32 = 60;
const BLOCK_DST_SIZE: u32 = 30;
const WINDOW_WIDTH: u32 = BLOCK_DST_SIZE * 10;
const WINDOW_HEIGHT: u32 = BLOCK_DST_SIZE * 20;

const TICK: usize = 10;

const BLOCK_IMAGE: &[u8] = include_bytes!("../img/blocks.png");

#[derive(Debug, Clone, Copy)]
enum BlockType {
    B,
    I,
    O,
    Z,
    T,
    J,
    S,
    L,
}

impl BlockType {
    fn from_usize(n: usize) -> Option<Self> {
        match n {
            0 => Some(Self::B),
            1 => Some(Self::I),
            2 => Some(Self::O),
            3 => Some(Self::Z),
            4 => Some(Self::T),
            5 => Some(Self::L),
            6 => Some(Self::S),
            7 => Some(Self::J),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone)]
struct Block {
    rotate: usize,
    p: Vec<Position>,
}

impl Block {
    fn new(rotate: usize, p: Vec<Position>) -> Self {
        Self { rotate, p }
    }
}

#[derive(Debug, Clone, Copy)]
struct Status {
    x: i32,
    y: i32,
    block_type: BlockType,
    rotate: usize,
    counter: usize,
}

impl Status {
    fn new(x: i32, y: i32, block_type: BlockType, rotate: usize) -> Self {
        Self {
            x,
            y,
            block_type,
            rotate,
            counter: 0,
        }
    }
}

struct Game<'a> {
    block_texture: sdl2::render::Texture<'a>,
    blocks: Vec<Block>,
    board: Vec<Vec<Option<u8>>>,
    status: Status,
    game_over: bool,
}

impl<'a> Game<'a> {
    fn new(
        block_texture: sdl2::render::Texture<'a>,
        blocks: Vec<Block>,
        board: Vec<Vec<Option<u8>>>,
    ) -> Self {
        Self {
            block_texture,
            blocks,
            board,
            status: Status::new(6, 11, BlockType::S, 2),
            game_over: false,
        }
    }
}

fn main() -> Result<(), String> {
    let mut rng = rand::thread_rng();
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Title", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .expect("could not initisalize the video subsystem");
    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Failed to initialize canavasr");

    // ゲームの初期化
    // ブロックデータの読み込み
    let texture_creator = canvas.texture_creator();
    let img = image::load_from_memory(BLOCK_IMAGE).unwrap();
    let mut texture = texture_creator
        .create_texture_target(Some(PixelFormatEnum::RGB24), 60, 480)
        .unwrap();
    texture
        .update(Rect::new(0, 0, 60, 480), img.as_rgb8().unwrap(), 180)
        .unwrap();
    let mut game = Game::new(
        // texture_creator.load_texture("img/blocks.png")?,
        texture,
        // ブロックの初期化
        vec![
            Block::new(
                // blank
                1,
                vec![
                    Position::new(0, 0),
                    Position::new(0, 0),
                    Position::new(0, 0),
                ],
            ),
            Block::new(
                // I
                2,
                vec![
                    Position::new(0, -1),
                    Position::new(0, 1),
                    Position::new(0, 2),
                ],
            ),
            Block::new(
                // O
                1,
                vec![
                    Position::new(0, 1),
                    Position::new(1, 0),
                    Position::new(1, 1),
                ],
            ),
            Block::new(
                // Z
                2,
                vec![
                    Position::new(0, -1),
                    Position::new(1, 0),
                    Position::new(1, 1),
                ],
            ),
            Block::new(
                // T
                4,
                vec![
                    Position::new(0, -1),
                    Position::new(-1, 0),
                    Position::new(1, 0),
                ],
            ),
            Block::new(
                // J
                4,
                vec![
                    Position::new(-1, 0),
                    Position::new(1, 0),
                    Position::new(1, -1),
                ],
            ),
            Block::new(
                // S
                2,
                vec![
                    Position::new(0, 1),
                    Position::new(1, 0),
                    Position::new(1, -1),
                ],
            ),
            Block::new(
                // L
                4,
                vec![
                    Position::new(1, 0),
                    Position::new(-1, 0),
                    Position::new(-1, -1),
                ],
            ),
        ],
        vec![vec![None; 12]; 25], // ボード・ブロックの初期化
    );

    // ボードに外壁を設置
    for y in 0..25 {
        for x in 0..12 {
            if x == 0 || x == 11 || y == 0 {
                game.board[y][x] = Some(0);
            }
        }
    }

    // ボードにブロックを配置
    game.status = Status::new(
        5,
        21,
        BlockType::from_usize(rng.gen_range(1..=7) as usize).unwrap(),
        0,
    );

    let s = game.status;
    put_block(&mut game, &s, false).unwrap();

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        let mut current_status = game.status;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                } => {
                    current_status.rotate += 1;
                }
                _ => {}
            }
        }
        if game.game_over {
            continue 'running;
        }

        if game.status.counter % 2 == 0 {
            user_input_proceed(&mut current_status, event_pump.keyboard_state());
        }

        if current_status.counter % TICK == 0 {
            current_status.y -= 1;
        }

        if game.status.x != current_status.x || game.status.rotate != current_status.rotate {
            let n = game.status;
            delete_block(&mut game, &n);
            if put_block(&mut game, &current_status, false).is_ok() {
                game.status = current_status;
            } else {
                put_block(&mut game, &n, true).unwrap();
                current_status.x = game.status.x;
                current_status.rotate = game.status.rotate;
            }
        }

        if game.status.y != current_status.y {
            block_down(&mut game, &current_status);
        }

        render(&mut canvas, &game);
        game.status.counter += 1;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
    Ok(())
}

fn render(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, game: &Game) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    for (i, y) in game.board[1..=20].iter().enumerate() {
        for (j, &x) in y[1..=10].iter().enumerate() {
            if let Some(block) = x {
                canvas
                    .copy(
                        &game.block_texture,
                        Rect::new(
                            0,
                            BLOCK_SRC_SIZE as i32 * block as i32,
                            BLOCK_SRC_SIZE,
                            BLOCK_SRC_SIZE,
                        ),
                        Rect::new(
                            BLOCK_DST_SIZE as i32 * (j) as i32,
                            BLOCK_DST_SIZE as i32 * (19 - i) as i32,
                            BLOCK_DST_SIZE,
                            BLOCK_DST_SIZE,
                        ),
                    )
                    .unwrap();
            }
        }
    }

    canvas.present();
}

fn put_block(game: &mut Game, status: &Status, action: bool) -> Result<(), ()> {
    if game.board[status.y as usize][status.x as usize].is_some() {
        return Err(());
    }
    if action {
        game.board[status.y as usize][status.x as usize] = Some(status.block_type as u8);
    }
    let block = game.blocks[status.block_type as usize].clone();
    for i in block.p {
        let (mut dx, mut dy) = (i.x, i.y);
        let r = status.rotate % block.rotate;
        for _ in 0..r {
            let tmp = dx;
            dx = -dy;
            dy = tmp;
        }
        if game.board[(status.y + dy) as usize][(status.x + dx) as usize].is_some() {
            return Err(());
        }
        if action {
            game.board[(status.y + dy) as usize][(status.x + dx) as usize] =
                Some(game.status.block_type as u8);
        }
    }
    if !action {
        put_block(game, status, true).unwrap();
    }

    Ok(())
}

fn delete_block(game: &mut Game, status: &Status) {
    game.board[status.y as usize][status.x as usize] = None;
    let block = game.blocks[status.block_type as usize].clone();
    for i in block.p {
        let (mut dx, mut dy) = (i.x, i.y);
        let r = status.rotate % block.rotate;
        for _ in 0..r {
            let tmp = dx;
            dx = -dy;
            dy = tmp;
        }
        game.board[(status.y + dy) as usize][(status.x + dx) as usize] = None;
    }
}

fn user_input_proceed(current_status: &mut Status, keyboard_state: KeyboardState) {
    if keyboard_state.is_scancode_pressed(Scancode::Left) {
        current_status.x -= 1;
    } else if keyboard_state.is_scancode_pressed(Scancode::Right) {
        current_status.x += 1;
    } else if keyboard_state.is_scancode_pressed(Scancode::Down) {
        current_status.counter = 0;
    }
}

fn block_down(game: &mut Game, status: &Status) {
    let n = game.status;
    delete_block(game, &n);
    if put_block(game, status, false).is_ok() {
        game.status = *status;
    } else {
        put_block(game, &n, false).unwrap();

        delete_line(game);

        let mut rng = rand::thread_rng();

        game.status = Status::new(
            5,
            21,
            BlockType::from_usize(rng.gen_range(1..=7) as usize).unwrap(),
            0,
        );
        let n = game.status;
        if put_block(game, &n, false).is_err() {
            game_over(game);
        }
    }
}

fn game_over(game: &mut Game) {
    game.game_over = true;
    for i in &mut game.board {
        for j in i {
            if j.is_some() {
                *j = Some(0);
            }
        }
    }
}

fn delete_line(game: &mut Game) {
    for y in 1..23 {
        while game.board[y].iter().all(|&x| x.is_some()) {
            for j in y..23 {
                game.board[j] = game.board[j + 1].clone();
            }
        }
    }
}
