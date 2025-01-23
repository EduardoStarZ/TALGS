use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike};

pub trait FromStr {
    fn from_str(str : &str) -> NaiveDateTime;
}

pub trait FromNDT {
    fn from_naive_datetime(ndv : &NaiveDateTime) -> String;
}

impl FromStr for NaiveDateTime {
    fn from_str(str : &str) -> NaiveDateTime {
        
        let ymd_hms : Vec<&str> = str.split("T").collect::<Vec<&str>>();
    
        let ymd : Vec<i32> = ymd_hms[0].split("-").map(|char| char.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        let hms : Vec<i32> = ymd_hms[1].split(":").map(|char| char.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        return NaiveDate::from_ymd_opt(ymd[0], ymd[1] as u32, ymd[2] as u32).unwrap().and_hms_opt(hms[0] as u32, hms[1] as u32, 0).unwrap();
    }
}

impl FromNDT for String {
    fn from_naive_datetime(ndt : &NaiveDateTime) -> String {
        return format!("{}-{}-{}T{}:{}", ndt.year(), ndt.month(), ndt.day(), ndt.hour(), ndt.minute());
    }
}

// 2024-05-23:14-05-23
