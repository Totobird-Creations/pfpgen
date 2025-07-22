use core::ops::{ Add, AddAssign, Sub, SubAssign, Mul, MulAssign };


#[derive(Clone, Copy)]
pub struct Rgba {
    pub r : f32,
    pub g : f32,
    pub b : f32,
    pub a : f32
}

impl Rgba {

    pub const BLACK       : Self = Self { r : 0.0, g : 0.0, b : 0.0, a : 1.0 };
    pub const TRANSPARENT : Self = Self { r : 0.0, g : 0.0, b : 0.0, a : 0.0 };

    pub fn mix(self, other : Self, t : f32) -> Self {
        Self {
            r : self.r + ((other.r - self.r) * t),
            g : self.g + ((other.g - self.g) * t),
            b : self.b + ((other.b - self.b) * t),
            a : self.a + ((other.a - self.a) * t)
        }
    }

}

impl Add for Rgba {
    type Output = Self;
    fn add(self, rhs : Self) -> Self::Output {
        Self {
            r : self.r + rhs.r,
            g : self.g + rhs.g,
            b : self.b + rhs.b,
            a : self.a + rhs.a
        }
    }
}
impl AddAssign for Rgba {
    fn add_assign(&mut self, rhs : Self) {
        *self = *self + rhs;
    }
}

impl Add<f32> for Rgba {
    type Output = Self;
    fn add(self, rhs : f32) -> Self::Output {
        Self {
            r : self.r + rhs,
            g : self.g + rhs,
            b : self.b + rhs,
            a : self.a + rhs
        }
    }
}
impl AddAssign<f32> for Rgba {
    fn add_assign(&mut self, rhs : f32) {
        *self = *self + rhs;
    }
}

impl Sub for Rgba {
    type Output = Self;
    fn sub(self, rhs : Self) -> Self::Output {
        Self {
            r : self.r - rhs.r,
            g : self.g - rhs.g,
            b : self.b - rhs.b,
            a : self.a - rhs.a
        }
    }
}
impl SubAssign for Rgba {
    fn sub_assign(&mut self, rhs : Self) {
        *self = *self - rhs;
    }
}

impl Mul for Rgba {
    type Output = Self;
    fn mul(self, rhs : Self) -> Self::Output {
        Self {
            r : self.r * rhs.r,
            g : self.g * rhs.g,
            b : self.b * rhs.b,
            a : self.a * rhs.a
        }
    }
}
impl MulAssign for Rgba {
    fn mul_assign(&mut self, rhs : Self) {
        *self = *self * rhs;
    }
}

impl Mul<f32> for Rgba {
    type Output = Self;
    fn mul(self, rhs : f32) -> Self::Output {
        Self {
            r : self.r * rhs,
            g : self.g * rhs,
            b : self.b * rhs,
            a : self.a * rhs
        }
    }
}
impl MulAssign<f32> for Rgba {
    fn mul_assign(&mut self, rhs : f32) {
        *self = *self * rhs;
    }
}

impl From<Rgba> for image::Rgba<f32> {
    fn from(value : Rgba) -> Self {
        Self([ value.r, value.g, value.b, value.a ])
    }
}
