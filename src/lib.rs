#![no_std]
#![feature(lang_items)]

mod base;
mod gba;

use base::rand::Rand;
pub use base::rust_begin_unwind;

#[derive(Copy, Clone)]
enum Tile {
    Empty,
    Snake,
    Food,
}
const WIDTH: usize = 30;
const HEIGHT: usize = 20;
const MAX_LENGTH: usize = 100;

struct Arena {
    data: [Tile; WIDTH * HEIGHT],
}

impl Arena {
    pub fn new() -> Arena {
        Arena { data: [Tile::Empty; WIDTH * HEIGHT] }
    }

    pub fn set(&mut self, x: usize, y: usize, tile: Tile) {
        if x < WIDTH && y < HEIGHT {
            self.data[x + y * WIDTH] = tile;
            let bg_tile = match tile {
                Tile::Empty => 1u16,
                Tile::Snake => 0u16,
                Tile::Food => 1u16 << 12,
            };
            gba::hw::write_vram16((0x400 + x + y * 32) as u32, bg_tile);
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Tile {
        if x < WIDTH && y < HEIGHT {
            self.data[x + y * WIDTH]
        } else {
            Tile::Snake
        }
    }
}

#[derive(Copy, Clone)]
struct Pos {
    x: usize,
    y: usize,
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

struct Game {
    arena: Arena,
    pos: Pos,
    snake: [Pos; MAX_LENGTH],
    length: usize,
    target_length: usize,
    dir: Dir,
    rand: Rand,
    food_count: u32,
}

impl Game {
    fn new() -> Game {
        Game {
            arena: Arena::new(),
            pos: Pos { x: 15, y: 12 },
            snake: [Pos { x: 0, y: 0 }; MAX_LENGTH],
            length: 0,
            target_length: 5,
            dir: Dir::Up,
            rand: Rand::new(1234),
            food_count: 0,
        }
    }
    fn reset(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.arena.set(x, y, Tile::Empty);
            }
        }
        self.pos.x = WIDTH / 2;
        self.pos.y = HEIGHT / 2;
        self.length = 0;
        self.target_length = 5;
        self.dir = Dir::Up;
        self.food_count = 0;
        self.arena.set(self.pos.x, self.pos.y, Tile::Snake);
    }

    fn update(&mut self, key_state: &gba::KeyState) {
        if key_state.is_triggered(gba::Key::Up) {
            self.dir = Dir::Up
        }
        if key_state.is_triggered(gba::Key::Down) {
            self.dir = Dir::Down
        }
        if key_state.is_triggered(gba::Key::Left) {
            self.dir = Dir::Left
        }
        if key_state.is_triggered(gba::Key::Right) {
            self.dir = Dir::Right
        }
        self.snake[self.length].x = self.pos.x;
        self.snake[self.length].y = self.pos.y;
        if self.length < self.target_length {
            self.length += 1;
        } else {
            self.arena
                .set(self.snake[0].x, self.snake[0].y, Tile::Empty);
            for i in 0..self.length {
                self.snake[i].x = self.snake[i + 1].x;
                self.snake[i].y = self.snake[i + 1].y;
            }
        }
        let food_x = (self.rand.next_u8() & 31) as usize;
        let food_y = (self.rand.next_u8() & 31) as usize;
        if self.food_count < 4 && food_x < WIDTH && food_y < HEIGHT {
            match self.arena.get(food_x, food_y) {
                Tile::Empty => {
                    self.arena.set(food_x, food_y, Tile::Food);
                    self.food_count += 1;
                }
                _ => {}
            }
        }
        match self.dir {
            Dir::Up => self.pos.y = self.pos.y.wrapping_sub(1),
            Dir::Down => self.pos.y += 1,
            Dir::Left => self.pos.x = self.pos.x.wrapping_sub(1),
            Dir::Right => self.pos.x += 1,
        }
        match self.arena.get(self.pos.x, self.pos.y) {
            Tile::Snake => self.reset(),
            Tile::Food => {
                self.food_count -= 1;
                self.target_length += 5;
                if self.target_length > MAX_LENGTH - 1 {
                    self.target_length = MAX_LENGTH - 1;
                }
            }
            _ => {}
        };
        self.arena.set(self.pos.x, self.pos.y, Tile::Snake);
    }
}

#[no_mangle]
pub extern "C" fn main() {
    let mut key_state = gba::KeyState::new();
    gba::hw::write_dispcnt(1 << 8);
    gba::hw::write_bg0cnt(1 << 8);
    gba::hw::write_pal(15, 0x7fff);
    gba::hw::write_pal(31, 31 << 5);
    for i in 1..7 {
        gba::hw::write_vram16(i * 2, 0xfff0);
        gba::hw::write_vram16(i * 2 + 1, 0x0fff);
    }
    let mut game = Game::new();
    game.reset();
    loop {
        key_state.update();
        game.update(&key_state);
        for _ in 0..4 {
            gba::wait_vblank();
        }
    }
}
