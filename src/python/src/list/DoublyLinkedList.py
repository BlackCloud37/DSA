import unittest


class DNode:
    def __init__(self, val=None, prev=None, next=None):
        self.prev = prev
        self.val = val
        self.next = next

# Structure: dummy head -> node -> node -> ...
class DoublyLinkedList:
    def __init__(self):
        self.head = DNode()
        self.tail = self.head
        self.len = 0

    def __str__(self) -> str:
        result = []
        curr = self.head.next
        while curr:
            result.append(str(curr.val))
            curr = curr.next
        return "->".join(result)

    # return the node inserted
    def push_back(self, val):
        return self.insert_after(self.tail, val)

    # return the node inserted
    def push_front(self, val):
        return self.insert_after(self.head, val)

    # return the value of removed node or None
    def pop_back(self):
        return self.remove_node(self.tail)

    # return the value of removed node or None
    def pop_front(self):
        return self.remove_node(self.head.next)

    # return the node inserted
    def insert_after(self, node, val):
        if node == None:
            return
        self.len += 1
        next = node.next
        new_node = DNode(val, node, next)
        node.next = new_node
        if next:
            next.prev = new_node
        else:
            self.tail = new_node
        return new_node

    # return the value of removed node or None
    def remove_node(self, node):
        if node == None or node == self.head or self.len == 0:
            return None
        self.len -= 1
        next = node.next
        prev = node.prev
        val = node.val
        prev.next = next
        if next:
            next.prev = prev
        else:
            self.tail = prev
        return val


class TestTrie(unittest.TestCase):
    def test_dll(self):
        dll = DoublyLinkedList()
        self.assertEqual(dll.len, 0)
        self.assertEqual(dll.__str__(), "")
        self.assertEqual(dll.pop_back(), None)
        dll.push_back(1)
        self.assertEqual(dll.__str__(), "1")
        self.assertEqual(dll.pop_back(), 1)
        dll.push_front(1)
        self.assertEqual(dll.__str__(), "1")
        self.assertEqual(dll.tail.val, 1)
        dll.push_back(3)
        self.assertEqual(dll.__str__(), "1->3")
        self.assertEqual(dll.tail.val, 3)
        dll.push_front(2)
        self.assertEqual(dll.len, 3)
        self.assertEqual(dll.__str__(), "2->1->3")
        self.assertEqual(dll.tail.val, 3)
        self.assertEqual(dll.pop_back(), 3)
        self.assertEqual(dll.pop_front(), 2)
        self.assertEqual(dll.__str__(), "1")
        self.assertEqual(dll.tail.val, 1)
        n1 = dll.push_back(2)
        n2 = dll.push_back(3)
        dll.remove_node(n1)
        self.assertEqual(dll.__str__(), "1->3")
        self.assertEqual(dll.tail.val, 3)
        dll.remove_node(n2)
        self.assertEqual(dll.__str__(), "1")
        self.assertEqual(dll.tail.val, 1)
