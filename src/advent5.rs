
use std::io::{Read, Error};
//use std::io::{Error};
use std::fs::File;
use std::iter::FromIterator;
//use chrono::{Timelike, NaiveDateTime};
//use std::cmp::Ordering;
//use std::collections::HashMap;


pub fn advent5a() -> Result<usize, Error> {
    let mut file = File::open("input5.txt")?;
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data)?;

    full_polymer_react(&mut data);

    return Ok(data.len())
}

pub fn advent5b() -> Result<usize, Error> {
    let mut file = File::open("input5.txt")?;
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data)?;

    return advent5b_part(&mut data)
}

pub fn advent5b_part(data: &mut Vec<u8>) -> Result<usize, Error> {
    let char_start = 'a' as u8;
    let char_end = ('z' as u8) + 1;
    let mut result = data.len();

    for c in char_start..char_end {
        let mut copy:Vec<u8> = partial_polymer_react(data, c, c - 32);

        full_polymer_react(&mut copy);

        result = result.min(copy.len())
    }
    return Ok(result)
}


fn full_polymer_react(data: &mut Vec<u8>) {
    let mut index = data.len() - 2;
    loop {
        if index + 1 < data.len() {
            let char_difference = data[index] as i32 - data[index + 1] as i32;
            let abs_char_difference = if char_difference < 0 { -char_difference } else { char_difference };

            if abs_char_difference == 32 {
                data.remove(index + 1);
                data.remove(index);
            } else if index == 0 {
                break;
            } else {
                index -= 1;
            }
        } else {
            index -= 1;
        }
    }
}

fn partial_polymer_react(data: &Vec<u8>, char_upper: u8, char_lower: u8) -> Vec<u8> {
    return Vec::from_iter(data.into_iter()
        .filter(|&&c| c != char_upper && c != char_lower)
        .map(|&c| c));
}
