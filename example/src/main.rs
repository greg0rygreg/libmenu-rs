use libmenu_rs::{Menu, util::*};
use std::io::{self, Write};
use rand::Rng;

fn main() -> Result<(), String> {
    let mut o: i32 = 0;
    let mut rng = rand::rng();
    let menu = Menu::new("libdraw-rs 0.1.0 example", "1.1.1", vec!["print something", "print random number", "info"], None);
    clear();
    loop {
        menu.get_input(&mut o, true, true).ok();
        match o {
            0 => {
                clear();
                break;
            },
            1 => {
                clear();
                let mut s = String::new();
                print!("what to print: ");
                io::stdout().flush().ok();
                match io::stdin().read_line(&mut s) {
                    Ok(_) => {
                        clear();
                        println!("you typed: {}", s.trim());
                        sep();
                    },
                    Err(e) => {
                        clear();
                        error(format!("unable to read input from stdin: {}", e));
                        sep();
                    }
                }
            },
            2 => {
                clear();
                let mut s1 = String::new();
                let mut s2 = String::new();
                print!("min number: ");
                io::stdout().flush().ok();
                if let Err(e) = io::stdin().read_line(&mut s1) {
                    clear();
                    error(format!("unable to read input from stdin: {}", e));
                    sep();
                }
                print!("max number: ");
                io::stdout().flush().ok();
                if let Err(e) = io::stdin().read_line(&mut s2) {
                    clear();
                    error(format!("unable to read input from stdin: {}", e));
                    sep();
                }
                let rand = rng.random_range(s1.trim().parse::<i32>().unwrap_or(0)..s2.trim().parse::<i32>().unwrap_or(100) + 1);
                clear();
                println!("your number is: {}", rand);
                sep();
            },
            3 => {
                clear();
                println!("{}\nlicensed under the MIT license", menu.get_formatted_version(true));
                sep();
            }
            _ => {
                clear();
                error(format!("no option made for input {}", o));
                sep();
            }
        }
    }
    Ok(())
}