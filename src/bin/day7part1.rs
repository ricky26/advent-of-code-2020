use std::io::Read;
use std::collections::{BTreeMap, BTreeSet, VecDeque, HashSet};
use aoc2020::bags;

fn invert_mappings(mappings: &BTreeMap<String, BTreeMap<String, usize>>) -> BTreeMap<String, BTreeSet<String>> {
    let mut inverted_mappings = BTreeMap::new();

    for (key, mapping) in mappings.iter() {
        for value in mapping.keys() {
            let set = inverted_mappings
                .entry(value.clone())
                .or_insert_with(|| BTreeSet::new());
            set.insert(key.clone());
        }
    }

    inverted_mappings
}

struct ParentIterator<'a> {
    mappings: &'a BTreeMap<String, BTreeSet<String>>,
    next: VecDeque<String>,
    visited: HashSet<String>,
}

impl<'a> ParentIterator<'a> {
    pub fn new(mappings: &'a BTreeMap<String, BTreeSet<String>>, next: String) -> ParentIterator<'a> {
        let mut iter = ParentIterator{
            mappings,
            next: vec![next].into(),
            visited: HashSet::new(),
        };
        iter.next();
        iter
    }
}

impl<'a> Iterator for ParentIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let v = self.next.pop_front();
        if let Some(ref next) = v {
            if let Some(set) = self.mappings.get(next) {
                for key in set.iter() {
                    if self.visited.insert(key.to_string()) {
                        self.next.push_back(key.to_string());
                    }
                }
            }
        }
        v
    }
}


struct ChildIterator<'a> {
    mappings: &'a BTreeMap<String, BTreeMap<String, usize>>,
    next: VecDeque<(String, usize)>,
}

impl<'a> ChildIterator<'a> {
    pub fn new(mappings: &'a BTreeMap<String, BTreeMap<String, usize>>, next: String) -> ChildIterator<'a> {
        let mut iter = ChildIterator{
            mappings,
            next: vec![(next, 1)].into(),
        };
        iter.next();
        iter
    }
}

impl<'a> Iterator for ChildIterator<'a> {
    type Item = (String, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let v = self.next.pop_front();
        if let Some(ref next) = v {
            if let Some(set) = self.mappings.get(&next.0) {
                for (key, value) in set.iter() {
                    self.next.push_back((key.to_string(), *value * next.1));
                }
            }
        }
        v
    }
}

fn main() -> anyhow::Result<()> {
    let mut contents = String::new();
    std::io::stdin().read_to_string(&mut contents)?;
    let mappings = bags::parse_mapping(&contents)
        .map_err(|e| anyhow::Error::msg(format!("{}", &e)))?.1;
    let inverted_mappings = invert_mappings(&mappings);

    let mut num = 0;
    for parent in ParentIterator::new(&inverted_mappings, "shiny gold".into()) {
        println!("p {}", parent);
        num += 1;

    }
    println!("num {}", num);

    let mut num_children = 0;
    for (child, count) in ChildIterator::new(&mappings, "shiny gold".into()) {
        println!("c {} x {}", child, count);
        num_children += count;
    }
    println!("children {}", num_children);

    Ok(())
}
