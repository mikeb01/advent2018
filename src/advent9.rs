use std::io::{BufRead, BufReader, Error};
use std::fs::File;
use std::iter::Iterator;
//use std::iter::FromIterator;
//use std::collections::VecDeque;
//use std::collections::BTreeMap;



pub fn advent9a() -> Result<usize, Error> {
    run_game2(403, 71920)
}

pub fn advent9b() -> Result<usize, Error> {
    run_game2(403, 71920 * 100)
}

fn run_game(num_players: usize, max_points: usize) -> Result<usize, Error> {
    let mut players: Vec<usize> = vec![0; num_players];
    let mut marbles: Vec<usize> = Vec::with_capacity(max_points);
    marbles.push(0);
    marbles.push(1);
    let mut current = 1;
    for points in 2..(max_points + 1) {

        if points % 23 == 0 {
            let to_remove_index = if current < 7 { 7 - current } else { current - 7 };
            let removed = marbles.remove(to_remove_index);
            current = to_remove_index % marbles.len();
            let player_index = points % players.len();
            players[player_index] += (points + removed);
        } else {
            let mut next = current + 2;
            next = if next > marbles.len() { 1 } else { next };
            marbles.insert(next, points);

            current = next;
        }
    }

    return Ok(*players.iter().max().unwrap_or(&0));
}

struct Node {
    value: usize,
    next: usize,
    prev: usize
}

fn run_game2(num_players: usize, max_points: usize) -> Result<usize, Error> {
    let mut players: Vec<usize> = vec![0; num_players];
    let mut marbles: Vec<Node> = Vec::with_capacity(max_points + 1);

    marbles.push(Node { value: 0, next: 1, prev: 1 });
    marbles.push(Node { value: 1, next: 0, prev: 0 });

    let mut current = 1;
    let mut points = 2;
    let mut head = 0;

    for points in 2..(max_points + 1) {

        if points % 23 == 0 {

            let mut to_remove = current;
            for _i in 0..7 {
                to_remove = marbles[to_remove].prev;
            }

            let score = marbles[to_remove].value;
            let left = marbles[to_remove].prev;
            let right = marbles[to_remove].next;

            marbles[left].next = right;
            marbles[right].prev = left;
            current = right;

            let player_index = points % players.len();
            players[player_index] += (points + score);

        } else {
            let left = marbles[current].next;
            let right = marbles[left].next;
            let new_entry = marbles.len();

            marbles.push(Node { value: points, next: right, prev: left });

            marbles[left].next = new_entry;
            marbles[right].prev = new_entry;

            current = new_entry;
        }

//        print_marbles(&marbles, head, current);
    }

    return Ok(*players.iter().max().unwrap_or(&0));
}

fn print_marbles(marbles: &Vec<Node>, head: usize, current: usize) {

    let mut index = head;

    loop {
        if index == current {
            print!("({}) ", marbles[index].value);
        } else {
            print!("{} ", marbles[index].value);
        }

        index = marbles[index].next;

        if index == head {
            break;
        }
    }

    println!();
}