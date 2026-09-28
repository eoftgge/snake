use std::time::Duration;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

impl Difficulty {
    pub fn name(self) -> &'static str {
        match self {
            Self::Easy => "easy",
            Self::Normal => "normal",
            Self::Hard => "hardeeest",
        }
    }

    fn params(self) -> (f64, f64, f64) {
        match self {
            Self::Easy => (200.0, 120.0, 0.98),
            Self::Normal => (150.0, 80.0, 0.97),
            Self::Hard => (100.0, 50.0, 0.96),
        }
    }

    pub fn tick(self, score: u32) -> Duration {
        let (start, min, factor) = self.params();
        let ms = (start * factor.powi(score as i32)).max(min);
        Duration::from_secs_f64(ms / 1000.0)
    }
}