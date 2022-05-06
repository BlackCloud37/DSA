use std::fmt::Debug;

use crate::tree::BinaryTree;

/// Binary Tree implemented with Option and Box
pub struct BinaryTreeBox<T> {
    root: Option<Box<BinaryTreeNodeBox<T>>>,
}

struct BinaryTreeNodeBox<T> {
    val: T,
    left: Option<Box<BinaryTreeNodeBox<T>>>,
    right: Option<Box<BinaryTreeNodeBox<T>>>,
}

impl<T> BinaryTreeNodeBox<T>
where
    T: Copy,
{
    pub fn from_vec(v: &[T]) -> Option<Box<BinaryTreeNodeBox<T>>> {
        if v.len() == 0 {
            return None;
        }

        let mid = v.len() / 2;
        return Some(Box::new(Self {
            val: v[mid],
            left: Self::from_vec(&v[..mid]),
            right: Self::from_vec(&v[mid + 1..]),
        }));
    }
}

impl<T> BinaryTree<T> for BinaryTreeBox<T>
where
    T: Copy + Clone + Default + Debug,
{
    fn from_vec(v: Vec<T>) -> Self {
        Self {
            root: BinaryTreeNodeBox::from_vec(&v),
        }
    }

    fn root(&self) -> Option<T> {
        return self.root.as_ref().map(|r| r.val);
    }

    fn height(&self) -> usize {
        if self.root.is_none() {
            return 0;
        }
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        queue.push_back((1, self.root.as_ref()));

        let mut height = 0;
        while let Some((d, n)) = queue.pop_front() {
            if let Some(node) = n {
                queue.push_back((d + 1, node.left.as_ref()));
                queue.push_back((d + 1, node.right.as_ref()));
                height = height.max(d);
            }
        }
        height
    }

    fn preorder_traverse(&self) -> Vec<T> {
        if self.root.is_none() {
            return vec![];
        }

        use std::collections::VecDeque;
        let mut stack = VecDeque::new();
        stack.push_back(self.root.as_ref().unwrap());

        let mut result = vec![];
        while let Some(top) = stack.pop_back() {
            result.push(top.val);

            if let Some(right) = top.right.as_ref() {
                stack.push_back(right)
            }
            if let Some(left) = top.left.as_ref() {
                stack.push_back(left)
            }
        }
        result
    }

    fn inorder_traverse(&self) -> Vec<T> {
        if self.root.is_none() {
            return vec![];
        }
        use std::collections::VecDeque;

        let mut curr = self.root.as_ref();
        let mut stack = VecDeque::new();
        let mut result = vec![];
        while !stack.is_empty() || curr.is_some() {
            if let Some(curr_node) = curr {
                stack.push_back(curr_node);
                curr = curr_node.left.as_ref();
            } else {
                let top = stack.pop_back().unwrap();
                result.push(top.val);
                curr = top.right.as_ref();
            }
        }
        return result;
    }

    fn postorder_traverse(&self) -> Vec<T> {
        if self.root.is_none() {
            return vec![];
        }

        use std::collections::VecDeque;
        let mut stack = VecDeque::new();
        stack.push_back(self.root.as_ref().unwrap());

        let mut result = vec![];
        while let Some(top) = stack.pop_back() {
            result.push(top.val);

            if let Some(left) = top.left.as_ref() {
                stack.push_back(left)
            }
            if let Some(right) = top.right.as_ref() {
                stack.push_back(right)
            }
        }
        result.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::{binary_tree::BinaryTreeBox, BinaryTree};

    #[test]
    fn binary_tree_box() {
        let tree = BinaryTreeBox::from_vec(vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(tree.root(), Some(4));
        assert_eq!(tree.height(), 3);
        assert_eq!(tree.preorder_traverse(), vec![4, 2, 1, 3, 6, 5]);
        assert_eq!(tree.inorder_traverse(), vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(tree.postorder_traverse(), vec![1, 3, 2, 5, 6, 4]);
    }
}
