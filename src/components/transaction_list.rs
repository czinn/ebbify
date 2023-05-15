use std::collections::HashSet;

use egui::{FontSelection, Label, RichText, Ui};
use egui_extras::{Column, TableBuilder};

use crate::data::{AppData, Transaction};

enum TransactionsSource<'a> {
    Ids(&'a Vec<u32>),
    Transactions(&'a Vec<Transaction>),
}

impl TransactionsSource<'_> {
    fn len(&self) -> usize {
        match self {
            Self::Ids(ids) => ids.len(),
            Self::Transactions(transactions) => transactions.len(),
        }
    }

    fn iter_ids<'b>(&'b self) -> Box<dyn Iterator<Item=&'b u32> + 'b> {
        match &self {
            TransactionsSource::Ids(ids) => Box::new(ids.iter()),
            TransactionsSource::Transactions(transactions) => Box::new(transactions.iter().map(|t| &t.id)),
        }
    }
}

pub struct TransactionList<'a> {
    transactions: TransactionsSource<'a>,
    selection: Option<&'a mut HashSet<u32>>,
}

impl<'a> TransactionList<'a> {
    pub fn new(transaction_ids: &'a Vec<u32>) -> Self {
        Self {
            transactions: TransactionsSource::Ids(transaction_ids),
            selection: None,
        }
    }

    pub fn new_of_transactions(transactions: &'a Vec<Transaction>) -> Self {
        Self {
            transactions: TransactionsSource::Transactions(transactions),
            selection: None,
        }
    }

    pub fn selection(self, selection: &'a mut HashSet<u32>) -> Self {
        Self {
            selection: Some(selection),
            ..self
        }
    }

    pub fn add(mut self, ui: &mut Ui, app_data: &AppData) {
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
                    transactions,
                    ..
                } = &mut self;
                match selection {
                    Some(selection) => {
                        header.col(|ui| {
                            let all_selected = selection.len() == transactions.len();
                            let mut all_checked = all_selected;
                            ui.checkbox(&mut all_checked, "");
                            if all_checked != all_selected {
                                if all_checked {
                                    transactions.iter_ids().for_each(|id| {
                                        selection.insert(*id);
                                    });
                                } else {
                                    selection.clear();
                                }
                            }
                        });
                    }
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
                    self.transactions.len(),
                    |row_index, mut row| {
                        let transaction = match &self.transactions {
                            TransactionsSource::Ids(ids) => {
                                app_data.transactions().get(&ids[row_index]).unwrap()
                            }
                            TransactionsSource::Transactions(transactions) => {
                                &transactions[row_index]
                            }
                        };
                        let account = app_data.accounts().get(&transaction.account_id).unwrap();
                        let currency = app_data.currencies().get(&account.currency_id).unwrap();
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
                            }
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
