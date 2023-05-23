use super::Currency;

pub struct Price<'a> {
    pub amount: i32,
    pub currency: &'a Currency,
}

impl<'a> Price<'a> {
    pub fn new(amount: i32, currency: &'a Currency) -> Self {
        Self { amount, currency }
    }

    pub fn scaled_amount(&self) -> f32 {
        (self.amount as f32) / (self.currency.major as f32)
    }
}

impl<'a> std::fmt::Display for Price<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}{}{:.2} {}",
            if self.amount < 0 { "-" } else { "" },
            self.currency.symbol,
            self.scaled_amount().abs(),
            self.currency.code
        )
    }
}

pub struct NumericPrice<'a> {
    pub amount: i32,
    pub currency: &'a Currency,
}

impl<'a> NumericPrice<'a> {
    pub fn new(amount: i32, currency: &'a Currency) -> Self {
        Self { amount, currency }
    }

    pub fn scaled_amount(&self) -> f32 {
        (self.amount as f32) / (self.currency.major as f32)
    }
}

impl<'a> std::fmt::Display for NumericPrice<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}{:.2}",
            if self.amount < 0 { "-" } else { "" },
            self.scaled_amount().abs(),
        )
    }
}
