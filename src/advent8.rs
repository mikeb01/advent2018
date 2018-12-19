use std::io::{BufRead, BufReader, Error};
use std::fs::File;
use std::iter::Iterator;
//use std::iter::FromIterator;
//use std::collections::VecDeque;
//use std::collections::BTreeMap;


fn process<I>(i: &mut I, node_num : i32) -> Option<usize> where I : Iterator<Item = usize> {

    let child_nodes = i.next()?;
    let meta_nodes = i.next()?;

    let mut child_node_values: Vec<usize> = Vec::new();
    for _c in 0..child_nodes {
        child_node_values.push(process(i, node_num + 1)?);
    }

    let mut meta_node_values: Vec<usize> = Vec::new();
    for _c in 0..meta_nodes {
        meta_node_values.push(i.next()?);
    }

    let result = sum(child_nodes, &mut child_node_values, &mut meta_node_values);

    return result;
}

fn sum(child_nodes: usize, child_node_values: &Vec<usize>, meta_node_values: &Vec<usize>) -> Option<usize> {
    return if 0 == child_nodes {
        Some(meta_node_values.iter().sum())
    } else {
        let mut total = 0;

        for &meta in meta_node_values {
            if meta <= child_node_values.len() {
                total += child_node_values[meta - 1];
            }
        }

        Some(total)
    };
}

pub fn advent8a() -> Result<usize, Error> {

    let f = File::open("input8.txt")?;

    let result = BufReader::new(f).lines().next();

    if result.is_some() {

        let line = result.unwrap()?;

        let mut numbers = line.split_whitespace()
            .map(|s| usize::from_str_radix(s, 10))
            .filter(|s| s.is_ok())
            .map(|s| s.unwrap());

        let i: usize = process(&mut numbers, 0).unwrap();

        return Ok(i);
    }

    return Ok(0);
}

