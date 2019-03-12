extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

const WIN_SIZE: u32 = 820;
const TILE_SIZE: u32 = 20;
const BOARD_LEN: u32 = WIN_SIZE / TILE_SIZE;
const NUM_TILES: u32 = BOARD_LEN * BOARD_LEN;

#[derive(Clone)]
struct Tile {
    rect: Rect,
    is_black: bool,
}

enum Direction {
    North,
    South,
    East,
    West,
}

struct Ant {
    x: i32,
    y: i32,
    facing: Direction,
}

fn update_ant(ant: &mut Ant, tiles: &mut Vec<Vec<Tile>>) {
    //check the underlying tile for right or left turn
    let turn_left = tiles[ant.x as usize][ant.y as usize].is_black;
    //flip the tile
    tiles[ant.x as usize][ant.y as usize].is_black =
        !tiles[ant.x as usize][ant.y as usize].is_black;
    //turn based on direction
    match ant.facing {
        Direction::North => {
            if turn_left {
                ant.x -= 1;
                ant.facing = Direction::West;
            } else {
                ant.x += 1;
                ant.facing = Direction::East;
            }
        }
        Direction::South => {
            if turn_left {
                ant.x += 1;
                ant.facing = Direction::East;
            } else {
                ant.x -= 1;
                ant.facing = Direction::West;
            }
        }
        Direction::East => {
            if turn_left {
                ant.y += 1;
                ant.facing = Direction::North;
            } else {
                ant.y -= 1;
                ant.facing = Direction::South;
            }
        }
        Direction::West => {
            if turn_left {
                ant.y -= 1;
                ant.facing = Direction::South;
            } else {
                ant.y += 1;
                ant.facing = Direction::North;
            }
        }
    }
}

fn main() {
    //assert that constants are acceptable
    assert_eq!(WIN_SIZE % TILE_SIZE, 0);
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Ant Iron Man", WIN_SIZE, WIN_SIZE)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    //initial tile population
    let mut tiles: Vec<Vec<Tile>> = vec![
        vec![
            Tile {
                //set the height and width as 1 pixel less than the actual tile size so it looks
                //pretty
                rect: Rect::new(0, 0, TILE_SIZE - 1, TILE_SIZE - 1),
                is_black: false
            };
            BOARD_LEN as usize
        ];
        BOARD_LEN as usize
    ];
    for i in 0..BOARD_LEN {
        for j in 0..BOARD_LEN {
            tiles[i as usize][j as usize]
                .rect
                .set_x(i as i32 * TILE_SIZE as i32);
            tiles[i as usize][j as usize]
                .rect
                .set_y(j as i32 * TILE_SIZE as i32);
        }
    }

    //initial ant population
    let mut ant = Ant {
        x: (BOARD_LEN / 2) as i32,
        y: (BOARD_LEN / 2) as i32,
        facing: Direction::North,
    };

    //clear screen and initially draw the tiles
    let white = Color::RGB(255, 255, 255);
    let black = Color::RGB(0, 0, 0);
    let red = Color::RGB(255, 0, 0);
    let light_blue = Color::RGB(0, 255, 255);
    canvas.set_draw_color(white);
    canvas.clear();

    //main loop for the process
    'running: loop {
        canvas.set_draw_color(light_blue);
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        //Main calculations before showing the screen
        update_ant(&mut ant, &mut tiles);

        //draw the tiles
        let white_rects: Vec<Rect> = tiles
            .iter()
            .flatten()
            .filter(|&tile| !tile.is_black)
            .map(|tile| tile.rect)
            .collect();
        let black_rects: Vec<Rect> = tiles
            .iter()
            .flatten()
            .filter(|&tile| tile.is_black)
            .map(|tile| tile.rect)
            .collect();
        canvas.set_draw_color(white);
        canvas.fill_rects(&white_rects).unwrap();
        canvas.set_draw_color(black);
        canvas.fill_rects(&black_rects).unwrap();
        //draw the ant on top of whatever square it is
        canvas.set_draw_color(red);
        canvas
            .fill_rect(tiles[ant.x as usize][ant.y as usize].rect)
            .unwrap();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
