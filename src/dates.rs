//! This is a library for interactiving with dates
//! # Example: 
//! ```
//! use cli_utils::dates;
//! let date = Date::new(day: 01, month: 02, year: 1970, format: US);
//! ```
use std::fmt;

#[derive(Debug)]
pub enum DateFormat {
    US,
    UK,
    ISO,
}

#[derive(Debug)]
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

pub enum DateError {
    MonthOutOfRange(String),
    MonthWrongFormat(String),
    DayOutOfRange(String),
    DayWrongFormat(String),
    YearOutofRange(String),
    YearWrongFormat(String),
    InvalidFormat(String),
}

impl fmt::Display for DateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DateError::MonthOutOfRange(s) => write!(f, "MonthOutOfRange: {}", s),
            DateError::MonthWrongFormat(s) => write!(f, "MonthWroteFormat: {}", s),
            DateError::DayOutOfRange(s) => write!(f, "DayOutOfRange: {}", s),
            DateError::DayWrongFormat(s) => write!(f, "DayWrongFormat: {}", s),
            DateError::YearOutofRange(s) => write!(f, "YearOutofRange: {}", s),
            DateError::YearWrongFormat(s) => write!(f, "YearWrongFormat: {}", s),
            DateError::InvalidFormat(s) => write!(f, "InvalidFormat: {}", s),
        }
    }
}

#[derive(Debug)]
pub struct Date {
    day: u8,
    month_alpha: Option<Month>,
    month_num: Option<String>,
    year: u32,
    format: DateFormat,
}

impl Date {
    pub fn format_us_numberical(date: Date) -> String {
        let mut day = String::new();
        if date.day < 10 {
            day = format!("0{}", date.day);
        } else {
            day = date.day.to_string();
        }

        let month = date.month_num.unwrap();
        let year = date.year;
        let d = format!("{month}-{day}-{year}");
        
        d
    }

    pub fn new(day: String, month: String, year: String, format: String) -> Result<Self, DateError> {
        let day_tmp = match Date::validate_day(&day) {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        let month_tmp = match Date::validate_month(&month) {
            Ok((num, alpha)) => (num, alpha),
            Err(e) => return Err(e),
        };

        let year_tmp = match Date::validte_year(&year) {
            Ok(y) => y,
            Err(e) => return Err(e),
        };

        let format_tmp = match Date::validate_format(&format) {
            Ok(f) => f,
            Err(e) => return Err(DateError::InvalidFormat("Date format must be US, UK or ISO.".to_string())),
        };

        let date = Self {
            day: day_tmp,
            month_num: month_tmp.0,
            month_alpha: month_tmp.1,
            year: year_tmp,
            format: format_tmp,
        };

        Ok(date)
    }

    // fn set_day(day: &String) {

    // }

    fn validate_day(day: &String) -> Result<u8, DateError> {
        let day_int:u8 = match day.parse() {
            Ok(num) => num,
            Err(e) => return Err(DateError::DayWrongFormat(e.to_string())),
        };

        if day_int < 1 || day_int > 31 {
            return Err(DateError::DayOutOfRange("Day must be between 1 and 31".to_string()))
        };

        Ok(day_int)

    }

    fn validate_month(m: &String) -> Result<(Option<String>, Option<Month>), DateError> {
        let month_int:u8 = match m.parse() {
            Ok(num) => num,
            Err(e) => return Err(DateError::MonthWrongFormat(e.to_string())),
        };

        match month_int {
            1 => return Ok((Some("01".to_string()), Some(Month::January))),
            2 => return Ok((Some("02".to_string()), Some(Month::February))),
            3 => return Ok((Some("03".to_string()), Some(Month::March))),
            4 => return Ok((Some("04".to_string()), Some(Month::April))),
            5 => return Ok((Some("05".to_string()), Some(Month::May))),
            6 => return Ok((Some("06".to_string()), Some(Month::June))),
            7 => return Ok((Some("07".to_string()), Some(Month::July))),
            8 => return Ok((Some("08".to_string()), Some(Month::August))),
            9 => return Ok((Some("09".to_string()), Some(Month::September))),
            10 => return Ok((Some("10".to_string()), Some(Month::October))),
            11 => return Ok((Some("11".to_string()), Some(Month::November))),
            12 => return Ok((Some("12".to_string()), Some(Month::December))),
            _ => return Err(DateError::MonthOutOfRange("Month out of range: 1 .. 12".to_string())),             
        };

    }

    fn validte_year(y: &String) -> Result<u32, DateError> {
        let year_tmp: u32 = match y.parse() {
            Ok(num) => num,
            Err(e) => return Err(DateError::YearWrongFormat(e.to_string())),
        };

        if year_tmp < 1582 || year_tmp > 9_999 {
            return Err(DateError::YearOutofRange("Year must be between 1582 and 9999.".to_string()));
        };

        Ok(year_tmp)
    }

    fn validate_format(f: &String) -> Result<DateFormat, DateError> {
        let date_format = match f.as_str() {
            "US" => DateFormat::US,
            "UK" => DateFormat::UK,
            "ISO" => DateFormat::ISO,
            _ => return Err(DateError::InvalidFormat("Supported format values are US, UK and ISO.".to_string())),
        };

        Ok(date_format)
    }
}