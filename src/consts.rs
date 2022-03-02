use ::std::f64::consts::PI;

/// Edge not currently 'owning' a solution
pub const UNASSIGNED: i64 = -1;
/// Edge that would otherwise close a path
pub const SKIP: i64 = -2;

pub const HORIZONTAL: f64 = -1.0E+40;
pub const TOLERANCE: f64 = 1.0e-20;
pub const TWO_PI: f64 = PI * 2.0;
pub const DEF_ARC_TOLERANCE: f64 = 0.25;

pub const IS_HOLE: u8 = 1 << 0;
pub const IS_OPEN: u8 = 1 << 1;

/// Returns the LO_RANGE and HI_RANGE
#[cfg(use_int32)]
const LO_RANGE: isize = 0x7FFF;
#[cfg(use_int32)]
const HI_RANGE: isize = 0x7FFF;

#[cfg(not(use_int32))]
const LO_RANGE: u64 = 0x3FFFFFFF;
#[cfg(not(use_int32))]
const HI_RANGE: u64 = 0x3FFFFFFFFFFFFFFF;
