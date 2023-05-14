use egui::{Ui, Widget, Response, Label, RichText, FontSelection};
use egui_extras::{TableBuilder, Column};

use crate::data::AppData;

pub struct TransactionList<'a> {
    id_source: &'a str,
    transaction_ids: &'a Vec<u32>,
    app_data: &'a AppData,
}

impl<'a> TransactionList<'a> {
    pub fn new(
        id_source: &'a str,
        transaction_ids: &'a Vec<u32>,
        app_data: &'a AppData,
    ) -> Self {
        Self {
            id_source,
            transaction_ids,
            app_data,
        }
    }

    pub fn add(self, ui: &mut Ui) {
        let row_height = FontSelection::Default.resolve(ui.style()).size + 2.0;
        TableBuilder::new(ui)
            .striped(true)
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::remainder())
            .column(Column::auto().at_least(100.0))
            .header(row_height, |mut header| {
                header.col(|ui| {
                    ui.label(RichText::new("Date").strong());
                });
                header.col(|ui| {
                    ui.label(RichText::new("Account").strong());
                });
                header.col(|ui| {
                    ui.label(RichText::new("Description").strong());
                });
                header.col(|ui| {
                    ui.label(RichText::new("Amount").strong());
                });
            })
            .body(|mut body| {
                body.rows(row_height, self.transaction_ids.len(), |row_index, mut row| {
                    let transaction = self.app_data.transactions().get(&self.transaction_ids[row_index]).unwrap();
                    let account = self.app_data.accounts().get(&transaction.account_id).unwrap();
                    let currency = self.app_data.currencies().get(&account.currency_id).unwrap();
                    row.col(|ui| {
                        ui.add(Label::new(&transaction.date.to_string()).wrap(false));
                    });
                    row.col(|ui| {
                        ui.add(Label::new(&account.name).wrap(false));
                    });
                    row.col(|ui| {
                        ui.add(Label::new(&transaction.description).wrap(false));
                    });
                    row.col(|ui| {
                        ui.add(Label::new(format!("{} {}", transaction.amount, currency.code)).wrap(false));
                    });
                });
            });
    }
}
