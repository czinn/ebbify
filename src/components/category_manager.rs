use egui::{Button, Context, Grid, Ui, Window};

use crate::data::{AmortizationType, AppData, Category, CategoryNode, SaveFile};
use crate::widgets::CategoryPicker;

#[derive(Default)]
struct NewCategory {
    name: String,
    parent_id: Option<u32>,
    default_amortization_type: Option<AmortizationType>,
    default_amortization_length: Option<i32>,
    autofocus: bool,
}

#[derive(Default)]
pub struct CategoryManager {
    new_category: Option<NewCategory>,
}

impl CategoryManager {
    // Returns a node to remove
    fn show_node(&mut self, node: &CategoryNode, ui: &mut Ui, app_data: &AppData) -> Option<u32> {
        let mut node_to_remove = None;
        ui.horizontal(|ui| {
            ui.label(&app_data.categories().get(&node.id).unwrap().name);
            if ui.link("+").clicked() {
                if self.new_category.is_none() {
                    self.new_category = Some(NewCategory {
                        parent_id: Some(node.id),
                        autofocus: true,
                        ..Default::default()
                    });
                }
            }
            if node.children.len() == 0 {
                if ui.link("-").clicked() {
                    node_to_remove = Some(node.id);
                }
            }
        });
        if node.children.len() > 0 {
            ui.indent(node.id, |ui| {
                for child in node.children.iter() {
                    if let Some(id) = self.show_node(child, ui, app_data) {
                        node_to_remove = Some(id);
                    }
                }
            });
        }
        node_to_remove
    }

    pub fn add(&mut self, ui: &mut Ui, ctx: &Context, save_file: &mut SaveFile) {
        let mut node_to_remove = None;
        for node in save_file.app_data.category_trees().iter() {
            if let Some(id) = self.show_node(node, ui, &save_file.app_data) {
                node_to_remove = Some(id);
            }
        }
        if let Some(node_to_remove) = node_to_remove {
            save_file
                .app_data
                .categories_mut(|categories| categories.remove(&node_to_remove));
            save_file.modified = true;
        }

        if ui.button("New Category").clicked() {
            self.new_category = Some(NewCategory {
                autofocus: true,
                ..Default::default()
            });
        }

        let mut is_open = true;
        let mut clicked_create = false;
        if let Some(new_category) = &mut self.new_category {
            Window::new("New Category")
                .open(&mut is_open)
                .collapsible(false)
                .show(ctx, |ui| {
                    Grid::new("new-category-grid")
                        .num_columns(2)
                        .spacing([40.0, 4.0])
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label("Name");
                            let resp = ui.text_edit_singleline(&mut new_category.name);
                            if new_category.autofocus {
                                new_category.autofocus = false;
                                ui.memory_mut(|m| m.request_focus(resp.id));
                            }
                            ui.end_row();

                            ui.label("Parent category");
                            ui.add(CategoryPicker::new(
                                "new-category-parent-picker",
                                &mut new_category.parent_id,
                                true,
                                &save_file.app_data,
                            ));
                            ui.end_row();
                        });
                    if ui
                        .add_enabled(new_category.name.len() > 0, Button::new("Create"))
                        .clicked()
                    {
                        clicked_create = true;
                    }
                });
        }

        if clicked_create {
            let next_id = save_file
                .app_data
                .categories()
                .last_key_value()
                .map_or(0, |(k, _)| *k + 1);
            let NewCategory {
                name,
                parent_id,
                default_amortization_type,
                default_amortization_length,
                autofocus: _,
            } = self.new_category.take().unwrap();
            save_file.app_data.categories_mut(|categories| {
                categories.insert(
                    next_id,
                    Category {
                        id: next_id,
                        name,
                        parent_id,
                        default_amortization_type,
                        default_amortization_length,
                    },
                )
            });
            save_file.modified = true;
        }

        if !is_open || clicked_create {
            self.new_category = None;
        }
    }
}
