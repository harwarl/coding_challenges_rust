// So as to declare a Tree of type char Tree::<char>

// explaination
/*
[a:5]   [b:2]   [c:1]    - Leaves
      (3)
     /   \
 [c:1]  [b:2] - Node


         (8)
        /   \
    [a:5]   (3)
            / \
        [c:1] [b:2]

Tree becomes
Node freq=8
 ├── left  -> Leaf { freq:5, char:'a' }
 └── right -> Node freq=3
              ├── left  -> Leaf { freq:1, char:'c' }
              └── right -> Leaf { freq:2, char:'b' }
*/

use Tree::*;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fmt::Debug,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Tree<T> {
    Leaf {
        char: T,
        freq: u64,
    },
    Node {
        freq: u64,
        left: Box<Tree<T>>,
        right: Box<Tree<T>>,
    },
}

#[allow(dead_code)]
impl<T: Clone> Tree<T> {
    pub fn freq(&self) -> u64 {
        match self {
            Self::Leaf { freq, .. } => *freq,
            Self::Node { freq, .. } => *freq,
        }
    }

    pub fn char(&self) -> Option<T> {
        match self {
            Self::Leaf { char, .. } => Some(char.clone()),
            Self::Node { .. } => None,
        }
    }

    pub fn left(&self) -> Option<&Tree<T>> {
        match self {
            Self::Node { left, .. } => Some(left),
            Self::Leaf { .. } => None,
        }
    }

    pub fn right(&self) -> Option<&Tree<T>> {
        match self {
            Self::Node { right, .. } => Some(right),
            Self::Leaf { .. } => None,
        }
    }
}

// Priority Queue depends on ORD
impl<T: Clone + Eq> Ord for Tree<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.freq().cmp(&other.freq())
    }
}

// Implement Partial Ord as well
impl<T: Clone + Eq> PartialOrd for Tree<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn huffman_tree<T: Clone + Eq + Debug>(char_map: HashMap<T, u64>) -> Tree<T> {
    // Since Binary Heap is Max Heap by default, use reverse for min Heap
    let mut min_heap = BinaryHeap::new();

    // Add char and frequencies to the heap table
    for (char, freq) in char_map {
        let (char, freq) = (char.clone(), freq);
        min_heap.push(Reverse(Leaf { char, freq }));
    }

    while min_heap.len() > 1 {
        // get the first two smallest elements
        let node1 = min_heap.pop().unwrap().0;
        let node2 = min_heap.pop().unwrap().0;

        println!("Node 1: {:?}, Node 2: {:?}", node1, node2);

        // merge the two nodes
        let merged_node = Node {
            freq: node1.freq() + node2.freq(),
            left: Box::new(node1),
            right: Box::new(node2),
        };

        // push the merged node into the max_heap
        min_heap.push(Reverse(merged_node));
    }
    // Return the Tree
    min_heap.pop().unwrap().0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_huffman() {
        let mut char_map = HashMap::new();
        char_map.insert('d', 10);
        char_map.insert('a', 40);
        char_map.insert('b', 25);
        char_map.insert('c', 15);

        // Build a huffman tree
        let tree = huffman_tree(char_map);
        assert_eq!(tree.freq(), 90); // confirming that the final node gives a freq of 90
        assert_eq!(tree.left().map(|n| n.freq()), Some(40));
        assert_eq!(tree.right().map(|n| n.freq()), Some(50));
        // test the most frequent char, which should be 'a'
        assert_eq!(tree.left().and_then(|char| char.char()), Some('a'));

        // second most frequent char
        assert_eq!(
            tree.right().and_then(|n| n.left()).and_then(|f| f.char()),
            Some('b')
        );
        assert_eq!(
            tree.right().and_then(|n| n.left()).map(|f| f.freq()),
            Some(25)
        );

        // Assert the 3rd most freqent char
        assert_eq!(
            tree.right()
                .and_then(|n| n.right())
                .and_then(|f| f.right())
                .and_then(|f| f.char()),
            Some('c')
        );
        assert_eq!(
            tree.right()
                .and_then(|n| n.right())
                .and_then(|f| f.right())
                .map(|f| f.freq()),
            Some(15)
        );

        // Assert the least occuring char
        assert_eq!(
            tree.right()
                .and_then(|n| n.right())
                .and_then(|f| f.left())
                .and_then(|f| f.char()),
            Some('d')
        );
        assert_eq!(
            tree.right()
                .and_then(|n| n.right())
                .and_then(|f| f.left())
                .map(|f| f.freq()),
            Some(10)
        );
    }
}
