use chrono::{NaiveDate, NaiveDateTime};

pub trait FromStr {
    fn from_str(str : &str) -> NaiveDateTime;
}

impl FromStr for NaiveDateTime {
    fn from_str(str : &str) -> NaiveDateTime {
        
        let ymd_hms : Vec<&str> = str.split(":").collect::<Vec<&str>>();
    
        let ymd : Vec<i32> = ymd_hms[0].split("-").map(|char| char.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        let hms : Vec<i32> = ymd_hms[1].split("-").map(|char| char.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        return NaiveDate::from_ymd_opt(ymd[0], ymd[1] as u32, ymd[2] as u32).unwrap().and_hms_opt(hms[0] as u32, hms[1] as u32, hms[2] as u32).unwrap();
    }
}

// 2024-05-23:14-05-23
