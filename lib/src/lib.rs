//! libmenu library for rust
use std::io::{self, Write};

/// contains information about a menu
pub struct Menu {
    /// menu name
    name: String,
    /// menu version
    version: String,
    /// menu options
    options: Vec<String>,
    /// menu exit text (optional)
    exit_text: String
}

/// contains information about a selection menu
pub struct SelMenu {
    /// menu name
    action: String,
    /// menu options
    options: Vec<String>,
    /// menu cancel text (optional)
    cancel_text: String
}

/// utilities
pub mod util {
    use super::*;
    /// clear the screen for the next operation
    /// 
    /// if flushing fails, nothing will happen until the next newline (i think)
    pub fn clear() -> () {
        print!("\x1b[2J\x1b[H");
        io::stdout().flush().ok();
    }
    /// print an error incase of input invalidation or something else
    /// 
    /// there's no specific `inputErr` function because you can just
    /// make your own using `format!`
    pub fn error(info: String) -> () {
        println!("\x1b[1;31merror:\x1b[0m\x1b[1m {}\x1b[0m", info);
    }
    /// print a warning incase something goes wrong and an error doesn't fit
    pub fn warning(info: String) -> () {
        println!("\x1b[1;33merror:\x1b[0m\x1b[1m {}\x1b[0m", info);
    }
    /// seperate the output by exactly 75 equal signs
    pub fn sep() -> () {
        println!("{}", "=".repeat(75));
    }
}

impl Menu {
    /// makes a new menu
    pub fn new(name: &str, version: &str, options: Vec<&str>, exit_text: Option<&str>) -> Self {
        let new_o: Vec<String> = options.iter().map(|o| o.to_string()).collect();
        Self {
            name: name.to_string(),
            version: version.to_string(),
            options: new_o,
            exit_text: exit_text.unwrap_or("exit").to_string()
        }
    }
    
    /// get a formatted string of the name and version of the menu
    ///
    /// `bool include_version`: defines if you want to include `Menu.version`,
    /// useful for submenus
    pub fn get_formatted_version(&self, include_version: bool) -> String {
        if include_version {
            format!("{} v. {}", self.name, self.version)
        } else {
            format!("{}", self.name)
        }
    }
    /// print a menu, get user input and put it
    /// on `option_var`
    /// 
    /// `int print_name`: if true, print the name and version of the menu defined
    /// in `Menu.name` and `Menu.version` respectively
    ///
    /// else, don't print them
    ///
    /// `int include_version`: defines if you want to include `Menu.version`
    /// on the menu name header, useful for submenus that have set their version
    /// to an empty string
    pub fn get_input(&self, option_var: &mut i32, print_name: bool, include_version: bool) -> Result<(), String> {
        let mut cuh = String::new();
        if print_name {
            println!("{}", self.get_formatted_version(include_version));
        }
        for (i, o) in self.options.iter().enumerate() {
            println!("({}) {}", i+1, o);
        }
        println!("(0) {}\n", self.exit_text);
        print!("(?) >> ");
        // dfhdszresd
        // HE.P
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut cuh).unwrap();
        *option_var = cuh.trim().parse().unwrap_or(0);
        Ok(())
    }
}

impl SelMenu {
    /// makes a new menu
    pub fn new(action: &str, options: Vec<&str>, cancel_text: Option<&str>) -> Self {
        let new_o: Vec<String> = options.iter().map(|o| o.to_string()).collect();
        Self {
            action: action.to_string(),
            options: new_o,
            cancel_text: cancel_text.unwrap_or("cancel").to_string()
        }
    }
    
    /// print a selection menu, get user input and put it
    /// on `option_var`
    /// 
    /// `int print_action`: if 1, print the name of the menu defined
    /// in `SelMenu.action`
    ///
    /// else, don't print it
    ///
    /// `int include_cancel`: defines if you want to include `SelMenu.cancelText`
    /// on the output
    pub fn get_input(&self, option_var: &mut i32, print_action: bool, include_cancel: bool) -> Result<(), String> {
        let mut cuh = String::new();
        if print_action {
            println!("{}", self.action);
        }
        for (i, o) in self.options.iter().enumerate() {
            println!("({}) {}", i+1, o);
        }
        if include_cancel {
            println!("(0) {}\n", self.cancel_text);
        }
        print!("(?) >> ");
        // dfhdszresd
        // HE.P
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut cuh).unwrap();
        *option_var = cuh.trim().parse().unwrap_or(0);
        Ok(())
    }
}