## aho-corasick

The Aho—Korasik algorithm is a substring search algorithm that implements a search for a set of substrings from a dictionary in a given string.
The algorithmic complexity of the algorithm is O((n + m) * log k), where n is the total length of all words in the dictionary, k is the size of the alphabet, m is the length of the text.

### Usage example:
```rust
use aho_corasick::aho_corasick;
use std::collections::BTreeMap;

fn main() {
    let dict = ["aba", "baba", "cc"];
    let text = "ababababa";
    let res = aho_corasick(&dict, text);

    let mut map = BTreeMap::new();
    map.insert(0, vec![0, 2, 4, 6]);
    map.insert(1, vec![1, 3, 5]);
    assert_eq!(map, res);
}
```

### Cargo.toml
```bash
[dependencies]
aho-corasick = {git = "https://github.com/mingendo/aho-corasick.git", branch="main"}
```
