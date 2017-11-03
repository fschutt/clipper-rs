//! # clipper-rs
//!
//! A Rust port of the Delphi clipper library by Felix Schütt
//!
//! __Author    :__  Angus Johnson
//! __Version   :__  6.4.2
//! __Date      :__  27 February 2017
//! __Website   :__  http://www.angusj.com
//! __Copyright :__  Angus Johnson 2010-2017
//!
//! # License
//!
//! Use, modification & distribution is subject to Boost Software License Ver 1.
//! http://www.boost.org/LICENSE_1_0.txt
//!
//! # About this library
//!
//! The code in this library is an extension of Bala Vatti's clipping algorithm:
//! "A generic solution to polygon clipping"
//! Communications of the ACM, Vol 35, Issue 7 (July 1992) pp 56-63.
//! http://portal.acm.org/citation.cfm?id=129906
//!
//! Computer graphics and geometric modeling: implementation and algorithms
//! By Max K. Agoston
//! Springer; 1 edition (January 4, 2005)
//! http://books.google.com/books?q=vatti+clipping+agoston
//!
//! See also:
//! "Polygon Offsetting by Computing Winding Numbers"
//! Paper no. DETC2005-85513 pp. 565-575
//! ASME 2005 International Design Engineering Technical Conferences
//! and Computers and Information in Engineering Conference (IDETC/CIE2005)
//! September 24-28, 2005 , Long Beach, California, USA
//!
//! http://www.me.berkeley.edu/~mcmains/pubs/DAC05OffsetPolygon.pdf

use std::sync::{Arc, Weak, RwLock};
use std::f64::consts::PI;

#[derive(PartialEq, Eq)]
#[repr(u8)]
pub enum Direction {
    RightToLeft,
    LeftToRight,
}

#[derive(PartialEq, Eq)]
#[repr(u8)]
pub enum ClipType {
    Intersection,
    Union,
    Difference,
    Xor,
}

#[derive(PartialEq, Eq)]
#[repr(u8)]
pub enum PolyType {
    Subject,
    Clip,
}

/// By far the most widely used winding rules for polygon filling are
/// EvenOdd & NonZero (GDI, GDI+, XLib, OpenGL, Cairo, AGG, Quartz, SVG, Gr32)
/// Others rules include Positive, Negative and ABS_GTR_EQ_TWO (only in OpenGL)
/// see http://glprogramming.com/red/chapter11.html
#[derive(PartialEq, Eq)]
#[repr(u8)]
pub enum PolyFillType {
    EvenOdd,
    NonZero,
    Positive,
    Negative,
}

pub const LO_RANGE: isize = ::std::isize::MAX;
pub const HI_RANGE: isize = ::std::isize::MAX;

pub trait IntPoint {
    #[inline(always)]
    fn get_x(&self) -> isize;
    #[inline(always)]
    fn get_y(&self) -> isize;
}

#[derive(Clone, PartialEq, Eq)]
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

#[derive(Clone, PartialEq, Eq)]
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

#[derive(PartialEq, Eq, Copy, Clone)]
#[repr(u8)]
pub enum InitOptions {
    ReverseSolution,
    StrictlySimple,
    PreserveCollinear,
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum JoinType {
    Square,
    Round,
    Miter,
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum EndType {
    ClosedPolygon,
    ClosedLine,
    OpenButt,
    OpenSquare,
    OpenRound
}

#[repr(u8)]
enum EdgeSide {
    Left,
    Right
}

pub struct PolyNode {
    //node index in parent.childs
    pub index: usize,
    pub parent: Weak<RwLock<PolyNode>>,
    pub childs: Vec<Arc<RwLock<PolyNode>>>,
    pub is_open: bool,
    pub join_type: JoinType,
    pub end_type: EndType,
}

pub trait PolyNodeWrapper {
    fn get_next(&self) -> Option<Arc<RwLock<PolyNode>>>;
    fn add_child(&mut self, child: Arc<RwLock<PolyNode>>);
    fn get_next_sibling_up(&self) -> Option<Arc<RwLock<PolyNode>>>;
    fn is_hole(&self) -> bool;
    fn child_count(&self) -> usize;
}

impl PolyNodeWrapper for Arc<RwLock<PolyNode>> {

    fn get_next(&self) -> Option<Arc<RwLock<PolyNode>>> {
        // TODO!
        None
    }

    fn add_child(&mut self, child: Arc<RwLock<PolyNode>>) {
        let cnt = self.read().unwrap().childs.len();
        child.write().unwrap().parent = Arc::downgrade(&self);
        child.write().unwrap().index = cnt;
        self.write().unwrap().childs.push(child);
    }

    fn get_next_sibling_up(&self) -> Option<Arc<RwLock<PolyNode>>> {
        use PolyNodeWrapper;
        if self.read().unwrap().parent.upgrade().is_none() {
            None
        } else if self.read().unwrap().index == self.read().unwrap().parent.read().unwrap().childs.size() - 1 {
            Some(self.read().unwrap().parent.read().unwrap().get_next_sibling_up())
        } else {
            Some(self.read().unwrap().parent.read().unwrap().childs(self.index + 1).clone())
        }
    }

    fn is_hole(&self) -> bool {
        let result = true;
        let node_ptr = self.read().unwrap().parent;

        while let Some(node) = node_ptr.upgrade() {
            result = !result;
            node_ptr = node.read().unwrap().parent;
        }

        return result;
    }

    fn child_count(&self) -> usize {
        self.read().unwrap().childs.len()
    }
}

pub struct PolyTree {
    pub all_nodes: Vec<PolyNode>,
}

impl PolyTree {
    pub fn get_first(&self) -> Option<&PolyNode> {
        self.all_nodes.get(0)
    }

    pub fn total(&self) -> usize {
        self.all_nodes.len()
        // with negative offsets, ignore the hidden outer polygon ...
        // ?????? no idea what this should do?
        // if result > 0 && Childs[0] != AllNodes[0] { result -= 1; };
    }

    pub fn clear(&self) {
        self.all_nodes.clear();
    }
}

pub struct Path<T: IntPoint> {
    pub poly: Vec<T>,
}

impl<T: IntPoint> Path<T> {
    pub fn orientation(&self) -> bool {
        self.area() >= 0.0
    }

    pub fn area(&self) -> f64 {
        if self.poly.len() < 3 { return 0.0; };

        let mut a = 0_f64;
        let i = self.poly.iter();
        let j = self.poly.iter().rev();

        for (p_cur, p_next) in i.zip(j) {
            a += (p_next.get_x() + p_cur.get_x()) * (p_next.get_y() - p_cur.get_y());
        }

        -a * 0.5
    }
}

pub struct Paths<T: IntPoint> {
    pub paths: Vec<Path<T>>,
}

pub struct LocalMinimum<T: IntPoint> {
  y: isize,
  left_bound: Weak<Edge<T>>,
  right_bound: Weak<Edge<T>>,
}

pub struct IntRect {
    pub left: isize,
    pub top: isize,
    pub right: isize,
    pub bottom: isize,
}

pub const TWO_PI: f64 = PI * 2.0;
pub const DEF_ARC_TOLERANCE: f64 = 0.25;

pub struct Edge<T: IntPoint> {
    bottom: T,
    /// current (updated for every new scanbeam)
    current: T,
    Top: T,
    dx: f64,
    poly_typ: PolyType,
    /// side only refers to current side of solution poly
    side: EdgeSide,
    /// 1 or -1 depending on winding direction
    winding_delta: u8,
    winding_count: isize,
    //winding count of the opposite polytype
    winding_count_2: isize,
    out_idx: isize,
    next: Weak<Edge<T>>,
    prev: Weak<Edge<T>>,
    next_in_lml: Weak<Edge<T>>,
    next_in_ael: Weak<Edge<T>>,
    prev_in_ael: Weak<Edge<T>>,
    next_in_sel: Weak<Edge<T>>,
    prev_in_sel: Weak<Edge<T>>,
}

/// Edge not currently 'owning' a solution
pub const UNASSIGNED: i64 = -1;
/// Edge that would otherwise close a path
pub const SKIP: i64 = -2;

pub const HORIZONTAL: f64 = -1.0E+40;
pub const TOLERANCE: f64 = 1.0e-20;

macro_rules! near_zero {
    ($val:expr) => ((($val) > -TOLERANCE) && ((val) < TOLERANCE))
}



