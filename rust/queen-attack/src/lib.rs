#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    pos:ChessPosition
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank >= 8 || file >= 8 {
            None
        } else if rank < 0 || file < 0 {
            None
        } else {
            Some(ChessPosition {
                rank,
                file,
            })
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { pos:position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        if self.pos.file == other.pos.file {
            true
        } else if self.pos.rank == other.pos.rank {
            true
        } else {
            let m = self.pos.rank - self.pos.file;
            let n = self.pos.rank + self.pos.file;
            if other.pos.rank == other.pos.file + m {
                true
            } else if other.pos.rank + other.pos.file == n {
                true
            } else {
                false
            }
        }
    }
}
