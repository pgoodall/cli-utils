use cli_utils::dates::{Date};

#[test]
fn test_leap_year_true() {
    let y = 1584;
    assert_eq!(Date::leap_year(&y), true)
}

#[test]
fn test_leap_year_false() {
    let y = 1800;
    assert_eq!(Date::leap_year(&y), false)
}

#[test]
fn test_date_creation_single_digit_day() {
    let date = match Date::new("01", "02", "1972") {
        Ok(d) => d,
        Err(e) => panic!("{}", e),
    };
    let result = format!("{}", date);
    assert_eq!(result, "Date | day: 1, month (#): 02, month (name): February, year: 1972, leap_year: true")
}

#[test]
#[should_panic = "YearOutofRange"]
fn test_date_creation_year_under_range_fails() {
    let date = match Date::new("01", "02", "999") {
        Ok(d) => d,
        Err(e) => panic!("{}", e),
    };
    let result = format!("{}", date);
    assert_eq!(result, "Date | day: 1, month (#): 02, month (name): February, year: 1972, leap_year: true")
}

#[test]
#[should_panic = "YearOutofRange"]
fn test_date_creation_year_over_range_fails() {
    let date = match Date::new("01", "02", "10000") {
        Ok(d) => d,
        Err(e) => panic!("{}", e),
    };
    let result = format!("{}", date);
    assert_eq!(result, "Date | day: 1, month (#): 02, month (name): February, year: 1972, leap_year: true")
}

#[test]
#[should_panic = "YearWrongFormat"]
fn test_date_creation_alpha_year_fails() {
    let date = match Date::new("01", "02", "foo") {
        Ok(d) => d,
        Err(e) => panic!("{}", e),
    };
    let result = format!("{}", date);
    assert_eq!(result, "Date | day: 1, month (#): 02, month (name): February, year: 1972, leap_year: true")
}

#[test]
#[should_panic = "MonthWrongFormat"]
fn test_date_creation_alpha_month_fails() {
    let date = match Date::new("01", "February", "1972") {
        Ok(d) => d,
        Err(e) => panic!("{}", e),
    };
    let result = format!("{}", date);
    assert_eq!(result, "Date | day: 1, month (#): 02, month (name): February, year: 1972, leap_year: true")
}

#[test]
#[should_panic = "MonthOutOfRange"]
fn test_date_creation_month_out_of_range_fails() {
    let date = match Date::new("01", "13", "1972") {
        Ok(d) => d,
        Err(e) => panic!("{}", e),
    };
    let result = format!("{}", date);
    assert_eq!(result, "Date | day: 1, month (#): 02, month (name): February, year: 1972, leap_year: true")
}

#[test]
#[should_panic = "DayWrongFormat"]
fn test_date_creation_alpha_day_fails() {
    let date = match Date::new("one", "02", "1972") {
        Ok(d) => d,
        Err(e) => panic!("{}", e),
    };
    let result = format!("{}", date);
    assert_eq!(result, "Date | day: 1, month (#): 02, month (name): February, year: 1972, leap_year: true")
}

#[test]
#[should_panic = "DayOutOfRange"]
fn test_date_creation_day_out_of_range() {
    let date = match Date::new("32", "02", "1972") {
        Ok(d) => d,
        Err(e) => panic!("{}", e),
    };
    let result = format!("{}", date);
    assert_eq!(result, "Date | day: 1, month (#): 02, month (name): February, year: 1972, leap_year: true")
}

#[test]
fn test_get_day() {
    let date = match Date::new("01", "02", "1972") {
        Ok(d) => d,
        Err(e) => panic!("{}", e),
    };
    let day = Date::get_day(&date);
    assert_eq!(day, "01")
}

#[test]
fn test_get_month_n() {
    let date = match Date::new("01", "02", "1972") {
        Ok(d) => d,
        Err(e) => panic!("{}", e),
    };
    let month = Date::get_month_n(&date);
    assert_eq!(month, "02")
}

#[test]
fn test_get_month_a() {
    let date = match Date::new("01", "02", "1972") {
        Ok(d) => d,
        Err(e) => panic!("{}", e),
    };
    let month = Date::get_month_a(&date);
    assert_eq!(month, "February")
}

#[test]
fn test_get_year() {
    let date = match Date::new("01", "02", "1972") {
        Ok(d) => d,
        Err(e) => panic!("{}", e),
    };
    let year = Date::get_year(&date);
    assert_eq!(year, 1972)
}

#[test]
fn test_get_leap_year() {
    let date = match Date::new("01", "02", "1972") {
        Ok(d) => d,
        Err(e) => panic!("{}", e),
    };
    let leap_year = Date::get_leap_year(&date);
    assert_eq!(leap_year, true)
}