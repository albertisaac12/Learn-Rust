// unrecoverable errors panic

// unwrap := unwrap(self): T
// expect := expect(self, msg: &str): T

use core::{num, panic};
use std::{fs::File, io};
mod isSeven;


use isSeven::{is_Seven, is_SevenWithError};

fn main(){
    // panic!("ends");

    // let f = File::open("nll.txt").unwrap();
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    match number.trim().parse::<i32>() {
        Ok(num) => {
            if num < 0 || num > 10 {
                panic!();
            }
        }
        Err(_)=> {
            panic!("failed");
        }
    }

    // let result = is_Seven(&number);

    // println!("Number is 7 {}", result);

    is_SevenWithError(&number).expect("you did not enter 7");
    // is_SevenWithError(&number).unwrap();


    

}

