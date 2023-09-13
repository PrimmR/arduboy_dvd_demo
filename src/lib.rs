#![no_std]
#![allow(non_upper_case_globals)]

//Include the Arduboy Library
use arduboy_rust::arduino;
#[allow(unused_imports)]
use arduboy_rust::prelude::*;
#[allow(dead_code)]
const arduboy: Arduboy2 = Arduboy2::new();

#[derive(Copy, Clone)]
struct Position(i16, i16);

#[derive(Copy, Clone)]
enum Direction {
    NE,
    SE,
    SW,
    NW,
}

impl Direction {
    fn new() -> Self {
        match arduino::random_less_than(4) {
            0 => Direction::NE,
            1 => Direction::SE,
            2 => Direction::SW,
            _ => Direction::NW,
        }
    }

    fn reflect_v(self) -> Self {
        match self {
            Direction::NE => Direction::SE,
            Direction::SE => Direction::NE,
            Direction::SW => Direction::NW,
            Direction::NW => Direction::SW,
        }
    }

    fn reflect_h(self) -> Self {
        match self {
            Direction::NE => Direction::NW,
            Direction::SE => Direction::SW,
            Direction::SW => Direction::SE,
            Direction::NW => Direction::NE,
        }
    }
}

fn movement(pos: Position, dir: &Direction) -> Position {
    match dir {
        Direction::NE => Position(pos.0 + 1, pos.1 - 1),
        Direction::SE => Position(pos.0 + 1, pos.1 + 1),
        Direction::SW => Position(pos.0 - 1, pos.1 + 1),
        Direction::NW => Position(pos.0 - 1, pos.1 - 1),
    }
}

fn boing(pos: Position, dir: Direction) -> Direction {
    let mut mut_dir = dir;
    // Horizontal bounce
    if pos.0 == 0 || pos.0 + DVD_WIDTH as i16 == WIDTH as i16 {
        mut_dir = mut_dir.reflect_h();
    }
    // Vertical bounce
    if pos.1 == 0 || pos.1 + DVD_HEIGHT as i16 == HEIGHT as i16 {
        mut_dir = mut_dir.reflect_v();
    }

    mut_dir
}

// Constants
const DVD_WIDTH: u8 = 31;
const DVD_HEIGHT: u8 = 16;

// Progmem data
progmem!(
    static DVD: [u8; _] = [
        31, 16, // width, height,
        0xe0, 0xfb, 0xfb, 0xbb, 0x83, 0x83, 0xc3, 0xe7, 0xff, 0x7f, 0x3f, 0x07, 0x3f, 0xff, 0xf8,
        0xe0, 0xf0, 0x3c, 0x1e, 0x0f, 0xe7, 0xfb, 0xfb, 0x9b, 0x83, 0x83, 0xc3, 0xe7, 0xff, 0x7e,
        0x3c, 0x21, 0x71, 0x71, 0x71, 0x71, 0x79, 0x78, 0xf8, 0xf8, 0xf8, 0xf8, 0xd8, 0xd8, 0xd9,
        0xdb, 0xd9, 0xd8, 0xd8, 0xf8, 0xf8, 0xf9, 0xf9, 0xf9, 0x79, 0x79, 0x71, 0x70, 0x70, 0x70,
        0x20, 0x00,
    ];
);

// dynamic ram variables
static mut position: Position = Position(0, 0);
static mut direction: Direction = Direction::SE;

// The setup() function runs once when you turn your Arduboy on
#[no_mangle]
pub unsafe extern "C" fn setup() {
    // put your setup code here, to run once:
    arduboy.begin();
    arduboy.init_random_seed();
    arduboy.set_frame_rate(30);
    arduboy.clear();

    position = Position(
        random_between(1, (WIDTH - DVD_WIDTH - 1).into()) as i16,
        random_between(1, (HEIGHT - DVD_HEIGHT - 1).into()) as i16,
    );
    direction = Direction::new();
}

// The loop() function repeats forever after setup() is done
#[no_mangle]
#[export_name = "loop"]
pub unsafe extern "C" fn loop_() {
    // put your main code here, to run repeatedly:
    if !arduboy.next_frame() {
        return;
    }
    arduboy.clear();

    direction = boing(position, direction);
    position = movement(position, &direction);
    sprites::draw_override(position.0, position.1, get_sprite_addr!(DVD), 0);

    arduboy.display();
}
