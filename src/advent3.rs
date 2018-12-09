use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::fs::File;

fn find_overlaps(stream: &mut BufRead, cloth: &mut [[u32; 1000]; 1000]) -> Result<i32, Error> {

    for buffer in stream.lines() {

        let line = buffer?;

        let num: i32;
        let off_x: usize;
        let off_y: usize;
        let width: usize;
        let height: usize;

        scan!(line.bytes() => "#{} @ {},{}: {}x{}", num, off_x, off_y, width, height);

        let max_x = off_x + width;
        let max_y = off_y + height;

        for x in off_x..max_x {
            for y in off_y..max_y {
                cloth[x][y] += 1;
            }
        }
    }

    let mut count = 0;

    for row in cloth.iter() {
        for v in row.iter() {
            if *v > 1_u32 {
                count += 1;
            }
        }
    }

    return Ok(count);
}

fn find_ok_pattern(stream: &mut BufRead, cloth: &[[u32; 1000]; 1000]) -> Result<i32, Error> {

    for buffer in stream.lines() {

        let line = buffer?;

        let num: i32;
        let off_x: usize;
        let off_y: usize;
        let width: usize;
        let height: usize;

        scan!(line.bytes() => "#{} @ {},{}: {}x{}\n", num, off_x, off_y, width, height);

        if is_ok(cloth, off_x, off_y, width, height) {
            return Ok(num);
        }
    }

    return Err(Error::new(ErrorKind::NotFound, ""));
}

fn is_ok(cloth: &[[u32; 1000]; 1000], off_x: usize, off_y: usize, width: usize, height: usize) -> bool {
    let max_x = off_x + width;
    let max_y = off_y + height;

    for x in off_x..max_x {
        for y in off_y..max_y {
            if 1 != cloth[x][y] {
                return false;
            }
        }
    }

    return true;
}

pub fn advent3a() -> Result<(), Error> {

    let mut cloth: [[u32; 1000]; 1000] = [[0; 1000]; 1000];

    println!("advent3a: {}", find_overlaps(&mut BufReader::new(File::open("input3.txt")?), &mut cloth)?);
    println!("advent3b: {}", find_ok_pattern(&mut BufReader::new(File::open("input3.txt")?), &cloth)?);
//    match File::open("input3.txt") {
//        Ok(file) => ,
//        Err(e) => println!("{}", e)
//    };

    return Ok(());


//    return;
}