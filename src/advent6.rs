use std::io::{BufRead, BufReader, Error};
use std::fs::File;
use std::iter::FromIterator;
use std::collections::VecDeque;

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
enum GridEntry {
    Empty,
    Near(usize),
    Neutral,
    Infinity
}

pub fn advent6a() -> Result<usize, Error> {
    let f = File::open("input6.txt")?;

    let mut points: Vec<(usize, usize)> = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for buffer in BufReader::new(f).lines() {
        let line = buffer?;

        let x: usize;
        let y: usize;

        scan!(line.bytes() => "{}, {}", x, y);

        max_x = max_x.max(x);
        max_y = max_y.max(y);

        points.push((x, y));
    }

    let mut grid_raw = vec![GridEntry::Empty; (max_x + 1) * (max_y + 1)];
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(max_y + 1).collect();
    let mut grid: &mut [&mut [_]] = grid_base.as_mut_slice();

    for px in 0..grid.len() {
        for py in 0..grid[0].len() {
            let mut distances = Vec::from_iter(
                points.iter().enumerate()
                    .map(|x| (manhattan_distance((px, py), ((x.1).0, (x.1).1)), x.0)));
            distances.sort_by(|a, b| a.0.cmp(&b.0));

            let entry = if distances[0].0 == distances[1].0 {
                GridEntry::Neutral
            } else {
                GridEntry::Near(distances[0].1)
            };

            grid[px][py] = entry;
        }
    }

    for px in 0..grid.len() {
        let target = grid[px][0];

        if target != GridEntry::Infinity {
            flood_fill(&mut grid, (px, 0), |x| x == target, GridEntry::Infinity);
        }
    }

    for px in 0..grid.len() {
        let last_y = grid[px].len() - 1;
        let target = grid[px][last_y];

        if target != GridEntry::Infinity {
            flood_fill(&mut grid, (px, last_y), |x| x == target, GridEntry::Infinity);
        }
    }

    for py in 0..grid[0].len() {
        let target = grid[0][py];

        if target != GridEntry::Infinity {
            flood_fill(&mut grid, (0, py), |x| x == target, GridEntry::Infinity);
        }
    }

    let x_last = grid.len() - 1;
    for py in 0..grid[x_last].len() {
        let target = grid[x_last][py];

        if target != GridEntry::Infinity {
            flood_fill(&mut grid, (x_last, py), |x| x == target, GridEntry::Infinity);
        }
    }

    let mut counts: Vec<usize> = vec![0; points.len()];

    for px in 0..grid.len() {
        for py in 0..grid[0].len() {

            match grid[px][py] {
                GridEntry::Near(n) => counts[n] += 1,
                _ => {}
            }
        }
    }

    return Ok(*counts.iter().max().unwrap_or(&0));
}

fn manhattan_distance(p: (usize, usize), q: (usize, usize)) -> usize {
    let x_diff = if p.0 < q.0 { q.0 - p.0 } else { p.0 - q.0 };
    let y_diff = if p.1 < q.1 { q.1 - p.1 } else { p.1 - q.1 };
    return x_diff + y_diff;
}

fn flood_fill<P>(grid: &mut [&mut [GridEntry]], node: (usize, usize), predicate: P, replacement: GridEntry) where
        P: Fn(GridEntry) -> bool {
//fn flood_fill(grid: &mut [&mut [GridEntry]], node: (usize, usize), target: GridEntry, replacement: GridEntry) {

    if !predicate(grid[node.0][node.1]) {
        return;
    }

    grid[node.0][node.1] = replacement;

    let mut node_q: VecDeque<(usize, usize)> = VecDeque::new();

    node_q.push_back(node);

    loop {
        let current_node = node_q.pop_front();
        if current_node.is_none() {
            break;
        }

        let n = current_node.unwrap();

        if 1 <= n.0 {
            let west = (n.0 - 1, n.1);
            check_paint_queue_node(grid, west, &predicate, replacement, &mut node_q)
        }

        if n.0 < (grid.len() - 1) {
            let east = (n.0 + 1, n.1);
            check_paint_queue_node(grid, east, &predicate, replacement, &mut node_q)
        }

        if 1 <= n.1 {
            let north = (n.0, n.1 - 1);
            check_paint_queue_node(grid, north, &predicate, replacement, &mut node_q)
        }

        if n.1 < (grid[n.0].len() - 1) {
            let south = (n.0, n.1 + 1);
            check_paint_queue_node(grid, south, &predicate, replacement, &mut node_q)
        }
    }
}

fn check_paint_queue_node<P>(
    grid: &mut [&mut [GridEntry]],
    candidate: (usize, usize),
    predicate: &P, replacement: GridEntry,
    node_q: &mut VecDeque<(usize, usize)>) where
        P: Fn(GridEntry) -> bool {

    if predicate(grid[candidate.0][candidate.1]) {
        grid[candidate.0][candidate.1] = replacement;
        node_q.push_back(candidate);
    }
}