//! This is a library for interactiving with dates
//! # Example: 
//! ```
//! use cli_utils::dates;
//! ```

enum DateFormats {
    US,
    UK,
    ISO,
}

enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

pub struct Date {
    day: u8,
    month: Month,
    year: u32,
    format: DateFormats,
}

impl Date {
    pub fn format_us_numberical(day: &str, month: &str, year: &str) -> String {
        let date = format!("{month}-{day}-{year}");
        date
    }

    pub fn new(day: String, month: String, year: String) -> Self {
        if (day.len() as i32) != 2 {
            println!("Day must be in two digit format: '01' to '31'");
            std::process::exit(1);
        }
        match day {
            "01" => 

        }
    }

    fn set_day
}