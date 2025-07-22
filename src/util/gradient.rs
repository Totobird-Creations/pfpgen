use crate::util::Rgba;
use std::borrow::Cow;
use float_cmp::{ ApproxEq, F32Margin };


#[derive(Clone, Copy)]
pub struct GradientPoint {
    pub x   : f32,
    pub col : Rgba
}


pub struct Gradient<'l> {
    pub points : Cow<'l, [GradientPoint]>
}

impl Gradient<'_> {

    pub fn sample(&self, t : f32) -> Rgba {
        let t           = t.clamp(0.0, 1.0);
        let point_count = self.points.len();
        match (point_count) {
            0   => Rgba::BLACK,
            1   => self.points[0].col,
            2.. => {
                let i = self.index_at(t);
                if (i == (point_count - 1)) {
                    self.points[i].col
                } else {
                    let local = t - self.points[i].x;
                    if ((i == 0) && (local <= 0.0)) {
                        self.points[0].col
                    } else {
                        unsafe { self.sample_local_unchecked(i, local) }
                    }
                }
            }
        }
    }

    pub fn index_at(&self, t : f32) -> usize {
        let     t    = t.clamp(0.0, 1.0);
        let mut imin = 0;
        let mut imax = self.points.len() - 1;
        while ((imax - imin) > 1) {
            let mid = (imin + imax) / 2;
            let a   = self.points[mid].x;
            let b   = self.points[mid + 1].x;
            if ((a < t) && (b < t)) {
                imin = mid;
            } else if (a > t) {
                imax = mid;
            } else {
                return mid;
            }
        }
        if (t > self.points[imax].x) { imax } else { imin }
    }

    pub unsafe fn sample_local_unchecked(&self, i : usize, t_local : f32) -> Rgba {
        let a  = self.points[i];
        let b  = self.points[i + 1];
        let dx = b.x - a.x;
        if (dx.approx_eq(0.0, F32Margin::default())) {
            b.col
        } else {
            let t_local = t_local / dx;

            a.col + ((b.col - a.col) * t_local)
        }
    }

}
