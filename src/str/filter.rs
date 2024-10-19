use regex::Regex;

pub struct Form<'a> {
    pub name: &'a str,
    pub value: &'a str
}

pub trait FromStr<'a> {
    fn from_str(vec : &'a str) -> Form<'a>;
}

impl<'a> FromStr<'a> for Form<'a> {
    fn from_str(string : &'a str) -> Form<'a> {
        let splitter : Vec<&str> = string.split("=").collect::<Vec<&str>>();
        return Form{name: splitter.get(0).unwrap(), value: splitter.get(1).unwrap()};
    }
}

pub fn payload_into_values<'a>(value : &'a String) -> Vec<Form> {
    let splitted : Vec<&str> = value.split("&").collect();

    let checker : Regex = Regex::new(r".+\=.+").unwrap();

    if checker.is_match(value) {
        let mut last : Vec<Form> = Vec::new();

        for x in splitted {
            last.push(Form::from_str(x))
        }
        return last;
    }

    return vec![];
}
