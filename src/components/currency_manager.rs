use egui::{Button, Context, DragValue, Grid, RichText, Ui, Window};

use crate::data::{next_id, AppData, Currency};

struct CurrencyEditor {
    id: Option<u32>,
    code: String,
    major: i32,
    equivalent_usd: f32,
    symbol: String,
    autofocus: bool,
}

impl Default for CurrencyEditor {
    fn default() -> Self {
        Self {
            id: None,
            code: Default::default(),
            major: 100,
            equivalent_usd: 1.0,
            symbol: "$".into(),
            autofocus: true,
        }
    }
}

impl CurrencyEditor {
    fn of_currency(currency: &Currency) -> Self {
        let Currency {
            id,
            code,
            major,
            equivalent_usd,
            symbol,
        } = currency;
        Self {
            id: Some(*id),
            code: code.clone(),
            major: *major,
            equivalent_usd: *equivalent_usd,
            symbol: symbol.clone(),
            autofocus: true,
        }
    }
}

#[derive(Default)]
pub struct CurrencyManager {
    currency_editor: Option<CurrencyEditor>,
}

impl CurrencyManager {
    pub fn add(&mut self, ui: &mut Ui, ctx: &Context, app_data: &mut AppData) {
        Grid::new("currency-manager-grid")
            .num_columns(4)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label(RichText::new("Currency").strong());
                ui.label(RichText::new("Equivalent USD").strong());
                ui.label(RichText::new("Minor to Major").strong());
                ui.label(RichText::new("Edit").strong());
                ui.end_row();
                for currency in app_data.currencies().values() {
                    ui.label(&currency.code);
                    ui.label(format!("{}", &currency.equivalent_usd));
                    ui.label(format!("{}", &currency.major));
                    if ui.button("Edit").clicked() {
                        if self.currency_editor.is_none() {
                            self.currency_editor = Some(CurrencyEditor::of_currency(currency));
                        }
                    }
                    ui.end_row();
                }
            });

        if ui.button("New Currency").clicked() {
            self.currency_editor = Some(Default::default());
        }

        let mut is_open = true;
        let mut clicked_create = false;
        if let Some(currency_editor) = &mut self.currency_editor {
            let (title, button_text) = if currency_editor.id.is_some() {
                ("Edit Currency", "Save")
            } else {
                ("New Currency", "Create")
            };
            Window::new(title)
                .open(&mut is_open)
                .collapsible(false)
                .show(ctx, |ui| {
                    Grid::new("currency-editor-grid")
                        .num_columns(2)
                        .spacing([40.0, 4.0])
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label("Code");
                            let resp = ui.text_edit_singleline(&mut currency_editor.code);
                            if currency_editor.autofocus {
                                currency_editor.autofocus = false;
                                ui.memory_mut(|m| m.request_focus(resp.id));
                            }
                            ui.end_row();

                            ui.label("Equivalent USD");
                            ui.add(
                                DragValue::new(&mut currency_editor.equivalent_usd)
                                    .speed(0.01)
                                    .min_decimals(2),
                            );
                            ui.end_row();

                            ui.label("Minor to Major");
                            ui.add(DragValue::new(&mut currency_editor.major));
                            ui.end_row();

                            ui.label("Symbol");
                            ui.text_edit_singleline(&mut currency_editor.symbol);
                            ui.end_row();
                        });
                    let is_ok = currency_editor.code.len() > 0;
                    if ui.add_enabled(is_ok, Button::new(button_text)).clicked() {
                        clicked_create = true;
                    }
                });
        }

        if clicked_create {
            let CurrencyEditor {
                id,
                code,
                major,
                equivalent_usd,
                symbol,
                autofocus: _,
            } = self.currency_editor.take().unwrap();
            let id = match id {
                Some(id) => id,
                None => next_id(app_data.currencies()),
            };
            app_data.currencies_mut(|currencies| {
                currencies.insert(
                    id,
                    Currency {
                        id,
                        code,
                        major,
                        equivalent_usd,
                        symbol,
                    },
                )
            });
        }

        if !is_open || clicked_create {
            self.currency_editor = None;
        }
    }
}
