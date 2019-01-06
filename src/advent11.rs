use std::io::Error;
use std::time::SystemTime;


fn power(x: i32, y: i32, serial: i32) -> i32 {
    let rack_id = x + 10;
    let mut power_level = rack_id * y;
    power_level += serial;
    power_level *= rack_id;

    return ((power_level / 100) % 10) - 5;
}

pub fn advent11a() -> Result<(i32, i32), Error> {
    let serial = 4172;

    let mut max_total_power = std::i32::MIN;
    let mut co_ords = (0, 0);

    for y in 1..=298 {
        for x in 1..=298 {
            let total_power =
                power(x, y, serial) +
                    power(x + 1, y, serial) +
                    power(x + 2, y, serial) +
                    power(x, y + 1, serial) +
                    power(x + 1, y + 1, serial) +
                    power(x + 2, y + 1, serial) +
                    power(x, y + 2, serial) +
                    power(x + 1, y + 2, serial) +
                    power(x + 2, y + 2, serial);

            if total_power > max_total_power {
                max_total_power = total_power;
                co_ords = (x, y);
            }
        }
    }

    return Ok(co_ords);
}

pub fn advent11b() -> Result<(i32, i32, i32), Error> {
    let serial = 4172;

    let mut max_total_power = std::i32::MIN;
    let mut co_ords = (0, 0, 0);

    for y in 1..=300 {

        let t0 = SystemTime::now();

        for x in 1..=300 {

            let max_square = 301 - x.max(y);

            let mut prev_total_power = 0;

            for size in 1..=max_square {

                let mut total_power = prev_total_power;

                for w in 0..size {
                    total_power += power(x + w, y + (size - 1), serial);
                }

                for w in 0..(size - 1) {
                    total_power += power(x + (size - 1), y + w, serial);
                }

//                println!("x = {}, y = {}, size = {}, power = {}", x, y, size, total_power);

                if total_power > max_total_power {
                    max_total_power = total_power;
                    co_ords = (x, y, size);
                }

                prev_total_power = total_power;
            }
        }

        let t1 = SystemTime::now();
    }

    return Ok(co_ords);
}