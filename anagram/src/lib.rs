use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut anagrams: HashSet<&str> = HashSet::new();

    for &pa in possible_anagrams {
        if pa.len() != word.len() {
            continue;
        }

        if pa.to_lowercase() == word.to_lowercase() {
            continue;
        }

        let mut w: Vec<_> = word.to_lowercase().chars().collect();

        for c_pa in pa.to_lowercase().chars().collect::<Vec<_>>() {
            if w.contains(&c_pa) {
                let c_idx = w.iter().position(|&x| x == c_pa).unwrap();
                w.remove(c_idx);
            } else {
                continue;
            }
            if w.is_empty() {
                anagrams.insert(pa);
            }
        }
    }

    anagrams
}
