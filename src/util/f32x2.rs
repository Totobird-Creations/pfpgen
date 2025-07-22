use core::ops::{ Add, AddAssign, Sub, SubAssign, Mul, MulAssign };


#[derive(Clone, Copy)]
pub struct F32x2 {
    pub x : f32,
    pub y : f32
}

impl F32x2 {

    pub const ZERO   : Self = Self { x : 0.0, y : 0.0 };
    pub const CENTRE : Self = Self { x : 0.5, y : 0.5 };

    pub fn dist_squared(self, other : Self) -> f32 {
        let d = other - self;
        (d.x * d.x) + (d.y * d.y)
    }

    pub fn dist(self, other : Self) -> f32 {
        self.dist_squared(other).sqrt()
    }

    pub fn len(self) -> f32 {
        self.dist(Self::ZERO)
    }

    pub fn atan2(self) -> f32 {
        self.y.atan2(self.x)
    }

    pub fn fract(self) -> Self {
        Self {
            x : self.x.fract(),
            y : self.y.fract()
        }
    }

    pub fn floor(self) -> Self {
        Self {
            x : self.x.floor(),
            y : self.y.floor()
        }
    }

}

impl Add for F32x2 {
    type Output = Self;
    fn add(self, rhs : Self) -> Self::Output {
        Self {
            x : self.x + rhs.x,
            y : self.y + rhs.y
        }
    }
}
impl AddAssign for F32x2 {
    fn add_assign(&mut self, rhs : Self) {
        *self = *self + rhs;
    }
}

impl Sub for F32x2 {
    type Output = Self;
    fn sub(self, rhs : Self) -> Self::Output {
        Self {
            x : self.x - rhs.x,
            y : self.y - rhs.y
        }
    }
}
impl SubAssign for F32x2 {
    fn sub_assign(&mut self, rhs : Self) {
        *self = *self - rhs;
    }
}

impl Mul for F32x2 {
    type Output = Self;
    fn mul(self, rhs : Self) -> Self::Output {
        Self {
            x : self.x * rhs.x,
            y : self.y * rhs.y
        }
    }
}
impl MulAssign for F32x2 {
    fn mul_assign(&mut self, rhs : Self) {
        *self = *self * rhs;
    }
}

impl Mul<f32> for F32x2 {
    type Output = Self;
    fn mul(self, rhs : f32) -> Self::Output {
        Self {
            x : self.x * rhs,
            y : self.y * rhs
        }
    }
}
impl MulAssign<f32> for F32x2 {
    fn mul_assign(&mut self, rhs : f32) {
        *self = *self * rhs;
    }
}
