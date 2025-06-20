use libmenu_rs::{Menu, util::*};
use std::io::{self, Write};

fn main() -> Result<(), String> {
    let mut o: i32 = 0;
    let menu = Menu::new("libdraw-rs 0.1.0 test", "1.0", vec!["print something", "print hello world", "info"], None);
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
                println!("hello world");
                sep();
            },
            _ => {
                clear();
                error(format!("no option made for input {}", o));
                sep();
            }
        }
    }
    Ok(())
}