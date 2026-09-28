use std::io::{self, Write};
use crossterm::{cursor, queue, style::{Print, PrintStyledContent, Stylize}, terminal};
use crate::game::Game;
use crate::game::ground::Ground;
use crate::game::pixel::Pixel;

pub struct Renderer<W: Write> {
    out: W,
    next: Ground,
    prev: Vec<Option<Pixel>>,
}

impl<W: Write> Renderer<W> {
    pub fn new(out: W, width: usize, height: usize) -> Self {
        Self {
            out,
            next: Ground::new(width, height),
            prev: vec![None; width * height],
        }
    }

    pub fn draw(&mut self, game: &Game) -> io::Result<()> {
        self.next.clear();
        for &(x, y) in game.snake.iter().skip(1) {
            self.next.set(x, y, Pixel::Skin);
        }
        let (hx, hy) = game.snake.head();
        self.next.set(hx, hy, Pixel::Head);
        let (ax, ay) = game.apple;
        self.next.set(ax, ay, Pixel::Apple);

        let width = self.next.width();
        queue!(self.out, terminal::BeginSynchronizedUpdate)?;
        for (i, &pixel) in self.next.pixels().iter().enumerate() {
            if self.prev[i] == Some(pixel) {
                continue;
            }
            let x = (i % width) as u16 * 2;
            let y = (i / width) as u16;
            queue!(self.out, cursor::MoveTo(x, y), Print(pixel))?;
            self.prev[i] = Some(pixel);
        }
        queue!(self.out, terminal::EndSynchronizedUpdate)?;

        self.out.flush()
    }

    pub fn draw_message(&mut self, lines: &[String]) -> io::Result<()> {
        let text_width = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
        let box_width = (text_width + 4 + 1) & !1;
        let screen_cols = self.next.width() * 2;
        let screen_rows = self.next.height();

        let x = (screen_cols.saturating_sub(box_width) / 2) & !1;
        let top = (screen_rows / 2).saturating_sub(lines.len() / 2 + 1);

        let empty = String::new();
        let rows = std::iter::once(&empty).chain(lines.iter()).chain(std::iter::once(&empty));
        for (i, line) in rows.enumerate() {
            let len = line.chars().count();
            let left = (box_width - len) / 2;
            let right = box_width - len - left;
            let padded = format!("{}{}{}", " ".repeat(left), line, " ".repeat(right));
            queue!(
                self.out,
                cursor::MoveTo(x as u16, (top + i) as u16),
                PrintStyledContent(padded.black().on_white()),
            )?;
        }
        self.out.flush()?;

        self.invalidate();
        Ok(())
    }

    pub fn invalidate(&mut self) {
        self.prev.fill(None);
    }
}