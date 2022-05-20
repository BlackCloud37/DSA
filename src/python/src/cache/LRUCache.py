import unittest
from list.DoublyLinkedList import DoublyLinkedList

class LRUCache:
    def __init__(self, size) -> None:
        self.lru_map_key_node = {}
        self.lru_map_key_value = {}
        self.lru_list = DoublyLinkedList()
        self.size = size

    # put a kv pair
    # if key exists, replace the value
    # if the cache is full, kick the LRU one
    def put(self, key, val):
        node = self.lru_map_key_node.get(key, None)
        if node:
            self.lru_map_key_value[key] = val
            self.lru_list.remove_node(node)
            self.lru_map_key_node[key] = self.lru_list.push_front(key)
        else:
            if len(self.lru_map_key_node) == self.size:
                victim_key = self.lru_list.pop_back()
                del self.lru_map_key_node[victim_key]
                del self.lru_map_key_value[victim_key]
            self.lru_map_key_value[key] = val
            self.lru_map_key_node[key] = self.lru_list.push_front(key)
    
    # None if key doesn't exist
    def get(self, key):
        node = self.lru_map_key_node.get(key, None)
        if node:
            self.lru_list.remove_node(node)
            self.lru_map_key_node[key] = self.lru_list.push_front(key)
        return self.lru_map_key_value.get(key, None)

class TestTrie(unittest.TestCase):
    def test_lru(self):
        cache = LRUCache(2)
        cache.put(1, 1)
        cache.put(2, 2)
        self.assertEqual(cache.get(1), 1)
        self.assertEqual(cache.get(3), None)
        cache.put(3, 3) # 2 kicked
        self.assertEqual(cache.get(2), None)
        cache.put(4, 4) # 1 kicked
        self.assertEqual(cache.get(1), None)
        self.assertEqual(cache.get(3), 3)
        self.assertEqual(cache.get(4), 4)