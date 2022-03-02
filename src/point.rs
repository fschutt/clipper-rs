#[cfg(use_int32)]
pub type CInt = i32;
#[cfg(not(use_int32))]
pub type CInt = i64;

pub trait IntPoint: PartialEq + Copy + Clone {
    fn new(x: CInt, y: CInt) -> Self;
    fn get_x(&self) -> CInt;
    fn get_y(&self) -> CInt;
    fn get_z(&self) -> Option<CInt>;
    fn set_x(&mut self, x: CInt);
    fn set_y(&mut self, y: CInt);
    fn set_z(&mut self, z: CInt);

    #[inline]
    fn get_dx(&self, other: &Self) -> f64 {
        if self.get_y() == other.get_y() {
            ::consts::HORIZONTAL
        } else {
            (other.get_x() - self.get_x()) as f64 / 
            (other.get_y() - self.get_y()) as f64
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(packed)]
struct IntPoint2d {
  pub x: CInt,
  pub y: CInt,
}

impl IntPoint for IntPoint2d {
    fn new(x: CInt, y: CInt) -> Self { Self { x: x, y: y } }
    fn get_x(&self) -> CInt { self.x }
    fn get_y(&self) -> CInt { self.y }
    fn get_z(&self) -> Option<CInt> { None }
    fn set_x(&mut self, x: CInt) { self.x = x; }
    fn set_y(&mut self, y: CInt) { self.y = y; }
    fn set_z(&mut self, _z: CInt) { }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct IntPoint3d {
  pub x: CInt,
  pub y: CInt,
  pub z: CInt,
}

impl IntPoint for IntPoint3d {
    fn new(x: CInt, y: CInt) -> Self { Self { x: x, y: y, z: 0 } }
    fn get_x(&self) -> CInt { self.x }
    fn get_y(&self) -> CInt { self.y }
    fn get_z(&self) -> Option<CInt> { Some(self.z) }
    fn set_x(&mut self, x: CInt) { self.x = x; }
    fn set_y(&mut self, y: CInt) { self.y = y; }
    fn set_z(&mut self, z: CInt) { self.z = z; }
}

pub trait DoublePoint {
    fn get_x(&self) -> f64;
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
