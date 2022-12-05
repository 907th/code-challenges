use std::collections::{HashMap, BTreeSet};
use std::cmp::{Ordering, Ord};

#[derive(Eq)]
struct E {
    freq: usize,
    word: String
}

impl E {
    fn new(freq: usize, word: String) -> Self {
        Self{ freq, word }
    }
}

impl Ord for E {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.freq != other.freq { return other.freq.cmp(&self.freq); }
        self.word.cmp(&other.word)
    }
}

impl PartialOrd for E {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for E {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq && self.word == other.word
    }
}

impl Clone for E {
    fn clone(&self) -> Self {
        Self::new(self.freq, self.word.clone())
    }
}

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut h: HashMap<String, usize> = HashMap::with_capacity(words.len());
        for s in words.into_iter() {
            h.entry(s).and_modify(|v| *v += 1).or_insert(1);
        }
        let mut s: BTreeSet<E> = BTreeSet::new();
        for (w, c) in h.iter() { // Complexity is O(n*log(k))
            s.insert(E::new(*c, w.clone()));
            if s.len() > k as usize {
                let e = s.iter().next_back().unwrap().clone();
                s.remove(&e);
            }
        }
        s.iter().map(|e| e.word.clone()).take(k as usize).collect()
    }
}
