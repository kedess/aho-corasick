use std::collections::{BTreeMap, VecDeque};

#[derive(Clone)]
struct VertexAhoCorasick {
    children: BTreeMap<i32, i32>,
    link: i32,
    good_link: i32,
    pat_num: i32,
    pch: i32,
    parent: i32,
}

struct TrieAhoCorasick {
    arr: Vec<VertexAhoCorasick>,
    sz: i32,
}

impl TrieAhoCorasick {
    fn new() -> TrieAhoCorasick {
        TrieAhoCorasick {
            arr: vec![
                VertexAhoCorasick {
                    children: BTreeMap::new(),
                    link: -1,
                    good_link: -1,
                    pat_num: -1,
                    pch: -1,
                    parent: -1
                };
                1
            ],
            sz: 1,
        }
    }
    fn insert(&mut self, s: &str, num: i32) {
        let mut v = 0;
        for ch in s.as_bytes() {
            let idx = *ch as i32;
            if self.arr[v].children.get(&idx).is_none() {
                self.arr.push(VertexAhoCorasick {
                    children: BTreeMap::new(),
                    link: 0,
                    good_link: -1,
                    pat_num: -1,
                    pch: idx,
                    parent: v as i32,
                });
                self.arr[v].children.insert(idx, self.sz);
                self.sz += 1;
            }
            v = *self.arr[v].children.get(&idx).unwrap() as usize;
        }
        self.arr[v].pat_num = num;
    }
}

pub fn aho_corasick(dict: &[&str], t: &str) -> BTreeMap<i32, Vec<usize>> {
    let mut res: BTreeMap<i32, Vec<usize>> = BTreeMap::new();
    let mut trie = TrieAhoCorasick::new();
    for (idx, s) in dict.iter().enumerate() {
        trie.insert(*s, idx as i32);
    }
    let mut q = VecDeque::new();
    q.push_back(0);
    while !q.is_empty() {
        let curr = q.pop_front().unwrap();

        for value in trie.arr[curr as usize].children.values() {
            q.push_back(*value);
        }
        if curr == 0 {
            continue;
        }
        let parent = trie.arr[curr as usize].parent;
        let mut next_link = trie.arr[parent as usize].link;
        let pch = trie.arr[curr as usize].pch;
        while next_link >= 0 && trie.arr[next_link as usize].children.get(&pch).is_none() {
            next_link = trie.arr[next_link as usize].link;
        }
        if next_link >= 0 {
            let link = *trie.arr[next_link as usize].children.get(&pch).unwrap();
            let good_link = if trie.arr[link as usize].pat_num != -1 {
                link
            } else {
                trie.arr[link as usize].good_link
            };
            let r = &mut trie.arr[curr as usize];
            r.link = link;
            r.good_link = good_link;
        }
    }
    let mut v = 0i32;
    for (i, ch) in t.as_bytes().iter().enumerate() {
        let idx = *ch as i32;
        while v >= 0 && trie.arr[v as usize].children.get(&idx).is_none() {
            v = trie.arr[v as usize].link;
        }
        if v == -1 {
            v = 0;
        } else {
            v = *trie.arr[v as usize].children.get(&idx).unwrap();
        }
        if trie.arr[v as usize].pat_num != -1 {
            let value = res.entry(trie.arr[v as usize].pat_num).or_insert(vec![]);
            (*value).push(i + 1 - dict[trie.arr[v as usize].pat_num as usize].len());
        }
        let mut good_link = trie.arr[v as usize].good_link;
        while good_link > 0 {
            let value = res.entry(trie.arr[good_link as usize].pat_num).or_insert(vec![]);
            (*value).push(i + 1 - dict[trie.arr[good_link as usize].pat_num as usize].len());
            good_link = trie.arr[good_link as usize].good_link;
        }
    }
    res
}