#[macro_use] extern crate text_io;
extern crate chrono;

pub mod advent1;
pub mod advent2;
pub mod advent3;
pub mod advent4;
pub mod advent5;
pub mod advent6;
pub mod advent7;
pub mod advent8;
pub mod advent9;

fn main() {
//    advent1::advent1a();
//    advent1::advent1b();
//    advent2::advent2a();
//    advent2::advent2b();
//    match advent3::advent3a() {
//        Ok(()) => {}
//        Err(e) => println!("{}", e)
//    }
//    match advent4::advent4a() {
//        Ok(n) => println!("Advent4a: {:?}", n),
//        Err(e) => println!("{}", e)
//    }
//    match advent5::advent5a() {
//        Ok(n) => println!("Advent5a: {:?}", n),
//        Err(e) => println!("{}", e)
//    }
//    match advent5::advent5b() {
//        Ok(n) => println!("Advent5b: {:?}", n),
//        Err(e) => println!("{}", e)
//    }
//
//    match advent6::advent6a() {
//        Ok(n) => println!("Advent6a: {:?}", n),
//        Err(e) => println!("{}", e)
//    }
//
//    match advent6::advent6b() {
//        Ok(n) => println!("Advent6b: {:?}", n),
//        Err(e) => println!("{}", e)
//    }

    match advent7::advent7a() {
        Ok(n) => println!("Advent7a: {:?}", n),
        Err(e) => println!("{}", e)
    }

    match advent7::advent7b() {
        Ok(n) => println!("Advent7b: {:?}", n),
        Err(e) => println!("{}", e)
    }

    match advent8::advent8a() {
        Ok(n) => println!("Advent8b: {:?}", n),
        Err(e) => println!("{}", e)
    }

    match advent9::advent9a() {
        Ok(n) => println!("Advent9a: {:?}", n),
        Err(e) => println!("{}", e)
    }

    match advent9::advent9b() {
        Ok(n) => println!("Advent9b: {:?}", n),
        Err(e) => println!("{}", e)
    }

//    advent9::advent9c();
}
