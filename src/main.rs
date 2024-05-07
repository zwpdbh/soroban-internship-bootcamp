use std::fmt;
use std::fmt::Display;

enum VertDir {
    Up,
    Down,
}

enum HorizDir {
    Left,
    Right,
}

struct Ball {
    x: i32,
    y: i32,
    ver_dir: VertDir,
    horiz_dir: HorizDir,
}

struct Frame {
    width: i32,
    height: i32,
}

struct Game {
    frame: Frame,
    ball: Ball,
}

impl Game {
    fn new() -> Self {
        Game {
            frame: Frame {
                width: 63,
                height: 31,
            },
            ball: Ball {
                x: 44,
                y: 21,
                ver_dir: VertDir::Down,
                horiz_dir: HorizDir::Right,
            },
        }
    }

    fn step(&mut self) {
        self.ball.bounce(&self.frame);
        self.ball.mv();
    }
}

impl Ball {
    fn bounce(&mut self, frame: &Frame) {
        if self.x <= 0 {
            self.horiz_dir = HorizDir::Right;
        } else if frame.width <= self.x {
            self.horiz_dir = HorizDir::Left;
        } else if self.y < 0 {
            self.ver_dir = VertDir::Down;
        } else if frame.height <= self.y {
            self.ver_dir = VertDir::Up;
        } else {
        }
    }

    fn mv(&mut self) {
        match self.horiz_dir {
            HorizDir::Left => self.x -= 1,
            HorizDir::Right => self.x += 1,
        }

        match self.ver_dir {
            VertDir::Up => self.y -= 1,
            VertDir::Down => self.y += 1,
        }
    }
}

#[allow(unused_must_use)]
impl Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x");
        for _ in 0..64 {
            write!(f, "-");
        }
        for y in 0..32 {
            for x in 0..64 {
                if self.ball.x == x as i32 && self.ball.y == y as i32 {
                    write!(f, "0");
                }
                if x == 0 {
                    write!(f, "|");
                } else if x == 62 && self.ball.y == y {
                    // do nothing such that to avoid repeated drawing
                } else if x == 63 {
                    write!(f, "|");
                } else if x != 0 && y != 31 {
                    write!(f, " ");
                } else {
                    write!(f, "-");
                }
            }
            write!(f, "\n");
        }
        write!(f, "\n")
    }
}

fn main() {
    let mut new_game = Game::new();
    let sleep_time = std::time::Duration::from_millis(32);
    loop {
        println!("{}", new_game);
        new_game.step();
        std::thread::sleep(sleep_time);
        println!("{} {}", new_game.ball.x, new_game.ball.y);
    }
}
