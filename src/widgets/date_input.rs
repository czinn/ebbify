use chrono::naive::NaiveDate as Date;
use chrono::Datelike;

use super::ValidatedTextEdit;

pub fn date_input(value: &mut Date) -> ValidatedTextEdit<'_, Date> {
    ValidatedTextEdit::new(value)
        .display_formatter(|date: &Date| date.to_string())
        .parser(|s: &str, original_date: &Date| match s.parse().ok() {
            Some(date) => Some(date),
            None => match format!("2000-{}", s).parse::<Date>().ok() {
                Some(month_day) => month_day.with_year(original_date.year()),
                None => match s.parse::<u32>().ok() {
                    Some(day) => original_date.with_day(day),
                    None => None,
                },
            },
        })
}
