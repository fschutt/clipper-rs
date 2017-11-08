use point::{CInt, IntPoint};
use {EdgeSide, PolyType, EdgeIndex};

pub struct Edge<T: IntPoint> {
    pub bot: T,
    /// current (updated for every new scanbeam)
    pub cur: T,
    pub top: T,
    pub dx: f64,
    pub poly_typ: PolyType,
    /// side only refers to current side of solution poly
    pub side: EdgeSide,
    /// 1 or -1 depending on winding direction
    pub winding_delta: u8,
    pub winding_count: isize,
    //winding count of the opposite polytype
    pub winding_count_2: isize,
    pub out_idx: isize,
    pub next: EdgeIndex,
    pub prev: EdgeIndex,
    pub next_in_lml: EdgeIndex,
    pub next_in_ael: EdgeIndex,
    pub prev_in_ael: EdgeIndex,
    pub next_in_sel: EdgeIndex,
    pub prev_in_sel: EdgeIndex,
}

impl<T: IntPoint> Edge<T> {
    #[inline]
    pub fn is_horizontal(&self) -> bool {
        self.dx == ::consts::HORIZONTAL
    }

    #[inline]
    pub fn set_dx(&mut self) {
        let dy  = self.top.get_y() - self.bot.get_y();
        self.dx = if dy == 0 {
            ::consts::HORIZONTAL
        } else {
            (self.top.get_x() - self.bot.get_x()) as f64 / dy as f64
        };
    }

    #[inline]
    pub fn swap_sides(&mut self, other: &mut Self) {
        ::std::mem::swap(&mut self.side, &mut other.side);
    }

    #[inline]
    pub fn swap_poly_indices(&mut self, other: &mut Self) {
        ::std::mem::swap(&mut self.out_idx, &mut other.out_idx);
    }

    #[inline]
    pub fn top_x(&self, current_y: CInt) -> CInt {
        if current_y == self.top.get_y() {
            self.top.get_x()
        } else {
            self.bot.get_x() + self.dx.round() as CInt * (current_y - self.bot.get_y())
        }
    }

    /// Calculates the intersection point between two edges
    pub fn intersect_point(&mut self, other: &mut Self) -> T {

        let mut ip: T;

        // warn: matching floating point value
        if self.dx == other.dx {
            let cy = self.cur.get_y();
            return T::new(self.top_x(cy), cy);
        }

        // warn: matching floating point value
        else if self.dx == 0.0 {
            let cur_y = if other.is_horizontal() {
                other.bot.get_y()
            } else {
                let b2 = other.bot.get_y() as f64 - (other.bot.get_x() as f64 / other.dx);
                (self.bot.get_x() as f64 / other.dx + b2).round() as CInt
            };

            ip = T::new(self.bot.get_x(), cur_y);
        }

        // warn: matching floating point value
        // reverse of previous block
        else if other.dx == 0.0 {
            let cur_y = if self.is_horizontal() {
                self.bot.get_y()
            } else {
                let b2 = self.bot.get_y() as f64 - (self.bot.get_x() as f64 / self.dx);
                (other.bot.get_x() as f64 / self.dx + b2).round() as CInt
            };

            ip = T::new(other.bot.get_x(), cur_y);
        }

        else {
            let b1 = (self.bot.get_x() - self.bot.get_y()) as f64 * self.dx;
            let b2 = (other.bot.get_x() - other.bot.get_y()) as f64 * other.dx;
            let q = (b2 - b1) as f64 / (self.dx - other.dx);
            let cur_y = q.round() as CInt;
            let cur_x = if self.dx.abs() < other.dx.abs() {
                (self.dx * q + b1).round() as CInt
            } else {
                (other.dx * q + b2).round() as CInt
            };

            ip = T::new(cur_x, cur_y);
        }

        if ip.get_y() > self.cur.get_y() {
            let prev_y = self.cur.get_y();
            ip.set_y(prev_y);
            if self.dx.abs() > other.dx.abs() {
                ip.set_x(other.top_x(prev_y));
            } else {
                ip.set_x(self.top_x(prev_y));
            }
        }

        ip
    }
}

#[inline]
#[cfg(all(use_int32, use_int128))]
pub fn slopes_equal_edge2<T: IntPoint>(e1: &Edge<T>, e2: &Edge<T>) -> bool {
    let sdy = e1.top.get_y() - e1.bot.get_y();
    let sdx = e1.top.get_x() - e1.bot.get_x();
    let edy = e2.top.get_y() - e2.bot.get_y();
    let edx = e2.top.get_x() - e2.bot.get_x();

    let a: (i64, u64) = int128mul!(sdy, edx);
    let b: (i64, u64) = int128mul!(sdx, edy);

    a.0 == b.0 && a.1 == b.1
}

#[inline]
#[cfg(not(all(use_int32, use_int128)))]
pub fn slopes_equal_edge2<T: IntPoint>(e1: &Edge<T>, e2: &Edge<T>) -> bool {
    let sdy = e1.top.get_y() - e1.bot.get_y();
    let sdx = e1.top.get_x() - e1.bot.get_x();
    let edy = e2.top.get_y() - e2.bot.get_y();
    let edx = e2.top.get_x() - e2.bot.get_x();
    sdy * edx == sdx * edy
}

#[inline]
#[cfg(all(use_int32, use_int128))]
pub fn slopes_equal_point3<T: IntPoint>(p1: &T, p2: &T, p3: &T) -> bool {
    let p12y = p1.get_y() - p2.get_y();
    let p12x = p1.get_x() - p2.get_x();
    let p23y = p2.get_y() - p3.get_y();
    let p23x = p2.get_x() - p3.get_x();

    let a: (i64, u64) = int128mul!(p12y, p23x);
    let b: (i64, u64) = int128mul!(p12x, p23y);

    a.0 == b.0 && a.1 == b.1
}

#[inline]
#[cfg(not(all(use_int32, use_int128)))]
pub fn slopes_equal_point3<T: IntPoint>(p1: &T, p2: &T, p3: &T) -> bool {
    let p12y = p1.get_y() - p2.get_y();
    let p12x = p1.get_x() - p2.get_x();
    let p23y = p2.get_y() - p3.get_y();
    let p23x = p2.get_x() - p3.get_x();
    p12y * p23x == p12x * p23y
}

#[inline]
#[cfg(all(use_int32, use_int128))]
pub fn slopes_equal_point4<T: IntPoint>(p1: &T, p2: &T, p3: &T, p4: &T) -> bool {
    let p12y = p1.get_y() - p2.get_y();
    let p12x = p1.get_x() - p2.get_x();
    let p34y = p3.get_y() - p4.get_y();
    let p34x = p3.get_x() - p4.get_x();
    let a: (i64, u64) = int128mul!(p12y, p34x);
    let b: (i64, u64) = int128mul!(p12x, p34y);
    a.0 == b.0 && a.1 == b.1
}

#[inline]
#[cfg(not(all(use_int32, use_int128)))]
pub fn slopes_equal_point4<T: IntPoint>(p1: &T, p2: &T, p3: &T, p4: &T) -> bool {
    let p12y = p1.get_y() - p2.get_y();
    let p12x = p1.get_x() - p2.get_x();
    let p34y = p3.get_y() - p4.get_y();
    let p34x = p3.get_x() - p4.get_x();
    p12y * p34x == p12x * p34y
}
