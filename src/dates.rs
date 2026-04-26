//! This is a library for interactiving with dates
//! # Example: 
//! ```
//! use cli_utils::dates::Date;
//! let date = match Date::new("01", "02", "1900") {
//!                 Ok(d) => d,
//!                 Err(e) => panic!("Error constructing date: {}", e),
//!             };
//! println!("{}", &date);
//! ```
use std::fmt;

/// Valid date formats are:
/// * "US" (MM-DD-YYYY)
/// * "UK" (DD-MM-YYYY)
/// * "ISO" (YYYY-MM-DD)
#[derive(Debug)]
pub enum DateFormat {
    US,
    UK,
    ISO,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
            DateError::MonthWrongFormat(s) => write!(f, "MonthWrongFormat: {}", s),
            DateError::DayOutOfRange(s) => write!(f, "DayOutOfRange: {}", s),
            DateError::DayWrongFormat(s) => write!(f, "DayWrongFormat: {}", s),
            DateError::YearOutofRange(s) => write!(f, "YearOutofRange: {}", s),
            DateError::YearWrongFormat(s) => write!(f, "YearWrongFormat: {}", s),
            DateError::InvalidFormat(s) => write!(f, "InvalidFormat: {}", s),
        }
    }
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Month::January => write!(f, "January"),
            Month::February => write!(f, "February"),
            Month::March => write!(f, "March"),
            Month::April => write!(f, "April"),
            Month::May => write!(f, "May"),
            Month::June => write!(f, "June"),
            Month::July => write!(f, "July"),
            Month::August => write!(f, "August"),
            Month::September => write!(f, "September"),
            Month::October => write!(f, "October"),
            Month::November => write!(f, "November"),
            Month::December => write!(f, "December"),
        }
    }
}

#[derive(Debug)]
pub struct Date {
    day: u8,
    month_alpha: Option<Month>,
    month_num: Option<String>,
    year: u32,
    leap_year: bool,
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Date | day: {}, month (#): {}, month (name): {}, year: {}, leap_year: {}", 
            self.day, 
            self.month_num.clone().unwrap(), 
            self.month_alpha.unwrap(), 
            self.year, 
            self.leap_year)
    }
}

// impl PartialEq for Date {
//     fn eq(&self, other: &Self) -> bool {
//         let mut result = true;
//         if self.day != other.day ||
//            self.month_alpha != other.month_alpha ||
//            self.month_num != other.month_num ||
//            self.year != other.year ||
//            self.leap_year != other.leap_year {
//                 result = false
//         }
        
//         result
//     }
// }

// impl Eq for Date {}

impl Date {
    pub fn format(date: &Date, format: &str) -> Result<String, DateError> {
        let fmt = match Date::validate_format(format) {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        let mut day = String::new();
        if date.day < 10 {
            day = format!("0{}", date.day);
        } else {
            day = date.day.to_string();
        }

        let month = date.month_num.clone().unwrap();
        let year = date.year;
        
        let d = match fmt {
            DateFormat::US => format!("{month}-{day}-{year}"),
            DateFormat::UK => format!("{day}-{month}-{year}"),
            DateFormat::ISO => format!("{year}-{month}-{day}"),
        };
        
        Ok(d)
    }
    /// The fields of the `Date` struct are private, so you have to use the `Date::new()` constructor
    /// along with the values for `day`, `month`, and `year`. This will return a Result, and internal 
    /// errors should bubble-up to the response.
    /// # Example:
    /// ```
    /// use cli_utils::dates::Date;
    /// let date = match Date::new("01", "02", "1972") {
    ///              Ok(d) => d,
    ///              Err(e) => return eprintln!("{}", e),
    /// };
    /// println!("{}", &date);
    /// ```
    /// In the example above, "01" is the day, "02" is the month and "1972" is the year. The following
    /// is the output:
    ///
    /// ```shell
    /// Date | day: 1, month (#), 02, month (name): February, year: 1972, leap_year: true
    /// ```
    pub fn new(day: &str, month: &str, year: &str) -> Result<Self, DateError> {
        let day_tmp = match Date::validate_day(day) {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        let month_tmp = match Date::validate_month(month) {
            Ok((num, alpha)) => (num, alpha),
            Err(e) => return Err(e),
        };

        let year_tmp = match Date::validte_year(year) {
            Ok((y, l)) => (y, l), 
            Err(e) => return Err(e),
        };

        let date = Self {
            day: day_tmp,
            month_num: month_tmp.0,
            month_alpha: month_tmp.1,
            year: year_tmp.0,
            leap_year: year_tmp.1,
        };

        Ok(date)
    }

    fn validate_day(day: &str) -> Result<u8, DateError> {
        let day_int:u8 = match day.parse() {
            Ok(num) => num,
            Err(e) => return Err(DateError::DayWrongFormat(e.to_string())),
        };

        if day_int < 1 || day_int > 31 {
            return Err(DateError::DayOutOfRange("Day must be between 1 and 31".to_string()))
        };

        Ok(day_int)

    }

    fn validate_month(m: &str) -> Result<(Option<String>, Option<Month>), DateError> {
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

    fn validte_year(y: &str) -> Result<(u32, bool), DateError> {
        let year_tmp: u32 = match y.parse() {
            Ok(num) => num,
            Err(e) => return Err(DateError::YearWrongFormat(e.to_string())),
        };

        if year_tmp < 1582 || year_tmp > 9_999 {
            return Err(DateError::YearOutofRange("Year must be between 1582 and 9999.".to_string()));
        };

        let leap_year = Date::leap_year(&year_tmp);

        Ok((year_tmp, leap_year))
    }

    /// You can send a year from `1582` to `9999` to see if it is a leap year. This is independent of 
    /// a `Date` object. To see if the year from a `Date` object is a leap year, use the `get_leap_year` 
    /// method.
    /// This returns a `bool`.
    /// # Example:
    /// ```
    /// use cli_utils::dates::Date;
    /// let year = 1972;
    /// println!("Is the year {} a leap year?: {}", year, Date::leap_year(&year));
    /// ```
    pub fn leap_year(y: &u32) -> bool {
        let mut leap_year = false;
        if y % 4 == 0 {
            leap_year = true;
            if y % 100 == 0 && y % 400 != 0 {
                leap_year = false;
            }
        }

        leap_year
    }

    fn validate_format(f: &str) -> Result<DateFormat, DateError> {
        let date_format = match f {
            "US" => DateFormat::US,
            "UK" => DateFormat::UK,
            "ISO" => DateFormat::ISO,
            _ => return Err(DateError::InvalidFormat("Supported format values are US, UK and ISO.".to_string())),
        };

        Ok(date_format)
    }

    /// As the fields for the `Date` struct are private, you need to use this method to fetch the `year` 
    /// field from a `Date` object.
    /// # Example:
    /// ```
    /// use cli_utils::dates::Date;
    /// let date = match Date::new("01", "02", "1972") {
    ///             Ok(d) => d,
    ///             Err(e) => return eprintln!("{}", e)
    /// };
    /// println!("Year: {}", Date::get_year(&date));
    /// ```
    pub fn get_year(&self) -> u32 {
        self.year
    }

    /// As the fields for the `Date` struct are private, you need to use this method to fetch the `month_num` 
    /// field from a `Date` object. This field is the month represented as a two-digit number.
    /// # Example:
    /// ```
    /// use cli_utils::dates::Date;
    /// let date = match Date::new("01", "02", "1972") {
    ///             Ok(d) => d,
    ///             Err(e) => return eprintln!("{}", e)
    /// };
    /// println!("Month (#): {}", Date::get_month_n(&date));
    /// ```
    pub fn get_month_n(&self) -> String {
        let month: String = self.month_num.clone().unwrap();
        month
    }

    /// As the fields for the `Date` struct are private, you need to use this method to fetch the 
    /// `month_alpha` field from a `Date` object. This field is the month represented as a name, 
    /// rather than a number.
    /// # Example:
    /// ```
    /// use cli_utils::dates::Date;
    /// let date = match Date::new("01", "02", "1972") {
    ///             Ok(d) => d,
    ///             Err(e) => return eprintln!("{}", e)
    /// };
    /// println!("Month (name): {}", Date::get_month_a(&date));
    /// ```
    pub fn get_month_a(&self) -> String {
        let month: String = match &self.month_alpha {
            Some(m) => m.to_string(),
            None => panic!("Something went wrong!"),
        };
        month
    }

    /// As the fields for the `Date` struct are private, you need to use this method to fetch the 
    /// `day` field from a `Date` object. This field is the day represented as a two-digit 
    /// number.
    /// # Example:
    /// ```
    /// use cli_utils::dates::Date;
    /// let date = match Date::new("01", "02", "1972") {
    ///             Ok(d) => d,
    ///             Err(e) => return eprintln!("{}", e)
    /// };
    /// println!("Day: {}", Date::get_day(&date));
    /// ```
    pub fn get_day(&self) -> String {
        let mut day: String = String::new();
        if self.day < 10 {
            day = format!("0{}", self.day.to_string());
        } else {
            day = self.day.to_string();
        };

        day
    }

    /// As the fields for the `Date` struct are private, you need to use this method to fetch the 
    /// `leap_year` field from a `Date` object. This field shows whether the year defined in this 
    /// `Date` object is a leap_year or not. Returns a bool.
    /// # Example:
    /// ```
    /// use cli_utils::dates::Date;
    /// let date = match Date::new("01", "02", "1972") {
    ///             Ok(d) => d,
    ///             Err(e) => return eprintln!("{}", e)
    /// };
    /// println!("Month (name): {}", Date::get_month_a(&date));
    /// ```
    pub fn get_leap_year(&self) -> bool {
        self.leap_year
    }

}