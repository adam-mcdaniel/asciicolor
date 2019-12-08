#![no_std]

#[macro_use] extern crate alloc;

use core::fmt::Display;
use alloc::string::String;

pub trait Colorize {
    fn black(self) -> String;
    fn red(self) -> String;
    fn green(self) -> String;
    fn yellow(self) -> String;
    fn blue(self) -> String;
    fn magenta(self) -> String;
    fn cyan(self) -> String;
    fn white(self) -> String;

    fn bright_black(self) -> String;
    fn bright_red(self) -> String;
    fn bright_green(self) -> String;
    fn bright_yellow(self) -> String;
    fn bright_blue(self) -> String;
    fn bright_magenta(self) -> String;
    fn bright_cyan(self) -> String;
    fn bright_white(self) -> String;

    fn underline(self) -> String;
    fn bold(self) -> String;
    fn invert(self) -> String;
}

impl<T: Display> Colorize for T {
    fn black(self) -> String {
        format!("\x1b[30m{}\x1b[m\x1b[0m", self)
    }
    
    fn red(self) -> String {
        format!("\x1b[31m{}\x1b[m\x1b[0m", self)
    }
    
    fn green(self) -> String {
        format!("\x1b[32m{}\x1b[m\x1b[0m", self)
    }
    
    fn yellow(self) -> String {
        format!("\x1b[33m{}\x1b[m\x1b[0m", self)
    }
    
    fn blue(self) -> String {
        format!("\x1b[34m{}\x1b[m\x1b[0m", self)
    }
    
    fn magenta(self) -> String {
        format!("\x1b[35m{}\x1b[m\x1b[0m", self)
    }
    
    fn cyan(self) -> String {
        format!("\x1b[36m{}\x1b[m\x1b[0m", self)
    }
    
    fn white(self) -> String {
        format!("\x1b[37m{}\x1b[m\x1b[0m", self)
    }

    fn bright_black(self) -> String {
        format!("\x1b[30;1m{}\x1b[m\x1b[0m", self)
    }
    
    fn bright_red(self) -> String {
        format!("\x1b[31;1m{}\x1b[m\x1b[0m", self)
    }
    
    fn bright_green(self) -> String {
        format!("\x1b[32;1m{}\x1b[m\x1b[0m", self)
    }
    
    fn bright_yellow(self) -> String {
        format!("\x1b[33;1m{}\x1b[m\x1b[0m", self)
    }
    
    fn bright_blue(self) -> String {
        format!("\x1b[34;1m{}\x1b[m\x1b[0m", self)
    }
    
    fn bright_magenta(self) -> String {
        format!("\x1b[35;1m{}\x1b[m\x1b[0m", self)
    }
    
    fn bright_cyan(self) -> String {
        format!("\x1b[36;1m{}\x1b[m\x1b[0m", self)
    }
    
    fn bright_white(self) -> String {
        format!("\x1b[37;1m{}\x1b[m\x1b[0m", self)
    }

    fn bold(self) -> String {
        format!("\x1b[1m{}\x1b[m\x1b[0m", self)
    }
    
    fn underline(self) -> String {
        format!("\x1b[4m{}\x1b[m\x1b[0m", self)
    }

    fn invert(self) -> String {
        format!("\x1b[7m{}\x1b[m\x1b[0m", self)
    }
}
