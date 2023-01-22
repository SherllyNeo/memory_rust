use random_string::generate;
use std::{thread, time};
use super::input_wrapper;

pub fn memory_numbers(length: usize, time: usize){


    let charset = "1234567890"; //character set used for random generation
    let numbers_to_mem = generate(length, charset);
    let sleep_time = time*length;
    let sleep_time_u64: u64 = sleep_time as u64;


    println!("{}", numbers_to_mem);
    println!("you have {} amount of second(s) to memorise these digits",sleep_time);
    thread::sleep(time::Duration::from_secs(sleep_time_u64));

    for _ in 1..50 {
    println!("\n");
    }
    println!("okay print what you memorised");


    let guess_trim: String = input_wrapper::get_input();


     if guess_trim == numbers_to_mem {

         println!("well done you got it correct, the number was {}",numbers_to_mem);

     }
     else {
         println!("wow you're trash, the number was {}",numbers_to_mem);


     }
     println!("test")



}
