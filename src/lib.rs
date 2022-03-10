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

// stdsimd needs this for detecting CPU features at runtime
//#![feature(cfg_target_feature)]

#![allow(dead_code)]
#![allow(unused_macros)]

#[macro_use]
pub mod macros;
pub mod consts;
pub mod edge;
pub mod node;
pub mod point;

use std::marker::PhantomData;
use std::sync::Arc;

use point::IntPoint;
use node::PolyNode;
use consts::*;
use edge::Edge;

#[derive(PartialEq, Eq)]
pub enum Direction {
    RightToLeft,
    LeftToRight,
}

#[derive(PartialEq, Eq)]
pub enum ClipType {
    Intersection,
    Union,
    Difference,
    Xor,
}

#[derive(PartialEq, Eq)]
pub enum PolyType {
    Subject,
    Clip,
}

/// By far the most widely used winding rules for polygon filling are
/// EvenOdd & NonZero (GDI, GDI+, XLib, OpenGL, Cairo, AGG, Quartz, SVG, Gr32)
/// Others rules include Positive, Negative and ABS_GTR_EQ_TWO (only in OpenGL)
/// see http://glprogramming.com/red/chapter11.html
#[derive(PartialEq, Eq)]
pub enum PolyFillType {
    EvenOdd,
    NonZero,
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum InitOptions {
    ReverseSolution,
    StrictlySimple,
    PreserveCollinear,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum JoinType {
    Square,
    Round,
    Miter,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum EndType {
    ClosedPolygon,
    ClosedLine,
    OpenButt,
    OpenSquare,
    OpenRound
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum EdgeSide {
    Left,
    Right
}

// In Rust you can't have pointers like in the C++ version
// So we use indices instead

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PolyNodeIndex {
    pub(crate) node_idx: usize
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct EdgeIndex {
    pub(crate) edge_idx: usize
}

pub struct PolyTree<T: IntPoint> {
    /// Pool of nodes
    pub all_nodes: Vec<PolyNode<T>>,
    pub all_edges: Vec<Edge<T>>,
}

impl<T: IntPoint> PolyTree<T> {

    /// Creates a new, empty PolyTree
    pub fn new() -> Self {
        Self {
            all_nodes: Vec::new(),
            all_edges: Vec::new(),
        }
    }

    pub fn get_first(&self) -> Option<&PolyNode<T>> {
        self.all_nodes.get(0)
    }

    pub fn total(&self) -> usize {
        let mut result = self.all_nodes.len();
        if result != 0 { result -= 1; }
        result
    }

    pub fn clear(&mut self) {
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

        let size = self.poly.len();
        if size < 3 { return 0.0; };

        let mut a = 0;
        let mut j = size - 1;

        for i in 0..size {
            a += unsafe { (self.poly.get_unchecked(j).get_x() + self.poly.get_unchecked(i).get_x())
                        * (self.poly.get_unchecked(j).get_y() - self.poly.get_unchecked(i).get_y()) };
            j = i;
        }

        -a as f64 * 0.5
    }
}

pub struct Paths<T: IntPoint> {
    pub paths: Vec<Path<T>>,
}

pub struct IntRect {
    pub left: isize,
    pub top: isize,
    pub right: isize,
    pub bottom: isize,
}

pub struct IntersectNode<T: IntPoint> {
    pub edge_1: EdgeIndex,
    pub edge_2: EdgeIndex,
    pub pt: T,
}

pub struct LocalMinimum<T: IntPoint> {
    #[cfg(use_int32)]
    y: isize,
    #[cfg(not(use_int32))]
    y: u64,
    left_bound: EdgeIndex,
    right_bound: EdgeIndex,
    _type: PhantomData<T>,
}

#[derive(PartialEq)]
pub struct OutPt<T: IntPoint> {
    pub idx: usize,
    pub pt: T,
    pub next: Arc<OutPt<T>>,
    pub prev: Arc<OutPt<T>>,
}

impl<T: IntPoint> OutPt<T> {
    // TODO!!
    pub fn area(&self) -> f64 {
        let start = self.next.clone();
        let mut area = 0.0;
        let mut op = start.clone();
        loop {
            area += ((op.prev.pt.get_x() + op.pt.get_x()) *
                     (op.prev.pt.get_y() - op.pt.get_y())) as f64;
            op = op.next.clone();
            if *op == *start { break; }
        }

        area * 0.5
    }

    pub fn reverse_poly_pt_list(&mut self) {
/*

        // not possible in the rust model, also very bad for cache
        let start = self.next.clone();
        let mut op = start.clone();
        loop {
            let pp2 = op.next.clone();
            op.next = op.prev.clone();
            op.prev = pp2.clone();
            op = pp2;
            if *op == *start { break; }
        }

            if (!pp) return;
            OutPt *pp1, *pp2;
            pp1 = pp;
            do {
            pp2 = pp1->Next;
            pp1->Next = pp1->Prev;
            pp1->Prev = pp2;
            pp1 = pp2;
            } while( pp1 != pp );
*/
    }
}

pub struct OutRec<T: IntPoint> {
    pub idx: usize,
    // TODO: is_hole and is_open can be merged!!!
    pub is_hole_open: u8,
    //see comments in clipper.pas
    pub first_left: Arc<OutRec<T>>,
    pub poly_node: Arc<PolyNode<T>>,
    pub pts: Arc<OutPt<T>>,
    pub bottom_pt: Arc<OutPt<T>>,
}

impl<T: IntPoint> OutRec<T> {
    pub fn area(&self) -> f64 {
        self.pts.area()
    }
}

impl<T: IntPoint> OutRec<T> {
    #[inline(always)]
    pub fn is_hole(&self) -> bool {
        self.is_hole_open & IS_HOLE == 0
    }
    #[inline(always)]
    pub fn is_open(&self) -> bool {
        self.is_hole_open & IS_OPEN == 0
    }
}

pub struct Join<T: IntPoint> {
    pub out_pt1: Arc<OutPt<T>>,
    pub out_pt2: Arc<OutPt<T>>,
    pub off_pt: T,
}

pub fn point_is_vertex<T: IntPoint>(pt: &T, pp: Arc<OutPt<T>>) -> bool {
    let mut pp2 = pp.clone();
    loop {
        if pp2.pt == *pt { return true; }
        pp2 = pp2.next.clone();
        if *pp2 == *pp { break; }
    }
    false
}

/// See http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.88.5498&rep=rep1&type=pdf
/// returns 0 if false, +1 if true, -1 if pt ON polygon boundary
pub fn is_point_in_path<T: IntPoint>(pt: &T, path: &Path<T>) -> i8 {

    if path.poly.len() < 3 { return 0; }

    let mut result: i8 = 0;
    let ip_iter = path.poly.iter();
    let mut np_iter = path.poly.iter().cycle();
    np_iter.next();

    let pt_y = pt.get_y();
    let pt_x = pt.get_x();

    for (ip, np) in ip_iter.zip(np_iter) {

        let ip_x = ip.get_x();
        let ip_y = ip.get_y();
        let np_x = np.get_x();
        let np_y = np.get_y();

        if np_y == pt_y &&
           (np_x == pt_x || ip_y == np_y &&
           ((np_x > pt_x) == (ip_x < pt_x))) {
           return -1;
        }

        if (ip_y < pt_y) == (np_y < pt_y) { continue; }

        let cond1 = ip_x >= pt_x;

        if cond1 && (np_x > pt_x) {
            result = 1 - result;
            continue;
        }

        if cond1 || (np_x > pt_x) {

            let mut vec_a = ip_x - pt_x;
            let mut vec_b = np_y - pt_y;
            let mut vec_c = np_x - pt_x;
            let mut vec_d = ip_y - pt_y;

            let cond2 = np_y > ip_y;

            if (vec_a >> 31) > 0 || (vec_b >> 31) > 0 || (vec_c >> 31) > 0 || (vec_d >> 31) > 0 {
                // possible overflow
                let mut a: (i64, u64) = int128mul!(vec_a, vec_b);
                let b: (i64, u64) = int128mul!(vec_c, vec_d);

                a.0 -= b.0;
                a.1 -= b.1;
                if a.1 < b.1 { a.0 += 1 };
                if a.0 == 0 && a.1 == 0 {
                    return -1;
                } else if a.0 >= 0 {
                    result = 1 - result;
                }
            } else {
                // will not overflow
                let d = vec_a * vec_b - vec_c * vec_d;
                if d == 0 {
                    return -1;
                } else if (d > 0) == cond2 {
                    result = 1 - result;
                }
            }
        }
    }

    return result;
}

/// Checks if a point falls in an OutPt
/// renamed from `int PointInPolygon (const IntPoint &pt, OutPt *op)`
pub fn is_point_in_out_pt<T: IntPoint>(pt: &T, op: Arc<OutPt<T>>) -> i8 {

    // This is different from the original algorithm:
    // Instead of following pointers, we collect the OutPt into a path
    // This provides better cache access + lets us reuse the point
    let mut out_path = Vec::<T>::new();
    let origin_op = op.clone();
    let mut cur_op = op.clone();

    while cur_op != origin_op {
        out_path.push(cur_op.pt);
        cur_op = cur_op.next.clone();
    }

    is_point_in_path(pt, &Path { poly: out_path })
}

/// TODO: this works, but it is worst-case O(n^2)
/// as we check every point against every other point
///
/// In theory, this should perform better than the C++ version ("Poly2ContainsPoly1")
/// due to better cache access.
pub fn poly2_contains_poly1<T: IntPoint>(pt1: Arc<OutPt<T>>, pt2: Arc<OutPt<T>>) -> bool {

    // create path for pt2
    let mut out_path = Vec::<T>::new();
    let origin_op = pt2.clone();
    let mut cur_op = pt2.clone();

    while cur_op != origin_op {
        out_path.push(cur_op.pt);
        cur_op = cur_op.next.clone();
    }

    let pt2_path = Path { poly: out_path };

    let origin_op = pt1.clone();
    let mut cur_op = pt1.clone();

    while cur_op != origin_op {
        let res = is_point_in_path(&cur_op.pt, &pt2_path);
        if res >= 0 { return res > 0 }
        cur_op = cur_op.next.clone();
    }

    true
}


