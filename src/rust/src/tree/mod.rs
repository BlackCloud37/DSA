use std::fmt::Debug;

mod binary_tree;

// Abstraction for binary tree
pub trait BinaryTree<T: Copy + Clone + Default + Debug> {
    /// from vector, middle elem of vec is current root, split and build left/right recursivly
    fn from_vec(v: Vec<T>) -> Self;
    /// value of root, None for empty tree
    fn root(&self) -> Option<T>;
    /// height of tree, 0 for empty tree, 1 for one-node-tree
    fn height(&self) -> usize;
    fn preorder_traverse(&self) -> Vec<T>;
    fn inorder_traverse(&self) -> Vec<T>;
    fn postorder_traverse(&self) -> Vec<T>;
}

#[cfg(test)]
mod tests {}
