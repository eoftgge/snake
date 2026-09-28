use rand::Rng;
use crate::game::snake::{Direction, Snake};

pub mod difficulty;
pub mod ground;
pub mod pixel;
pub mod snake;

pub enum TickResult {
    Running,
    Lost,
    Won,
}

pub const START_LEN: usize = 2;

pub struct Game {
    pub snake: Snake,
    pub apple: (usize, usize),
    pub width: usize,
    pub height: usize,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        assert!(width >= START_LEN + 1 && height >= 1, "the ground is too small");

        let center = (width / 2, height / 2);
        let snake = Snake::new(center, Direction::Right, START_LEN);
        let apple = spawn_apple(&snake, width, height).expect("the ground is too small");

        Self { snake, apple, width, height }
    }

    pub fn score(&self) -> u32 {
        (self.snake.len() - START_LEN) as u32
    }

    pub fn tick(&mut self) -> TickResult {
        let next = self.snake.next_head(self.width, self.height);
        let grow = next == self.apple;
        if self.snake.hits_self(next, grow) {
            return TickResult::Lost;
        }
        self.snake.step(next, grow);

        if grow {
            if self.snake.len() == self.width * self.height {
                return TickResult::Won;
            }
            self.apple = spawn_apple(&self.snake, self.width, self.height)
                .expect("free cell exists while snake doesn't fill the ground");
        }
        TickResult::Running
    }
}

fn spawn_apple(snake: &Snake, width: usize, height: usize) -> Option<(usize, usize)> {
    let mut rng = rand::thread_rng();
    for _ in 0..32 {
        let pos = (rng.gen_range(0..width), rng.gen_range(0..height));
        if !snake.iter().any(|&p| p == pos) {
            return Some(pos);
        }
    }
    let free: Vec<_> = (0..height)
        .flat_map(|y| (0..width).map(move |x| (x, y)))
        .filter(|pos| !snake.iter().any(|p| p == pos))
        .collect();
    if free.is_empty() {
        None
    } else {
        Some(free[rng.gen_range(0..free.len())])
    }
}