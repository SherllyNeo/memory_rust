mod numbers;
mod words;
mod cards;
mod input_wrapper;
mod facts;
mod api_call;
mod names;
use std::env;



/*

   Main function to direct user arguements


*/
fn main() {
    // turn arguments into a vector
    let args: Vec<String> = env::args().collect();
    // according to format, first argument should be the game mode
    let mode = &args[1];


    if mode == "-numbers" {
    let amount_of_digits = args[2].parse::<usize>().unwrap();
    let amount_of_seconds_per_digits = args[3].parse::<usize>().unwrap();
    numbers::memory_numbers(amount_of_digits,amount_of_seconds_per_digits);
    }
    else if mode == "-words" {
    let amount_of_words = args[2].parse::<usize>().unwrap();
    let amount_of_seconds_per_word = args[3].parse::<usize>().unwrap();
    words::memory_words(amount_of_words,amount_of_seconds_per_word);

    }
    else if mode == "-cards" {
    let amount_of_cards = args[2].parse::<usize>().unwrap();
    let amount_of_seconds_per_card = args[3].parse::<usize>().unwrap();
    cards::memory_cards(amount_of_cards,amount_of_seconds_per_card);

    }
    else if mode == "-facts" {

    let amount_of_facts = args[2].parse::<usize>().unwrap();
    let amount_of_seconds_per_facts = args[3].parse::<usize>().unwrap();
    facts::memory_facts(amount_of_facts,amount_of_seconds_per_facts);
    }
     else if mode == "-names" {
        let amount_of_names = args[2].parse::<usize>().unwrap();
        let amount_of_seconds_per_names = args[3].parse::<usize>().unwrap();
        names::memory_names(amount_of_names,amount_of_seconds_per_names);
    }
    else if mode == "-objects" {
       let amount_of_objects = args[2].parse::<usize>().unwrap();
       let amount_of_seconds_per_objects = args[3].parse::<usize>().unwrap();
       names::memory_names(amount_of_objects,amount_of_seconds_per_objects);
    }

    else {
        println!("please use the format -game amount time with the games list being numbers, words, cards.");
    }
}
