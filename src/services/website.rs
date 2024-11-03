use chrono::Datelike;

pub fn get_current_age() -> u32 {
    let now = chrono::Utc::now();
    let birth_date = chrono::NaiveDate::from_ymd_opt(2002, 2, 27);
    let age = match birth_date {
        Some(birth_date) => now.year_ce().1 - birth_date.year_ce().1,
        None => 0,
    };

    age
}
