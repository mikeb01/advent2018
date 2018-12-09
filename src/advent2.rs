use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::BTreeMap;


fn hash_code(stream: &mut BufRead) -> i32 {
    let mut twos = 0;

    let mut threes = 0;

    loop {
        let mut buffer = String::new();
        match stream.read_line(&mut buffer) {
            Ok(n) => {

                if n < 1 {
                    break;
                }

                let mut counts: BTreeMap<char, i32> = BTreeMap::new();

                buffer.trim().chars().for_each(|c| {
                    let new_count = match counts.get(&c) {
                        Some(n) => n + 1,
                        None => 1
                    };

                    counts.insert(c, new_count);

                    return ();
                });

                if counts.values().any(|x| *x == 2) {
                    twos += 1;
                }
                if counts.values().any(|x| *x == 3) {
                    threes += 1;
                }

                counts.clear();
            }
            Err(_e) => break
        }

    }

    return twos * threes;
}

pub fn advent2a() {
    match File::open("input2.txt") {
        Ok(file) => println!("{}", hash_code(&mut BufReader::new(file))),
        Err(e) => println!("{}", e)
    }
}

fn find_single_diff_rec(stream: &mut BufRead, inputs: &mut Vec<String>) -> Option<String> {

    let mut buffer = String::new();

    match stream.read_line(&mut buffer) {
        Ok(n) => {
            if n < 1 {
                return None;
            }

            match inputs.iter()
                .map(|x| {
                    let s: String = x.trim().chars().zip(buffer.trim().chars()).filter(|(a, b)| a == b).map(|(a, _)| a).collect();
                    return s;
                })
//                .map(|x| {
//                    return x.trim().chars().zip(buffer.trim().chars()).filter(|(a, b)| a == b).map(|(a, _)| a).collect();
//                })
                .find(|s| s.len() == (buffer.trim().len() - 1)) {

                Some(a) => {
                    println!("{}", a.trim());
                    return None
                }
                None => {}
            };

            inputs.push(buffer);

            find_single_diff_rec(stream, inputs);
        }
        Err(e) => println!("{}", e)
    }

    return None
}

fn find_single_diff(stream: &mut BufRead) -> String {

    let mut vec = Vec::new();

    find_single_diff_rec(stream, &mut vec);
    return String::new();
}

pub fn advent2b() {
    match File::open("input2.txt") {
        Ok(file) => println!("{}", find_single_diff(&mut BufReader::new(file))),
        Err(e) => println!("{}", e)
    }
}