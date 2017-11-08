use point::{CInt, IntPoint};
use {EdgeSide, PolyType, EdgeIndex};

pub struct Edge<T: IntPoint> {
    pub bot: T,
    /// current (updated for every new scanbeam)
    pub current: T,
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
    pub fn is_horizontal(&self) {
        self.dx == ::consts::HORIZONTAL;
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
            self.dx.round() as CInt * (current_y - self.bot.get_y())
        }
    }
}

#[inline]
#[cfg(all(use_int32, use_int128))]
pub fn slopes_equal_edge2<T: IntPoint>(e1: &Edge<T>, e2: &Edge<T>) -> bool {
    use extprim::i128::i128;
    let sdy = i128::new(e1.top.get_y() - e1.bot.get_y());
    let sdx = i128::new(e1.top.get_x() - e1.bot.get_x());
    let edy = i128::new(e2.top.get_y() - e2.bot.get_y());
    let edx = i128::new(e2.top.get_x() - e2.bot.get_x());
    sdy * edx == sdx * edy
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
    use extprim::i128::i128;
    let p12y = i128::new(p1.get_y() - p2.get_y());
    let p12x = i128::new(p1.get_x() - p2.get_x());
    let p23y = i128::new(p2.get_y() - p3.get_y());
    let p23x = i128::new(p2.get_x() - p3.get_x());
    p12y * p23x == p12x * p23y
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
    use extprim::i128::i128;
    let p12y = i128::new(p1.get_y() - p2.get_y());
    let p12x = i128::new(p1.get_x() - p2.get_x());
    let p34y = i128::new(p3.get_y() - p4.get_y());
    let p34x = i128::new(p3.get_x() - p4.get_x());
    p12y * p34x == p12x * p34y
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