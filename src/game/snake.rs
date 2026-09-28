use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

pub struct Snake {
    body: VecDeque<(usize, usize)>, // front = head
    direction: Direction,
    next_direction: Direction,
}

impl Snake {
    pub fn new(head: (usize, usize), direction: Direction, len: usize) -> Self {
        assert!(len >= 1, "snake couldn't be empty");

        let (dx, dy): (isize, isize) = match direction {
            Direction::Right => (-1, 0),
            Direction::Left => (1, 0),
            Direction::Down => (0, -1),
            Direction::Up => (0, 1),
        };

        let body = (0..len as isize)
            .map(|i| {
                let x = head.0 as isize + dx * i;
                let y = head.1 as isize + dy * i;
                (x as usize, y as usize)
            })
            .collect();

        Self { body, direction, next_direction: direction }
    }

    pub fn head(&self) -> (usize, usize) {
        self.body[0]
    }

    pub fn iter(&self) -> impl Iterator<Item = &(usize, usize)> {
        self.body.iter()
    }

    pub fn len(&self) -> usize {
        self.body.len()
    }

    pub fn turn(&mut self, dir: Direction) {
        if self.body.len() == 1 || dir != self.direction.opposite() {
            self.next_direction = dir;
        }
    }

    pub fn next_head(&self, width: usize, height: usize) -> (usize, usize) {
        let (x, y) = self.head();
        match self.next_direction {
            Direction::Up => (x, (y + height - 1) % height),
            Direction::Down => (x, (y + 1) % height),
            Direction::Left => ((x + width - 1) % width, y),
            Direction::Right => ((x + 1) % width, y),
        }
    }

    pub fn hits_self(&self, pos: (usize, usize), grow: bool) -> bool {
        let skip_tail = if grow { 0 } else { 1 };
        self.body
            .iter()
            .take(self.body.len() - skip_tail)
            .any(|&p| p == pos)
    }

    pub fn step(&mut self, new_head: (usize, usize), grow: bool) {
        self.direction = self.next_direction;
        self.body.push_front(new_head);
        if !grow {
            self.body.pop_back();
        }
    }
}
