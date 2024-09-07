/*
 *
 * color.rs
 *
 * Copyright (c) 2023-2024 EduardoStarZ, NandoBFK, Erenan257
 *
 * All rights reserved
 *
 * TALGS is distributed under the GNU General Public license v2, see LICENSE for details
 * 
 * */


use colored::{ColoredString, Colorize};

pub trait Color {
    fn request(&self) -> ColoredString;
    fn warning(&self) -> ColoredString;
    fn database_values(&self) -> ColoredString;
}

impl Color for String {
    fn request(&self) -> ColoredString {
       return self.bright_green(); 
    }

    fn warning(&self) -> ColoredString {
        return self.bright_red();
    }

    fn database_values(&self) -> ColoredString {
        return self.bright_blue();
    }
}

impl Color for str {
    fn request(&self) -> ColoredString {
       return self.bright_green(); 
    }

    fn warning(&self) -> ColoredString {
        return self.bright_red();
    }

    fn database_values(&self) -> ColoredString {
        return self.bright_blue();
    }
}
