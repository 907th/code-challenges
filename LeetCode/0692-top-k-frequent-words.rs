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

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut h: HashMap<String, usize> = HashMap::with_capacity(words.len());
        for s in words.into_iter() {
            h.entry(s).and_modify(|v| *v += 1).or_insert(1);
        }
        let mut s: BTreeSet<E> = BTreeSet::new();
        for (w, c) in h.iter() {
            s.insert(E::new(*c, w.clone()));
            // If I was able to remove the last element of the set when s.len() > k
            // then overall complexity would be O(n*log(k))
        }
        s.iter().map(|e| e.word.clone()).take(k as usize).collect()
    }
}
