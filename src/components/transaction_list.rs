use std::collections::HashSet;

use egui::{FontSelection, Label, RichText, Ui};
use egui_extras::{Column, TableBuilder};

use crate::data::AppData;

pub struct TransactionList<'a> {
    transaction_ids: &'a Vec<u32>,
    app_data: &'a AppData,
    selection: Option<&'a mut HashSet<u32>>,
}

impl<'a> TransactionList<'a> {
    pub fn new(transaction_ids: &'a Vec<u32>, app_data: &'a AppData) -> Self {
        Self {
            transaction_ids,
            app_data,
            selection: None,
        }
    }

    pub fn selection(self, selection: &'a mut HashSet<u32>) -> Self {
        Self {
            selection: Some(selection),
            ..self
        }
    }

    pub fn add(mut self, ui: &mut Ui) {
        let row_height = FontSelection::Default.resolve(ui.style()).size + 6.0;
        let builder = TableBuilder::new(ui)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center));
        let builder = match &self.selection {
            Some(_selection) => builder.column(Column::auto()),
            None => builder,
        };
        builder
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::remainder())
            .column(Column::auto().at_least(100.0))
            .header(row_height, |mut header| {
                let Self {
                    selection,
                    transaction_ids,
                    ..
                } = &mut self;
                match selection {
                    Some(selection) => {
                        header.col(|ui| {
                            let all_selected = selection.len() == transaction_ids.len();
                            let mut all_checked = all_selected;
                            ui.checkbox(&mut all_checked, "");
                            if all_checked != all_selected {
                                if all_checked {
                                    transaction_ids.iter().for_each(|id| {
                                        selection.insert(*id);
                                    });
                                } else {
                                    selection.clear();
                                }
                            }
                        });
                    },
                    None => (),
                }
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
            .body(|body| {
                body.rows(
                    row_height,
                    self.transaction_ids.len(),
                    |row_index, mut row| {
                        let transaction = self
                            .app_data
                            .transactions()
                            .get(&self.transaction_ids[row_index])
                            .unwrap();
                        let account = self
                            .app_data
                            .accounts()
                            .get(&transaction.account_id)
                            .unwrap();
                        let currency = self
                            .app_data
                            .currencies()
                            .get(&account.currency_id)
                            .unwrap();
                        match &mut self.selection {
                            Some(selection) => {
                                let row_selected = selection.contains(&transaction.id);
                                let mut checked = row_selected;
                                row.col(|ui| {
                                    ui.checkbox(&mut checked, "");
                                });
                                if row_selected != checked {
                                    if checked {
                                        (*selection).insert(transaction.id);
                                    } else {
                                        (*selection).remove(&transaction.id);
                                    }
                                }
                            },
                            None => (),
                        }
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
                            ui.add(
                                Label::new(format!("{} {}", transaction.amount, currency.code))
                                    .wrap(false),
                            );
                        });
                    },
                );
            });
    }
}
