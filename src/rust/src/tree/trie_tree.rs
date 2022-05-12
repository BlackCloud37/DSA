use std::collections::HashMap;

use indextree::{Arena, NodeId};

use crate::tree::TrieTree;

/// Trie tree implementation with indextree
/// Node stores (is a word, HashMap from char to next node)
pub struct TrieIndexTree {
    arena: Arena<(bool, HashMap<u8, NodeId>)>,
    root_id: NodeId,
}

impl TrieIndexTree {
    /// get inner value in node, aux function to make code cleaner
    fn get_node(&self, id: NodeId) -> Option<&(bool, HashMap<u8, NodeId>)> {
        self.arena.get(id).map(|n| n.get())
    }

    /// get inner value in node, aux function to make code cleaner
    fn get_node_mut(&mut self, id: NodeId) -> Option<&mut (bool, HashMap<u8, NodeId>)> {
        self.arena.get_mut(id).map(|n| n.get_mut())
    }
}

impl TrieTree for TrieIndexTree {
    fn new() -> Self {
        let mut arena = Arena::new();
        let root_id = arena.new_node((false, HashMap::new()));
        Self { arena, root_id }
    }

    fn insert(&mut self, word: String) {
        let mut curr_id = self.root_id;
        for ch in word.into_bytes() {
            let next_id = {
                let (_, map) = self.get_node(curr_id).unwrap();
                map.get(&ch)
            };

            curr_id = match next_id {
                Some(next_id) => *next_id,
                None => {
                    let new_id = self.arena.new_node((false, HashMap::new()));
                    let (_, map) = self.get_node_mut(curr_id).unwrap();
                    map.insert(ch, new_id);
                    new_id
                }
            };
        }
        let (is_word, _) = self.get_node_mut(curr_id).unwrap();
        *is_word = true;
    }

    fn search(&self, word: String) -> bool {
        let mut curr_id = self.root_id;
        for ch in word.into_bytes() {
            curr_id = match self.arena.get(curr_id).unwrap().get().1.get(&ch) {
                Some(next_id) => *next_id,
                None => return false,
            };
        }
        let curr_node = self.arena.get(curr_id).unwrap();
        let (is_word, _) = curr_node.get();
        return *is_word;
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut curr_id = self.root_id;
        for ch in prefix.into_bytes() {
            curr_id = match self.arena.get(curr_id).unwrap().get().1.get(&ch) {
                Some(next_id) => *next_id,
                None => return false,
            };
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::{TrieIndexTree, TrieTree};

    #[test]
    fn trie_index_tree() {
        let mut trie = TrieIndexTree::new();
        trie.insert("apple".into());
        assert_eq!(trie.search("apple".into()), true); // 返回 True
        assert_eq!(trie.search("app".into()), false); // 返回 False
        assert_eq!(trie.starts_with("app".into()), true); // 返回 True
        trie.insert("app".into());
        assert_eq!(trie.search("app".into()), true); // 返回 True
    }
}
