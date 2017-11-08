pub trait IntPoint: PartialEq + Copy + Clone {
    #[inline(always)]
    fn get_x(&self) -> isize;
    #[inline(always)]
    fn get_y(&self) -> isize;
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(packed)]
struct IntPoint2d {
  pub x: isize,
  pub y: isize,
}

impl IntPoint for IntPoint2d {
    #[inline(always)]
    fn get_x(&self) -> isize { self.x }
    #[inline(always)]
    fn get_y(&self) -> isize { self.y }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct IntPoint3d {
  pub x: isize,
  pub y: isize,
  pub z: isize,
}

impl IntPoint for IntPoint3d {
    #[inline(always)]
    fn get_x(&self) -> isize { self.x }
    #[inline(always)]
    fn get_y(&self) -> isize { self.y }
}

pub trait DoublePoint {
    #[inline(always)]
    fn get_x(&self) -> f64;
    #[inline(always)]
    fn get_y(&self) -> f64;
}

#[repr(packed)]
pub struct DoublePoint2d {
  pub x: f64,
  pub y: f64,
}

impl From<IntPoint2d> for DoublePoint2d {
    fn from(p: IntPoint2d) -> Self {
        DoublePoint2d {
            x: p.x as f64,
            y: p.y as f64,
        }
    }
}

impl DoublePoint for DoublePoint2d {
    #[inline(always)]
    fn get_x(&self) -> f64 { self.x }
    #[inline(always)]
    fn get_y(&self) -> f64 { self.y }
}

pub struct DoublePoint3d {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl From<IntPoint3d> for DoublePoint3d {
    fn from(p: IntPoint3d) -> Self {
        DoublePoint3d {
            x: p.x as f64,
            y: p.y as f64,
            z: p.z as f64,
        }
    }
}

impl DoublePoint for DoublePoint3d {
    #[inline(always)]
    fn get_x(&self) -> f64 { self.x }
    #[inline(always)]
    fn get_y(&self) -> f64 { self.y }
}
