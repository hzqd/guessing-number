use aoko::{no_std::pipelines::tap::Tap, standard::functions::fun::read_line};
use itertools::Itertools;
use rand::{thread_rng, Rng};
use std::process;

fn main() {
    'start: loop {
        print!("###########################################################\n");
        print!("##  Welcome to the Guessing Number Game!                 ##\n");
        print!("##  Each game produces a random number from 0 to 100.    ##\n");
        print!("##  You can only use >,>=,==,<,<= symbols and numbers    ##\n");
        print!("##  to ask me questions. Like >50, <=38, ==47 etc.       ##\n");
        print!("##  I can only answer yes or no!                         ##\n");
        println!("###########################################################");

        let secret_number = thread_rng().gen_range(1u8..=100);
        let mut ask_times = 1;

        loop {
            println!("Please input your guess:");
            let input = read_line();

            match input.trim_end().as_bytes() {
                // <=
                [60, 61, ..] => {
                    if match input
                        .chars()
                        .tap_mut(|c| c.next())
                        .tap_mut(|c| c.next())
                        .join("")
                        .trim_end()
                        .parse::<u8>()
                    {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Wrong input, try again.");
                            continue;
                        }
                    } >= secret_number
                    {
                        println!("Yes! You guessed {ask_times} times!")
                    } else {
                        println!("No! You guessed {ask_times} times!")
                    }
                }

                // >=
                [62, 61, ..] => {
                    if match input
                        .chars()
                        .tap_mut(|c| c.next())
                        .tap_mut(|c| c.next())
                        .join("")
                        .trim_end()
                        .parse::<u8>()
                    {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Wrong input, try again.");
                            continue;
                        }
                    } <= secret_number
                    {
                        println!("Yes! You guessed {ask_times} times!")
                    } else {
                        println!("No! You guessed {ask_times} times!")
                    }
                }

                // ==
                [61, 61, ..] => {
                    if match input
                        .chars()
                        .tap_mut(|c| c.next())
                        .tap_mut(|c| c.next())
                        .join("")
                        .trim_end()
                        .parse::<u8>()
                    {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Wrong input, try again.");
                            continue;
                        }
                    } == secret_number
                    {
                        println!("Yes! You guessed {ask_times} times!")
                    } else {
                        println!("No! You guessed {ask_times} times!")
                    }
                }

                // <
                [60, ..] => {
                    if match input
                        .chars()
                        .tap_mut(|c| c.next())
                        .join("")
                        .trim_end()
                        .parse::<u8>()
                    {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Wrong input, try again.");
                            continue;
                        }
                    } > secret_number
                    {
                        println!("Yes! You guessed {ask_times} times!")
                    } else {
                        println!("No! You guessed {ask_times} times!")
                    }
                }

                // >
                [62, ..] => {
                    if match input
                        .chars()
                        .tap_mut(|c| c.next())
                        .join("")
                        .trim_end()
                        .parse::<u8>()
                    {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Wrong input, try again.");
                            continue;
                        }
                    } < secret_number
                    {
                        println!("Yes! You guessed {ask_times} times!")
                    } else {
                        println!("No! You guessed {ask_times} times!")
                    }
                }

                _num => {
                    if match input.trim_end().parse::<u8>() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Wrong input, try again.");
                            continue;
                        }
                    } == secret_number
                    {
                        print!("#####################################\n");
                        print!("##   Bingo!   You guessed right!   ##\n");
                        print!("#####################################\n");
                        print!("You asked {ask_times} times in total!\n");
                        println!("Input n to start a new game and q to exit the game!");
                        match read_line().trim_end() {
                            "n" => continue 'start,
                            "q" => process::exit(0),
                            _ => panic!("Wrong input, game exit."),
                        }
                    } else {
                        println!("Sorry! You guessed wrong. Guessed {ask_times} times!")
                    }
                }
            }

            ask_times += 1;
        }
    }
}