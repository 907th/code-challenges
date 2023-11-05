// Esko Ukkonen's algorithm for a suffix tree construction
//
// Time: O(N) where N is a string length
// Memory: O(N)
// Reference: Ukkonen, E. On-line construction of suffix trees. Algorithmica 14, 249–260 (1995)

// FIXME: Do not use 'use' in Library
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct SuffixTree {
    chars: Vector<char>,
    root: SuffixTreeRef
}

pub type SuffixTreeRef = Rc<RefCell<SuffixTreeNode>>;

pub struct SuffixTreeNode {
    suffix_link: Option<SuffixTreeRef>,
    transitions: HashMap<char, (usize, usize, SuffixTreeRef)>
}

impl SuffixTree {
    pub fn new(source_string: &str) -> Self {
        let suffix_tree = SuffixTree{
            chars: source_string.chars().collect(),
            root: Rc::new(RefCell::new(SuffixTreeNode{
                suffix_link: None,
                transitions: HashMap::new()
            }))
        };
        suffix_tree.construct();
        suffix_tree
    }

    fn construct(&self) {
        let mut s = Rc::clone(self.root);
        let mut k = 0;
        for i in 0..self.chars.len() {
            (s, k) = self.update(s, k, i);
            (s, k) = self.canonize(s, k, i);
        }
    }

    fn update(s: SuffixTreeRef, k: usize, p: usize) -> (SuffixTreeRef, usize) {
        (s, k)
    }

    fn canonize(s: SuffixTreeRef, k: usize, p: usize) -> (SuffixTreeRef, usize) {
        if p < k { return (s, k); }

    }
}

