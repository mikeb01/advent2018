use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashSet;

fn run(stream: &mut BufRead, seen: &mut HashSet<i32>, current_value: i32) -> (i32, Option<i32>) {
    let mut buffer = String::new();
    let mut value = current_value;


    loop {
        match stream.read_line(&mut buffer) {
            Ok(n) => {
                let b = buffer.trim();

                if n == 0 || b.len() == 0 {
                    break;
                }

                let v = i32::from_str_radix(buffer.trim(), 10);

                match v {
                    Ok(n) => value += n,
                    Err(e) => println!("Parse: {}", e)
                }

                if seen.contains(&value) {
                    return (value, Some(value));
                }

                seen.insert(value);
            }
            Err(e) => {
                println!("Read: {}", e);
                break;
            }
        }

        buffer.clear();
    }

    return (value, None);
}

pub fn advent1b() {
    let mut seen: HashSet<i32> = HashSet::new();
    let mut current_value = 0;
    loop {
        match File::open("/home/barkerm/sources/rust/advent2018/input1a.txt") {
            Ok(file) => {
                let mut f = BufReader::new(file);
                let (value, result) = run(&mut f, &mut seen, current_value);

                match result {
                    Some(n) => {
                        println!("First repeated freq = {}", n);
                        break;
                    }
                    _ => current_value = value
                }
            }
            Err(e) => {
                println!("{}", e);
                break;
            }
        }
    }
}

pub fn advent1a() {
    let mut seen: HashSet<i32> = HashSet::new();
    match File::open("/home/barkerm/sources/rust/advent2018/input1a.txt") {
        Ok(file) => {
            let mut f = BufReader::new(file);
            let (value, _result) = run(&mut f, &mut seen, 0);
            println!("First value = {}", value);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}