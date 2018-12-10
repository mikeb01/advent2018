#[macro_use] extern crate text_io;
extern crate chrono;

pub mod advent1;
pub mod advent2;
pub mod advent3;
pub mod advent4;
pub mod advent5;
pub mod advent6;

fn main() {
//    advent1::advent1a();
//    advent1::advent1b();
//    advent2::advent2a();
//    advent2::advent2b();
    match advent3::advent3a() {
        Ok(()) => {}
        Err(e) => println!("{}", e)
    }
    match advent4::advent4a() {
        Ok(n) => println!("Advent4a: {:?}", n),
        Err(e) => println!("{}", e)
    }
    match advent5::advent5a() {
        Ok(n) => println!("Advent5a: {:?}", n),
        Err(e) => println!("{}", e)
    }
    match advent5::advent5b() {
        Ok(n) => println!("Advent5b: {:?}", n),
        Err(e) => println!("{}", e)
    }

    match advent6::advent6a() {
        Ok(n) => println!("Advent6b: {:?}", n),
        Err(e) => println!("{}", e)
    }

//    return Ok(());
//
//    let x = "dabAcCaCBAcCcaDA";
//    let mut string = x.to_string();
//    let mut input = unsafe { string.as_mut_vec() };
//
//    advent5::advent5b_part(input);
}