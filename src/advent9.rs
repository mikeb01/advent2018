use std::io::Error;
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
            players[player_index] += points + score;

        } else {
            let left = marbles[current].next;
            let right = marbles[left].next;
            let new_entry = marbles.len();

            marbles.push(Node { value: points, next: right, prev: left });

            marbles[left].next = new_entry;
            marbles[right].prev = new_entry;

            current = new_entry;
        }

    }

    return Ok(*players.iter().max().unwrap_or(&0));
}
