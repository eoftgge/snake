use super::pixel::Pixel;

pub struct Ground {
    pixels: Vec<Pixel>,
    width: usize,
    height: usize,
}

impl Ground {
    pub fn new(width: usize, height: usize) -> Self {
        assert!(width > 0 && height > 0, "ground must not be empty");
        Self { pixels: vec![Pixel::Empty; width * height], width, height }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Pixel> {
        if x >= self.width || y >= self.height { return None; }
        self.pixels.get(y * self.width + x)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Pixel> {
        if x >= self.width || y >= self.height { return None; }
        self.pixels.get_mut(y * self.width + x)
    }

    pub fn set(&mut self, x: usize, y: usize, pixel: Pixel) {
        if let Some(p) = self.get_mut(x, y) {
            *p = pixel;
        }
    }

    pub fn pixels(&self) -> &[Pixel] {
        &self.pixels
    }

    pub fn rows(&self) -> impl Iterator<Item = &[Pixel]> {
        self.pixels.chunks(self.width)
    }

    pub fn clear(&mut self) {
        self.pixels.fill(Pixel::Empty)
    }
}