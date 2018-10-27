//External crates
extern crate rand;
#[macro_use]
extern crate text_io;
//rand, for generating random numbers
use rand::distributions::{Distribution, Range};
//std, for taking user input
use std::io::BufRead;
use std::process;
//use std::io::prelude::*;

fn main() {
    //Calls the rng functions(?) and sets the maximum value
    // of the die.
    let mut rng = rand::thread_rng();
    let mut throw = 0;
    let mut d_size = 0;

    //Intros
    println!("*******************************************");
    println!("            Dice Roll Helper");
    println!("*******************************************\n");
    //Offers exit condition
    println!("At any time, you can type '0'  to exit.\n");
    //Asks for the die size. d6, d20, etc.
    println!("What size dice?");
    let d_size: i32 = read!();
    if d_size < 1 || d_size > 100 {
        println!("Exiting");
        process::exit(0);
    }

    //Asks how many die should be thrown
    println!("How many?");
    let cast_num: i32 = read!();
    if cast_num < 1 || cast_num > 100 {
        println!("Exiting");
        process::exit(0);
    }

    // Set die size equal to user input
    let d_size_int: i32 = d_size;

    println!("\nResults");
    println!("*******************************************");
    //Control loop
    let mut roll_count = 1;
    while roll_count < cast_num + 1 {
        //Rolls the die of appropriate size
        println!("\nDie number {}: ", roll_count);
        let d_limit = d_size_int + 1;
        let die = Range::new(1, d_limit);
        throw = die.sample(&mut rng);
        println!("d{}: {}", d_size_int, throw);
        if d_size_int == 20 && throw == 20 {
            println!("Critical!");
        } else if d_size_int == 20 && throw == 1 {
            println!("Critical, except the bad kind!");
        }
        roll_count = roll_count + 1;
    }
}
