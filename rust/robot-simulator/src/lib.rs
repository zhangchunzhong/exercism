// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
#[derive(Clone)]
pub struct Robot {
    posx: i32,
    posy: i32,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            posx: x,
            posy: y,
            direction: d,
        }
    }

    pub fn turn_right(self) -> Self {
        match self.direction {
            Direction::North => Robot {
                posx: self.posx,
                posy: self.posy,
                direction: Direction::East,
            },
            Direction::East => Robot {
                posx: self.posx,
                posy: self.posy,
                direction: Direction::South,
            },
            Direction::South => Robot {
                posx: self.posx,
                posy: self.posy,
                direction: Direction::West,
            },
            Direction::West => Robot {
                posx: self.posx,
                posy: self.posy,
                direction: Direction::North,
            },
        }
    }

    pub fn turn_left(self) -> Self {
        match self.direction {
            Direction::North => Robot {
                posx: self.posx,
                posy: self.posy,
                direction: Direction::West,
            },
            Direction::East => Robot {
                posx: self.posx,
                posy: self.posy,
                direction: Direction::North,
            },
            Direction::South => Robot {
                posx: self.posx,
                posy: self.posy,
                direction: Direction::East,
            },
            Direction::West => Robot {
                posx: self.posx,
                posy: self.posy,
                direction: Direction::South,
            },
        }
    }

    pub fn advance(self) -> Self {
        match self.direction {
            Direction::North => Robot {
                posx: self.posx,
                posy: self.posy + 1,
                direction: Direction::North,
            },
            Direction::East => Robot {
                posx: self.posx + 1,
                posy: self.posy,
                direction: Direction::East,
            },
            Direction::South => Robot {
                posx: self.posx,
                posy: self.posy - 1,
                direction: Direction::South,
            },
            Direction::West => Robot {
                posx: self.posx - 1,
                posy: self.posy,
                direction: Direction::West,
            },
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut ret =  Robot {
            posx: self.posx,
            posy: self.posy,
            direction: self.direction
        };
        for ch in instructions.chars() {
            ret = match ch {
                'L' => ret.turn_left(),
                'R' => ret.turn_right(),
                'A' => ret.advance(),
                _ => ret
            }
        }
        ret
    }

    pub fn position(&self) -> (i32, i32) {
        (self.posx, self.posy)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
