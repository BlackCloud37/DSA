use std::fmt::Debug;

mod binary_tree;
mod trie_tree;

/// Abstraction for binary tree
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

/// Abstracion for trie tree, here all word must be in ascii
/// Refer to https://leetcode.cn/problems/implement-trie-prefix-tree/
pub trait TrieTree {
    fn new() -> Self;
    /// Insert `word` into tree, do nothing if `word` exists
    fn insert(&mut self, word: String);
    /// Return `true` if `word` in `tree` else `false`
    fn search(&self, word: String) -> bool;
    /// Return `true` if any word starts with `prefix` in tree else `false`
    fn starts_with(&self, prefix: String) -> bool;
}

#[cfg(test)]
mod tests {}
