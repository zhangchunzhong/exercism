use std::ops::Add;

/* 
#[cfg(not(generic))]
pub struct Triangle {
    s1: u64,
    s2: u64,
    s3: u64,
}

#[cfg(not(generic))]
impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if sides[0] == 0 || sides[1] == 0 || sides[2] == 0 {
            None
        } else if sides[0] + sides[1] <= sides[2]
            || sides[1] + sides[2] <= sides[0]
            || sides[0] + sides[2] <= sides[1]
        {
            None
        } else {
            Some(Triangle {
                s1: sides[0],
                s2: sides[1],
                s3: sides[2],
            })
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.s1 == self.s2 && self.s2 == self.s3
    }

    pub fn is_scalene(&self) -> bool {
        self.s1 != self.s2 && self.s1 != self.s3 && self.s2 != self.s3
    }

    pub fn is_isosceles(&self) -> bool {
        if self.s1 == self.s2 && self.s2 == self.s3 {
            false
        } else {
            self.s1 == self.s2 || self.s2 == self.s3 || self.s1 == self.s3
        }
    }
}
*/


pub struct Triangle <T> {
    s1: T,
    s2: T,
    s3: T,
}

impl<T> Triangle<T>
where
    T: PartialEq + PartialOrd + Add<Output = T> + Default + Copy,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        if sides[0] == T::default() || sides[1] == T::default() || sides[2] == T::default() {
            None
        } else if sides[0] + sides[1] <= sides[2]
            || sides[1] + sides[2] <= sides[0]
            || sides[0] + sides[2] <= sides[1]
        {
            None
        } else {
            Some(Triangle {
                s1: sides[0],
                s2: sides[1],
                s3: sides[2],
            })
        }
    }
    pub fn is_equilateral(&self) -> bool {
        self.s1 == self.s2 && self.s2 == self.s3
    }
    pub fn is_scalene(&self) -> bool {
        self.s1 != self.s2 && self.s1 != self.s3 && self.s2 != self.s3
    }
    pub fn is_isosceles(&self) -> bool {
        if self.s1 == self.s2 && self.s2 == self.s3 {
            false
        } else {
            self.s1 == self.s2 || self.s2 == self.s3 || self.s1 == self.s3
        }
    }
}

