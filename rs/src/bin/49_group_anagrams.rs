use std::collections::HashMap;

fn main() {
    let mut v = vec!["eat", "tea", "tan", "ate", "nat", "bat"];
    let res = v.iter_mut().map(|x| x.to_string()).collect();

    println!("result: {:?}", group_anagrams(res));
}

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::new();

    for s in strs {
        let mut key = s.as_bytes().to_vec();
        key.sort();

        map.entry(key).or_insert_with(Vec::new).push(s);
    }

    map.values().cloned().collect()
}
