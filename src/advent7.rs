use std::io::{BufRead, BufReader, Error};
use std::fs::File;
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

pub fn advent7b() -> Result<i32, Error> {

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

    let mut workers = [('x', 0); 5];
    let mut time_taken = 0;

    while !tree.is_empty() {
        let mut to_remove: Vec<char> = Vec::new();
        tree.iter().for_each(|(k, v)| if v.is_empty() { to_remove.push(*k) });

        to_remove.sort_by(|a, b|  a.cmp(b).reverse() );
        to_remove.dedup();

        while !to_remove.is_empty() {
            let next_free_worker = workers.iter().position(|(_step, time)| *time <= 0);
            if next_free_worker.is_some() {
                let candidate_step = to_remove.pop().unwrap();
                if workers.iter().find(|(c, _time)| *c == candidate_step).is_none() {
                    let time_required = (candidate_step as i32 - 'A' as i32) + 61;
                    workers[next_free_worker.unwrap()] = (candidate_step, time_required);
                    tree.remove(&candidate_step);
                }
            } else {
                break;
            }
        }

        for i in 0..workers.len() {
            let (c, time_remaining) = workers[i];
            if time_remaining == 1 {
                tree.values_mut().for_each(|v| { v.retain(|&x| x != c ); });
                workers[i] = ('x', 0);
            } else {
                workers[i] = (c, time_remaining - 1);
            }
        }

        time_taken += 1;
    }

    let remaining = workers.iter().map(|(_c, time)| time).max().unwrap_or(&0);

    return Ok(time_taken + remaining);
}