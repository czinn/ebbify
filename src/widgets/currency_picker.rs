use egui::{ComboBox, Response, Ui, Widget};

use crate::data::AppData;

pub struct CurrencyPicker<'a> {
    id_source: &'a str,
    selected: &'a mut Option<u32>,
    null_allowed: bool,
    app_data: &'a AppData,
}

impl<'a> CurrencyPicker<'a> {
    pub fn new(
        id_source: &'a str,
        selected: &'a mut Option<u32>,
        null_allowed: bool,
        app_data: &'a AppData,
    ) -> Self {
        Self {
            id_source,
            selected,
            null_allowed,
            app_data,
        }
    }

    fn selected_text(&self) -> &str {
        match &self.selected {
            Some(selected) => &self.app_data.currencies().get(selected).unwrap().code,
            None => "",
        }
    }
}

impl<'a> Widget for CurrencyPicker<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ComboBox::from_id_source(self.id_source)
            .selected_text(self.selected_text())
            .show_ui(ui, |ui| {
                if self.selected.is_none() || self.null_allowed {
                    ui.selectable_value(self.selected, None, "");
                }
                for currency in self.app_data.currencies().values() {
                    ui.selectable_value(self.selected, Some(currency.id), &currency.code);
                }
            })
            .response
    }
}
