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
