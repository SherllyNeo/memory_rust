use random_string::generate;
use std::{thread, time};
use terminal::{Clear, Action};
use text_io::read;


pub fn memory_numbers(length: usize,sleep_per_digit: usize) {


    let terminal = terminal::stdout();



    let charset = "1234567890";
    let numbers_to_mem = generate(length, charset);
    let sleep_time = sleep_per_digit*length;
    let sleep_time_u64: u64 = sleep_time as u64;


    println!("{}", numbers_to_mem);
    println!("you have {} amount of second(s) to memorise these digits",sleep_time);
    thread::sleep(time::Duration::from_secs(sleep_time_u64));


     terminal.act(Action::ClearTerminal(Clear::All));
     println!("okay print what you memorised");


    let guess_trim: String = read!("{}\n");


     if guess_trim == numbers_to_mem {

         println!("well done you got it correct, the number was {}",numbers_to_mem);

     }
     else {
         println!("wow you're trash, the number was {}",numbers_to_mem);


     }



}
