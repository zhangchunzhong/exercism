#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Default)]
pub struct BowlingGame {
    frames: Vec<u16>,
    roll_again: bool,
}

impl BowlingGame {
    pub fn new() -> BowlingGame {
        BowlingGame {
            frames: Vec::new(),
            roll_again: false,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 || (self.roll_again && pins + self.frames.last().unwrap() > 10) {
            return Err(Error::NotEnoughPinsLeft);
        } else if self.score().is_some() {
            return Err(Error::GameComplete);
        }
        self.frames.push(pins);
        self.roll_again = match pins {
            10 => false,
            _ => !self.roll_again,
        };
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        let (mut total, mut idx) = (0, 0);
        for _ in 0..10 {
            if let (Some(&a), Some(&b)) = (&self.frames.get(idx), &self.frames.get(idx + 1)) {
                total += a + b;
                if a == 10 || a + b == 10 {
                    if let Some(&c) = &self.frames.get(idx + 2) {
                        total += c;
                    } else {
                        return None;
                    }
                }
                idx += if a == 10 { 1 } else { 2 };
            } else {
                return None;
            }
        }
        Some(total)
    }
}