use egui::{ComboBox, Response, Ui, Widget};

use crate::data::{AppData, CategoryNode};

pub struct CategoryPicker<'a> {
    id_source: &'a str,
    selected: &'a mut Option<u32>,
    null_allowed: bool,
    app_data: &'a AppData,
}

impl<'a> CategoryPicker<'a> {
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

    fn show_node(&mut self, node: &CategoryNode, ui: &mut Ui) {
        ui.selectable_value(
            self.selected,
            Some(node.id),
            &self.app_data.categories().get(&node.id).unwrap().name,
        );
        if node.children.len() > 0 {
            ui.indent(node.id, |ui| {
                for child in node.children.iter() {
                    self.show_node(child, ui);
                }
            });
        }
    }

    fn selected_text(&self) -> &str {
        match &self.selected {
            Some(selected) => &self.app_data.categories().get(selected).unwrap().name,
            None => "",
        }
    }
}

impl<'a> Widget for CategoryPicker<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        ComboBox::from_id_source(self.id_source)
            .width(200.0)
            .selected_text(self.selected_text())
            .show_ui(ui, |ui| {
                if self.selected.is_none() || self.null_allowed {
                    ui.selectable_value(self.selected, None, "");
                }
                for node in self.app_data.category_trees().iter() {
                    self.show_node(node, ui);
                }
            })
            .response
    }
}
