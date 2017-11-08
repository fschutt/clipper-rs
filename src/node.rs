use point::IntPoint;
use {PolyTree, Path, PolyNodeIndex, JoinType, EndType};

// TODO: This system should not rely on IDs, rather on Arc<> references
// Clean up!!

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
    pub(crate) fn add_child(&mut self, mut child: Self) {
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

    pub(crate) fn child_count(&self) -> usize {
        self.childs.len()
    }
}
