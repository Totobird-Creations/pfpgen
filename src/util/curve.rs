use crate::util::F32x2;
use std::borrow::Cow;
use float_cmp::{ ApproxEq, F32Margin };


#[derive(Clone, Copy)]
pub struct CurvePoint {
    pub pos   : F32x2,
    pub ltan  : f32,
    pub rtan  : f32
}


pub struct Curve<'l> {
    pub points : Cow<'l, [CurvePoint]>
}

impl Curve<'_> {

    pub fn sample(&self, t : f32) -> f32 {
        let t           = t.clamp(0.0, 1.0);
        let point_count = self.points.len();
        match (point_count) {
            0   => 0.0,
            1   => self.points[0].pos.y,
            2.. => {
                let i = self.index_at(t);
                if (i == (point_count - 1)) {
                    self.points[i].pos.y
                } else {
                    let local = t - self.points[i].pos.x;
                    if ((i == 0) && (local <= 0.0)) {
                        self.points[0].pos.y
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
            let a   = self.points[mid].pos.x;
            let b   = self.points[mid + 1].pos.x;
            if ((a < t) && (b < t)) {
                imin = mid;
            } else if (a > t) {
                imax = mid;
            } else {
                return mid;
            }
        }
        if (t > self.points[imax].pos.x) { imax } else { imin }
    }

    pub unsafe fn sample_local_unchecked(&self, i : usize, t_local : f32) -> f32 {
        let a  = self.points[i];
        let b  = self.points[i + 1];
        let dx = b.pos.x - a.pos.x;
        if (dx.approx_eq(0.0, F32Margin::default())) {
            b.pos.y
        } else {
            let t_local = t_local / dx;
            let dx      = dx / 3.0;
            let yac     = a.pos.y + (dx * a.rtan);
            let ybc     = b.pos.y - (dx * b.ltan);
            interpolate_bezier(a.pos.y, yac, ybc, b.pos.y, t_local)
        }
    }

}


fn interpolate_bezier(p0 : f32, p1 : f32, p2 : f32, p3 : f32, t1 : f32) -> f32 {
    let u1 = 1.0 - t1;
    let u2 = u1 * u1;
    let u3 = u2 * u1;
    let t2 = t1 * t1;
    let t3 = t2 * t1;
    (u3 * p0) + (3.0 * u2 * t1 * p1) + (3.0 * u1 * t2 * p2) + (t3 * p3)
}
