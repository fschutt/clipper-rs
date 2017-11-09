/// Bitflags for Clipper init options
const EXECUTE_LOCKED: u8    = 0;
const HAS_OPEN_PATHS: u8    = 0;
const USE_FULL_RANGE: u8    = 0;
const REVERSE_OUTPUT: u8    = 0;
const STRICT_SIMPLE: u8     = 0;
const PRESERVE_COLINEAR: u8 = 0;

pub struct ThreadPool; // todo: make real threadpool

pub struct ClipperBuilder<'a> {
    options: ClipperInitOptions,
    thread_pool: Option<&'a ThreadPool>,
}

pub struct Clipper<'a, F: Fn(IntPoint3d, IntPoint3d) -> IntPoint3d + 'a> {
    options: u8,
    subj_fill_type: PolyFillType,
    clip_fill_type: PolyFillType,
    clip_type: PolyClipType,
    z_fill: Option<F>>,
    thread_pool: Option<&'a ThreadPool>,
}

#[repr(packed)]
pub struct ClipperInitOptions {
    pub execute_locked: bool,
    pub strict_simple: bool,
    pub preserve_colinear: bool,
}

impl<'a> ClipperBuilder<'a> {

    #[inline]
    pub fn new(options: ClipperInitOptions, thread_pool: Option<&'a ThreadPool>) -> Self {
        Self {
            options: options,
            thread_pool: thread_pool,
        }
    }

    #[inline]
    pub fn with_z_fill_function(&mut self, func: Option<fn(IntPoint3d, IntPoint3d) -> IntPoint3d>) {
        self.z_fill = func;
    }

    #[inline]
    pub fn build<'b: 'a>(self) -> Clipper<'b> {

        let mut opts = 0;
        if self.options.execute_locked() { opts |= EXECUTE_LOCKED };
        if self.options.strict_simple() { opts |= STRICT_SIMPLE };
        if self.options.preserve_colinear() { opts |= PRESERVE_COLINEAR };
        
        Clipper {
            options: opts,
            subj_fill_type: PolyFillType::EvenOdd,
            clip_fill_type: PolyFillType::EvenOdd,
            z_fill: self.z_fill,
            thread_pool: self.thread_pool,
        }
    }
}

// TODO!!!!
pub struct ClipperError;

impl<'a> Clipper<'a> {

    pub fn execute_polytree(clip_type: ClipType, solution: &mut Paths, fill_type: PolyFillType)
                            -> Result<(), ClipperError>
    {
        // do something with paths
    }

    pub fn execute_polytree(clip_type: ClipType, solution: &mut PolyTree, fill_type: PolyFillType)
                            -> Result<(), ClipperError>
    {
        // do something with polytree
    }

    pub fn closed_paths_from_polytree(poly_tree: &PolyTree) -> Paths {
        let relevant_nodes = poly_tree.iter().filter(|node| node.is_closed()).collect();
        let mut paths = Vec::<Path>::with_capacity(relevant_nodes.len());
        for node in relevant_nodes {
            paths.push
        }
        Paths { paths:  }
    }
    
    fn execute_internal() -> Result<(), ClipperError> {
        
    }
    
    fn is_contributing(edge: &Edge) -> bool {
        let (mut pft, mut pft2) = (self.subj_fill_type, self.clip_fill_type);
        if edge.poly_typ != self.pt_subject { ::std::mem::swap(&mut pft, &mut pft2); }

        match pft {
            PolyFillType::EvenOdd => if edge.wind_delta == 0 && edge.wind_cnt != 1 { return false; }
            PolyFillType::NonZero => if edge.wind_cnt.abs() != 1 { return false; }
            PolyFillType::Positive => if edge.wind_cnt != 1 { return false; }
            PolyFillType::Negative => if edge.wind_cnt != -1 { return false; }
        }

        match self.clip_type {
            
        }
    }                   
}
