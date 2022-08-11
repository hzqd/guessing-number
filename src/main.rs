use aoko::standard::functions::fun::read_line;
use itertools::Itertools;
use rand::{thread_rng, Rng};
use std::process;

macro_rules! match_op {
    ($input:ident $num:ident $ask:ident, $($pattern:pat, $idx:expr, $op:tt $($succ:expr)?);+ $(;)?) => {
        match $input.as_slice() {
            $(
                $pattern => {
                    if match $input[$idx..].iter().join("").parse::<u8>() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Wrong input, try again.");
                            continue;
                        }
                    } $op $num {
                        println!("Yes! You guessed {} times!", $ask);
                        $($succ)?
                    } else {
                        println!("No! You guessed {} times!", $ask);
                    }
                },
            )+
        }
    };
}

macro_rules! n {
    ($new:tt) => {
        match read_line().trim_end() {
            "n" => continue $new,
            "q" => process::exit(0),
            _ => panic!("Wrong input, game exit."),
        }
    };
}

fn main() {
    'start: loop {
        print!("########################################################\n");
        print!("##  Welcome to the Guessing Number Game!              ##\n");
        print!("##  Each game produces a random number from 1 to 100. ##\n");
        print!("##  You can only use >,>=,=,<,<= symbols and numbers  ##\n");
        print!("##  or pure numbers to ask me questions.              ##\n");
        print!("##  Like >=50, <=38, >47, <63, =52, 58 etc.           ##\n");
        print!("##  I can only answer you yes or no.                  ##\n");
        print!("########################################################\n");

        let secret_number = thread_rng().gen_range(1..=100);
        let mut ask_times = 1;

        loop {
            println!("Please input your guess:");
            let input = read_line().trim_end().chars().collect_vec();
            let succ = || {
                print!("#####################################\n");
                print!("##   Bingo!   You guessed right!   ##\n");
                print!("#####################################\n");
                print!("You asked {ask_times} times in total!\n");
                println!("Input n to start a new game or q to exit the game!");
            };

            match_op! {
                input   secret_number   ask_times,

                ['<', '=', ..], 2, >=;  ['<', ..], 1, >;
                ['>', '=', ..], 2, <=;  ['>', ..], 1, <;
                ['=', ..], 1, == { succ(); n!('start); };
                _pure_num, 0, == { succ(); n!('start); };
            }
            
            ask_times += 1;
        }
    }
}