use aho_corasick::aho_corasick;
use std::collections::BTreeMap;

#[test]
fn test_aho_corasick() {
    let mut dict = ["aba", "abb", "bbca"];
    let text = "abaabbbbca";
    let mut res = aho_corasick(&dict, text);

    let mut map = BTreeMap::new();
    map.insert(0, vec![0]);
    map.insert(1, vec![3]);
    map.insert(2, vec![6]);
    assert_eq!(map, res);

    let text = "abaabbbbcaaba";
    res = aho_corasick(&dict, text);

    map = BTreeMap::new();
    map.insert(0, vec![0, 10]);
    map.insert(1, vec![3]);
    map.insert(2, vec![6]);
    assert_eq!(map, res);

    let text = "abaabbbbcaba";
    res = aho_corasick(&dict, text);

    map = BTreeMap::new();
    map.insert(0, vec![0, 9]);
    map.insert(1, vec![3]);
    map.insert(2, vec![6]);
    assert_eq!(map, res);

    dict = ["abba", "bb", "cc"];
    let text = "abba";
    res = aho_corasick(&dict, text);

    map = BTreeMap::new();
    map.insert(0, vec![0]);
    map.insert(1, vec![1]);
    assert_eq!(map, res);

    dict = ["aba", "baba", "cc"];
    let text = "ababababa";
    res = aho_corasick(&dict, text);

    map = BTreeMap::new();
    map.insert(0, vec![0, 2, 4, 6]);
    map.insert(1, vec![1, 3, 5]);
    assert_eq!(map, res);
}

#[test]
fn test_doc() {
    let dict = ["aba", "baba", "cc"];
    let text = "ababababa";
    let res = aho_corasick(&dict, text);

    let mut map = BTreeMap::new();
    map.insert(0, vec![0, 2, 4, 6]);
    map.insert(1, vec![1, 3, 5]);
    assert_eq!(map, res);
}
