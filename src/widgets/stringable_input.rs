use std::str::FromStr;
use std::string::ToString;

use super::ValidatedTextEdit;

pub fn stringable_input<'a, T: FromStr + ToString + PartialEq + Clone>(
    value: &'a mut T,
) -> ValidatedTextEdit<'a, T> {
    ValidatedTextEdit::new(value)
        .display_formatter(|t: &T| t.to_string())
        .parser(|s: &str, _t: &T| s.parse().ok())
}
