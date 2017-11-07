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

#![allow(dead_code)]
#![allow(unused_macros)]

/// For i128 support on stable rust (may be slow)
#[cfg(use_int128)]
pub extern crate extprim;

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


pub struct OutPt<T: IntPoint> {
    pub idx: usize,
    pub pt: T,
    pub next: Arc<OutPt<T>>,
    pub prev: Arc<OutPt<T>>,
}

impl<T: IntPoint> OutPt<T> {
    // TODO!!
    pub fn area(&self) -> f64 {
        /*
            double Area(const OutPt *op)
            {
              const OutPt *startOp = op;
              if (!op) return 0;
              double a = 0;
              do {
                a +=  (double)(op->Prev->Pt.X + op->Pt.X) * (double)(op->Prev->Pt.Y - op->Pt.Y);
                op = op->Next;
              } while (op != startOp);
              return a * 0.5;
            }
        */
        0.0
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
