use super::ValidatedTextEdit;
use crate::data::{Currency, NumericPrice, Price};

pub fn price_input<'a>(value: &'a mut i32, currency: &'a Currency) -> ValidatedTextEdit<'a, i32> {
    ValidatedTextEdit::new(value)
        .display_formatter(|value: &i32| format!("{}", Price::new(*value, currency)))
        .edit_formatter(|value: &i32| format!("{}", NumericPrice::new(*value, currency)))
        .parser(|s: &str, _t: &i32| match s.parse::<f32>().ok() {
            Some(raw_price) => Some((raw_price * (currency.major as f32)).round() as i32),
            None => None,
        })
}
