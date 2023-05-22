use chrono::naive::NaiveDate as Date;
use egui::{Button, Color32, Context, DragValue, Grid, RichText, Window, Ui};
use egui_extras::DatePickerButton;

use crate::data::{AppData, Balance, CachedValue, Price, Update};

struct BalanceEditor {
    new_balance: bool,
    date: Date,
    amount: Option<i32>,
    computed_amount: CachedValue<i32>,
}

impl Default for BalanceEditor {
    fn default() -> Self {
        Self {
            new_balance: true,
            date: chrono::offset::Local::now().date_naive(),
            amount: None,
            computed_amount: Default::default(),
        }
    }
}

impl BalanceEditor {
    fn of_balance(balance: &Balance) -> Self {
        Self {
            new_balance: false,
            date: balance.date,
            amount: Some(balance.amount),
            computed_amount: Default::default(),
        }
    }
}

pub struct BalanceManager {
    account_id: u32,
    balance_table: CachedValue<Vec<(Balance, i32)>>,
    balance_editor: Option<BalanceEditor>,
}

impl BalanceManager {
    pub fn new(account_id: u32) -> Self {
        Self {
            account_id,
            balance_table: Default::default(),
            balance_editor: None,
        }
    }

    pub fn add(&mut self, ui: &mut Ui, ctx: &Context, app_data: &mut AppData) {
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

        let account = app_data.accounts().get(&self.account_id).unwrap();
        let currency = app_data.currencies().get(&account.currency_id).unwrap();
        let mut delete_index: Option<usize> = None;

        Grid::new(format!("balance-manager-grid-{}", self.account_id))
            .num_columns(5)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.strong("Date");
                ui.strong("Balance");
                ui.strong("Transaction Total");
                ui.strong("Edit");
                ui.strong("Delete");
                ui.end_row();

                let mut last_balance_amount = 0;
                for (index, (balance, delta)) in balance_table.iter().enumerate() {
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
                        if ui.button("Edit").clicked() {
                            if self.balance_editor.is_none() {
                                self.balance_editor = Some(BalanceEditor::of_balance(balance));
                            }
                        }
                        if ui.button("Delete").clicked() {
                            delete_index = Some(index);
                        }
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

        if ui.add_enabled(self.balance_editor.is_none(), Button::new("New Balance")).clicked() {
            self.balance_editor = Some(Default::default());
        }

        let mut is_open = true;
        let mut clicked_create = false;
        if let Some(balance_editor) = &mut self.balance_editor {
            let (title, button_text) = if balance_editor.new_balance {
                ("New Balance", "Create")
            } else {
                ("Edit Balance", "Save")
            };
            Window::new(title)
                .open(&mut is_open)
                .collapsible(false)
                .show(ctx, |ui| {
                    Grid::new("balance-editor-grid")
                        .num_columns(2)
                        .spacing([40.0, 4.0])
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label("Date");
                            if balance_editor.new_balance {
                                let old_date = balance_editor.date.clone();
                                ui.add(DatePickerButton::new(&mut balance_editor.date));
                                if old_date != balance_editor.date {
                                    balance_editor.computed_amount.invalidate();
                                }
                            } else {
                                ui.label(&balance_editor.date.to_string());
                            }
                            ui.end_row();

                            let computed = *balance_editor.computed_amount.get(app_data, |app_data: &AppData| {
                                account.balance_on_date(app_data, balance_editor.date)
                            });

                            ui.label("Amount");
                            ui.horizontal(|ui| {
                                ui.horizontal(|ui| {
                                    ui.spacing_mut().item_spacing.x = 1.0;
                                    ui.label(&currency.symbol);
                                    let mut edit_amount = (balance_editor.amount.unwrap_or(computed) as f32) / (currency.major as f32);
                                    ui.add(DragValue::new(&mut edit_amount).min_decimals(2).max_decimals(2));
                                    let edit_amount: i32 = (edit_amount * (currency.major as f32)).round() as i32;
                                    balance_editor.amount =
                                        if edit_amount != computed {
                                            Some(edit_amount)
                                        } else {
                                            None
                                        };
                                    ui.label(&currency.code);
                                });
                                if balance_editor.amount.is_some() {
                                    if ui.button("⟲").clicked() {
                                        balance_editor.amount = None;
                                    }
                                }
                            });
                            ui.end_row();

                            ui.label("Computed balance on date");
                            ui.label(format!("{}", Price::new(computed, currency)));
                            ui.end_row();
                        });

                    let is_ok = !balance_editor.new_balance || account.balances.binary_search_by(|b| b.date.cmp(&balance_editor.date)).is_err();
                    if ui.add_enabled(is_ok, Button::new(button_text)).clicked() {
                        clicked_create = true;
                    }
                    if !is_ok {
                        ui.label(RichText::new("Balance already exists for that date").color(Color32::RED));
                    }
                });
        }

        if clicked_create {
            let mut account = account.clone();
            let BalanceEditor {
                new_balance,
                date,
                amount,
                computed_amount: _
            } = self.balance_editor.take().unwrap();
            let amount = match amount {
                Some(amount) => amount,
                None => account.balance_on_date(app_data, date),
            };
            let balance = Balance {
                date,
                amount,
            };
            if new_balance {
                let index = account.balances.partition_point(|b| b.date < date);
                account.balances.insert(index, balance);
            } else {
                let index = account.balances.binary_search_by(|b| b.date.cmp(&date)).unwrap();
                account.balances[index] = balance;
            }
            app_data.perform_update(vec![Update::SetAccount(account)]);
        } else if let Some(delete_index) = delete_index {
            let mut account = account.clone();
            account.balances.remove(delete_index);
            app_data.perform_update(vec![Update::SetAccount(account)]);
        }

        if !is_open || clicked_create {
            self.balance_editor = None;
        }
    }
}
