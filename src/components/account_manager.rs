use egui::{Button, Context, Grid, RichText, Ui, Window};

use crate::data::{Account, AppData, Balance};
use crate::widgets::CurrencyPicker;

struct AccountEditor {
    id: Option<u32>,
    name: String,
    currency_id: Option<u32>,
    debit_account: bool,
    balances: Vec<Balance>,
    autofocus: bool,
}

impl Default for AccountEditor {
    fn default() -> Self {
        Self {
            id: None,
            name: Default::default(),
            currency_id: None,
            debit_account: false,
            balances: Vec::new(),
            autofocus: true,
        }
    }
}

impl AccountEditor {
    fn of_account(account: &Account) -> Self {
        Self {
            id: Some(account.id),
            name: account.name.clone(),
            currency_id: Some(account.currency_id),
            debit_account: account.debit_account,
            balances: account.balances.clone(),
            autofocus: true,
        }
    }
}

#[derive(Default)]
pub struct AccountManager {
    account_editor: Option<AccountEditor>,
}

impl AccountManager {
    fn credit_or_debit(debit_account: bool) -> &'static str {
        if debit_account {
            "Debit"
        } else {
            "Credit"
        }
    }
    pub fn add(&mut self, ui: &mut Ui, ctx: &Context, app_data: &mut AppData) {
        Grid::new("account-manager-grid")
            .num_columns(4)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label(RichText::new("Account").strong());
                ui.label(RichText::new("Currency").strong());
                ui.label(RichText::new("Credit/Debit").strong());
                ui.label(RichText::new("Edit").strong());
                ui.end_row();
                for account in app_data.accounts().values() {
                    ui.label(&account.name);
                    ui.label(
                        &app_data
                            .currencies()
                            .get(&account.currency_id)
                            .unwrap()
                            .code,
                    );
                    ui.label(Self::credit_or_debit(account.debit_account));
                    if ui.button("Edit").clicked() {
                        if self.account_editor.is_none() {
                            self.account_editor = Some(AccountEditor::of_account(account));
                        }
                    }
                    ui.end_row();
                }
            });

        if ui.button("New Account").clicked() {
            self.account_editor = Some(Default::default());
        }

        let mut is_open = true;
        let mut clicked_create = false;
        if let Some(account_editor) = &mut self.account_editor {
            let (title, button_text) = if account_editor.id.is_some() {
                ("Edit Account", "Save")
            } else {
                ("New Account", "Create")
            };
            Window::new(title)
                .open(&mut is_open)
                .collapsible(false)
                .show(ctx, |ui| {
                    Grid::new("account-editor-grid")
                        .num_columns(2)
                        .spacing([40.0, 4.0])
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label("Name");
                            let resp = ui.text_edit_singleline(&mut account_editor.name);
                            if account_editor.autofocus {
                                account_editor.autofocus = false;
                                ui.memory_mut(|m| m.request_focus(resp.id));
                            }
                            ui.end_row();

                            ui.label("Currency");
                            ui.add(CurrencyPicker::new(
                                "account-editor-currency-picker",
                                &mut account_editor.currency_id,
                                false,
                                app_data,
                            ));
                            ui.end_row();

                            ui.label("Debit account");
                            let credit_or_debit =
                                Self::credit_or_debit(account_editor.debit_account);
                            ui.toggle_value(&mut account_editor.debit_account, credit_or_debit);
                            ui.end_row();
                        });
                    let is_ok =
                        account_editor.name.len() > 0 && account_editor.currency_id.is_some();
                    if ui.add_enabled(is_ok, Button::new(button_text)).clicked() {
                        clicked_create = true;
                    }
                });
        }

        if clicked_create {
            let AccountEditor {
                id,
                name,
                currency_id,
                debit_account,
                balances,
                autofocus: _,
            } = self.account_editor.take().unwrap();
            let id = match id {
                Some(id) => id,
                None => app_data
                    .accounts()
                    .last_key_value()
                    .map_or(0, |(k, _)| *k + 1),
            };
            app_data.accounts_mut(|accounts| {
                accounts.insert(
                    id,
                    Account {
                        id,
                        name,
                        currency_id: currency_id.unwrap(),
                        debit_account,
                        balances,
                    },
                )
            });
        }

        if !is_open || clicked_create {
            self.account_editor = None;
        }
    }
}
