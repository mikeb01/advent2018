use std::io::{BufRead, BufReader, Error};
use std::fs::File;
use std::iter::Iterator;

pub fn advent10a() -> Result<usize, Error> {

    let f = File::open("input10.txt")?;

    let mut positions: Vec<(i32, i32)> = Vec::new();
    let mut velocities: Vec<(i32, i32)> = Vec::new();

    for buffer in BufReader::new(f).lines() {
        let line = buffer?;

        let x_s: String;
        let y_s: String;
        let dx_s: String;
        let dy_s: String;

        scan!(line.bytes() => "position=<{},{}> velocity=<{},{}>", x_s, y_s, dx_s, dy_s);

        let x = i32::from_str_radix(x_s.trim(), 10).unwrap();
        let y = i32::from_str_radix(y_s.trim(), 10).unwrap();
        let dx = i32::from_str_radix(dx_s.trim(), 10).unwrap();
        let dy = i32::from_str_radix(dy_s.trim(), 10).unwrap();

        positions.push((x, y));
        velocities.push((dx, dy));
    }

    let mut prev_width = std::usize::MAX;
    let mut prev_height = std::usize::MAX;
    let mut prev_min = (0i32, 0i32);
    let mut wait_time = 0;

    for j in 0..100000 {

        let mut min = (std::i32::MAX, std::i32::MAX);
        let mut max = (std::i32::MIN, std::i32::MIN);

        for &(x, y) in positions.iter() {
            min = (min.0.min(x), min.1.min(y));
            max = (max.0.max(x), max.1.max(y));
        }

        let width = (max.0 - min.0) as usize;
        let height = (max.1 - min.1) as usize;

        if prev_width < width || prev_height < height {

            for i in 0..positions.len() {
                let x_p = positions[i].0 - velocities[i].0;
                let y_p = positions[i].1 - velocities[i].1;

                positions[i] = (x_p, y_p);
            }

            wait_time = j - 1;

            break;
        }

        prev_width = width;
        prev_height = height;
        prev_min = min;

        for i in 0..positions.len() {
            let x_p = positions[i].0 + velocities[i].0;
            let y_p = positions[i].1 + velocities[i].1;

            positions[i] = (x_p, y_p);
        }
    }

    let grid_width = prev_width + 1;
    let grid_height = prev_height + 1;

    let mut grid = vec!['.'; grid_width * grid_height];

    for &(x, y) in positions.iter() {

        let x_n : usize = (x - prev_min.0) as usize;
        let y_n : usize = (y - prev_min.1) as usize;

        if x_n >= grid_width {
            panic!("x_n: {}, width: {}", x_n, grid_width);
        }
        if y_n >= grid_height {
            panic!("y_n: {}, height: {}", y_n, grid_height);
        }

        grid[x_n + (y_n * grid_width)] = '#';
    }

    for j in 0..grid_height {
        for i in 0..grid_width {
            print!("{}", grid[i + (j * grid_width)]);
        }
        println!();
    }

    return Ok(wait_time);
}