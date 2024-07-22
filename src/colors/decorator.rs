/*
 *
 * decorator.rs
 *
 * Copyright (c) 2023-2024 EduardoStarZ, NandoBFK, Erenan257
 *
 * All rights reserved
 *
 * TALGS is distributed under the GNU General Public license v2, see LICENSE for details
 * 
 * */


use colored::{ColoredString, Colorize};

pub trait Decore {
    fn to_italic(&self) -> ColoredString;
    fn to_underline(&self) -> ColoredString;
    fn to_strong(&self) -> ColoredString;
}

impl Decore for String {
    fn to_italic(&self) -> ColoredString {
        return self.italic();
    }

    fn to_underline(&self) -> ColoredString {
        return self.underline();
    }

    fn to_strong(&self) -> ColoredString {
        return self.bold();
    }
}

impl Decore for str {
    fn to_italic(&self) -> ColoredString {
        return self.italic();
    }

    fn to_underline(&self) -> ColoredString {
        return self.underline();
    }

    fn to_strong(&self) -> ColoredString {
        return self.bold();
    }
}
