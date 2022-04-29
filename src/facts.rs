use super::api_call;
use std::{thread};
use terminal::{Clear, Action};

pub fn memory_facts(length: usize, time: usize) {

    let terminal = terminal::stdout();

    // sexy anonymous function
    let random_fact = || {

        let fact = api_call::api_get_request("https://uselessfacts.jsph.pl/random.txt?language=en");
        let fact = fact + " \n ";

        return fact;

    };

    // generating facts

    let mut facts_line: String = "".to_string();

    for  fact in 0..length{
        // random_fact();
        // let random_fact_str = &random_fact(); // to push random strings into vector
        facts_line = facts_line + &random_fact();
        // facts_vec.push(random_fact_str);
        if fact < length-1 {
            facts_line = facts_line + ",  ";
        }

        // "looks,  like,  this"

    }



    let facts_text_block = facts_line;



    // wait for {time} seconds

    let mem_time_u64 = (length*time).try_into().unwrap();
    let mem_dur = std::time::Duration::from_secs(mem_time_u64);

    println!("Memorise this:\n{}\nYou have {} seconds.", facts_text_block, length*time);

    thread::sleep(mem_dur);

    terminal.act(Action::ClearTerminal(Clear::All)).map_err(|err| println!("{:?}", err)).ok();

    }
