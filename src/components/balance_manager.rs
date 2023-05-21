use chrono::naive::NaiveDate as Date;
use egui::{Color32, Grid, RichText, Ui};

use crate::data::{AppData, Balance, CachedValue, Price};

pub struct BalanceManager {
    account_id: u32,
    balance_table: CachedValue<Vec<(Balance, i32)>>,
}

impl BalanceManager {
    pub fn new(account_id: u32) -> Self {
        Self {
            account_id,
            balance_table: Default::default(),
        }
    }

    pub fn add(&mut self, ui: &mut Ui, app_data: &mut AppData) {
        let balance_table = self.balance_table.get(app_data, |app_data: &AppData| {
            let account = match app_data.accounts().get(&self.account_id) {
                Some(account) => account,
                None => return Vec::new(),
            };
            let mut balance_table = Vec::new();
            for balance in &account.balances {
                balance_table.push((balance.clone(), 0));
            }
            balance_table.push((
                Balance {
                    date: Date::MAX,
                    amount: 0,
                },
                0,
            ));
            let mut i = 0;
            for transactions_on_date in app_data.transactions_by_date().values() {
                for transaction_id in transactions_on_date {
                    let transaction = app_data.transactions().get(&transaction_id).unwrap();
                    if transaction.account_id == account.id {
                        if transaction.date > balance_table[i].0.date {
                            i += 1;
                        }
                        balance_table[i].1 += transaction.amount;
                    }
                }
            }
            balance_table
        });

        Grid::new(format!("balance-manager-grid-{}", self.account_id))
            .num_columns(5)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.strong("Date");
                ui.strong("Balance");
                ui.strong("Transaction Total");
                ui.end_row();

                let account = app_data.accounts().get(&self.account_id).unwrap();
                let currency = app_data.currencies().get(&account.currency_id).unwrap();
                let mut last_balance_amount = 0;
                for (balance, delta) in balance_table.iter() {
                    if balance.date < Date::MAX {
                        ui.label(&balance.date.to_string());
                        ui.label(format!("{}", Price::new(balance.amount, currency)));
                        let expected_delta = balance.amount - last_balance_amount;
                        let color = if *delta == expected_delta {
                            Color32::GREEN
                        } else {
                            Color32::RED
                        };
                        last_balance_amount = balance.amount;
                        ui.label(
                            RichText::new(format!("{}", Price::new(*delta, currency))).color(color),
                        );
                    } else {
                        ui.label("Latest");
                        ui.label(format!(
                            "{}",
                            Price::new(last_balance_amount + delta, currency)
                        ));
                        ui.label(format!("{}", Price::new(*delta, currency)));
                    }
                    ui.end_row();
                }
            });
    }
}
