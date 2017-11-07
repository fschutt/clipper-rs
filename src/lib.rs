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
    
#[macro_use]
pub mod macros;
pub mod consts;
pub mod edge;
pub mod node;
pub mod point;

use std::f64::consts::PI;
use std::marker::PhantomData;
use point::IntPoint;

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

/// In Rust you can't have pointers like in the C++ version
/// So we indices instead
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PolyNodeIndex { 
    pub(crate) node_idx: usize 
}

/// In Rust you can't have pointers like in the C++ version
/// So we indices instead
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct EdgeIndex { 
    pub(crate) edge_idx: usize 
}

pub struct PolyNode<T: IntPoint> {
    /// Reference to the tree the node is located in
    pub tree: ::std::sync::Arc<::std::sync::Mutex<PolyTree<T>>>,
    /// The index in the global node memory pool
    pub glob_index: PolyNodeIndex,
    /// The index in the current child vector
    pub index: usize,
    pub contour: Path<T>,
    pub parent: Option<PolyNodeIndex>,
    pub childs: Vec<PolyNodeIndex>,
    pub is_open: bool,
    pub join_type: JoinType,
    pub end_type: EndType,
}

impl<T: IntPoint> PolyNode<T> {

    pub(crate) fn get_next(&self) -> Option<PolyNodeIndex> {
        // TODO
        None
    }

    /// Adds a child node by registering on the tree, adding the indices and saving the index of the node
    /// TODO: This is maybe a bit less efficient than the C++ version ...
    pub(crate) fn add_child(&mut self, mut child: PolyNode<T>) {
        let cnt = self.childs.len() - 1;
        let mut tree_lock = self.tree.lock().unwrap();
        child.parent = Some(self.glob_index);
        child.index = cnt;
        let glob_cnt = tree_lock.all_nodes.len();
        child.glob_index = PolyNodeIndex { node_idx: glob_cnt };
        tree_lock.all_nodes.push(child);
    }

    pub(crate) fn get_next_sibling_up(&self) -> Option<PolyNodeIndex> {
        match self.parent {
            None => None,
            Some(parent) => {
                let tree_lock = self.tree.lock().unwrap();
                let parent_node = &tree_lock.all_nodes[parent.node_idx];
                if self.index == parent_node.childs.len() - 1 {
                    parent_node.get_next_sibling_up()
                } else {
                    Some(parent_node.childs[self.index + 1])
                }
            }
        }
    }

    pub(crate) fn is_hole(&self) -> bool {
        let mut result = true;
        let mut node_idx = self.parent;

        loop {
            match node_idx {
                Some(idx) => { 
                    result = !result;
                    // node_idx MUST always be valid, so this unwrap is safe
                    node_idx = self.tree.lock().unwrap().all_nodes[idx.node_idx].parent;
                }, 
                None => { break; }
            }
        }

        return result;
    }

    fn child_count(&self) -> usize {
        self.childs.len()
    }
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
        self.all_nodes.len()
        // with negative offsets, ignore the hidden outer polygon ...
        // ?????? no idea what this should do?
        // if result > 0 && Childs[0] != AllNodes[0] { result -= 1; };
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

    // TODO: NOT TESTED IF THIS FUNCTION ACTUALL DOES THE RIGHT THING
    pub fn area(&self) -> f64 {
        if self.poly.len() < 3 { return 0.0; };

        let mut a = 0;
        let i = self.poly.iter();
        let j = self.poly.iter().rev();

        for (p_cur, p_next) in i.zip(j) {
            a += (p_next.get_x() + p_cur.get_x()) * (p_next.get_y() - p_cur.get_y());
        }

        -a as f64 * 0.5
    }
}

pub struct Paths<T: IntPoint> {
    pub paths: Vec<Path<T>>,
}

pub struct LocalMinimum<T: IntPoint> {
  y: isize,
  left_bound: EdgeIndex,
  right_bound: EdgeIndex,
  _type: PhantomData<T>,
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
    top: T,
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
    next: EdgeIndex,
    prev: EdgeIndex,
    next_in_lml: EdgeIndex,
    next_in_ael: EdgeIndex,
    prev_in_ael: EdgeIndex,
    next_in_sel: EdgeIndex,
    prev_in_sel: EdgeIndex,
}




