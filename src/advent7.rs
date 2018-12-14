use std::io::{BufRead, BufReader, Error};
use std::fs::File;
use std::iter::FromIterator;
use std::collections::VecDeque;
use std::collections::BTreeMap;



pub fn advent7a() -> Result<String, Error> {

    let f = File::open("input7.txt")?;

    let mut tree: BTreeMap<char, Vec<char>> = BTreeMap::new();

    for buffer in BufReader::new(f).lines() {

        let line = buffer?;
        let depends_on: char;
        let c: char;
        scan!(line.bytes() => "Step {} must be finished before step {} can begin.", depends_on, c);

        if !tree.contains_key(&depends_on) {
            tree.insert(depends_on, Vec::new());
        }

        if !tree.contains_key(&c) {
            tree.insert(c, vec![depends_on]);
        } else {
            tree.get_mut(&c).unwrap().push(depends_on);
        }
    }

    let mut removed: Vec<char> = Vec::new();

    while !tree.is_empty() {
        let mut to_remove: Vec<char> = Vec::new();
        tree.iter().for_each(|(k, v)| if v.is_empty() { to_remove.push(*k) });

        to_remove.sort_by(|a, b|  a.cmp(b).reverse() );
        to_remove.dedup();

        let c = to_remove.pop().unwrap();
        tree.remove(&c);
        removed.push(c);
        tree.values_mut().for_each(|v| { v.retain(|&x| x != c ); });
    }

    let s: String = removed.into_iter().collect();

    return Ok(s);
}

pub fn advent7b() -> Result<String, Error> {

    let f = File::open("input7.txt")?;

    let mut tree: BTreeMap<char, Vec<char>> = BTreeMap::new();

    for buffer in BufReader::new(f).lines() {

        let line = buffer?;
        let depends_on: char;
        let c: char;
        scan!(line.bytes() => "Step {} must be finished before step {} can begin.", depends_on, c);

        if !tree.contains_key(&depends_on) {
            tree.insert(depends_on, Vec::new());
        }

        if !tree.contains_key(&c) {
            tree.insert(c, vec![depends_on]);
        } else {
            tree.get_mut(&c).unwrap().push(depends_on);
        }
    }

    let mut removed: Vec<char> = Vec::new();

    let mut workers = [('x', 0); 5];

    while !tree.is_empty() {
        let mut to_remove: Vec<char> = Vec::new();
        tree.iter().for_each(|(k, v)| if v.is_empty() { to_remove.push(*k) });

        if !to_remove.is_empty() {
            let next_free_worker = workers.iter().position(|(step, time)| time <= 0);
            if next_free_worker.is_some() {
                to_remove.sort_by(|a, b|  a.cmp(b).reverse() );
                to_remove.dedup();

                let c = to_remove.pop().unwrap();
                tree.remove(&c);
                removed.push(c);
                tree.values_mut().for_each(|v| { v.retain(|&x| x != c ); });
            }
        }

    }

    let s: String = removed.into_iter().collect();

    return Ok(s);
}