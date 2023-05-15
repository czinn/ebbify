use egui::{Button, Context, Grid, Ui, Window};

use crate::data::{next_id, AmortizationType, AppData, Category, CategoryNode};
use crate::widgets::CategoryPicker;

#[derive(Default)]
struct CategoryEditor {
    id: Option<u32>,
    name: String,
    parent_id: Option<u32>,
    default_amortization_type: Option<AmortizationType>,
    default_amortization_length: Option<i32>,
    autofocus: bool,
}

impl CategoryEditor {
    fn of_category(category: &Category) -> Self {
        Self {
            id: Some(category.id),
            name: category.name.clone(),
            parent_id: category.parent_id,
            default_amortization_type: category.default_amortization_type,
            default_amortization_length: category.default_amortization_length,
            autofocus: true,
        }
    }
}

#[derive(Default)]
pub struct CategoryManager {
    category_editor: Option<CategoryEditor>,
}

impl CategoryManager {
    // Returns a node to remove
    fn show_node(&mut self, node: &CategoryNode, ui: &mut Ui, app_data: &AppData) -> Option<u32> {
        let mut node_to_remove = None;
        ui.horizontal(|ui| {
            ui.label(&app_data.categories().get(&node.id).unwrap().name);
            if ui.link("+").clicked() {
                if self.category_editor.is_none() {
                    self.category_editor = Some(CategoryEditor {
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
            if ui.link("Edit").clicked() {
                if self.category_editor.is_none() {
                    self.category_editor = Some(CategoryEditor::of_category(
                        app_data.categories().get(&node.id).unwrap(),
                    ));
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

    pub fn add(&mut self, ui: &mut Ui, ctx: &Context, app_data: &mut AppData) {
        let mut node_to_remove = None;
        for node in app_data.category_trees().iter() {
            if let Some(id) = self.show_node(node, ui, app_data) {
                node_to_remove = Some(id);
            }
        }
        if let Some(node_to_remove) = node_to_remove {
            app_data.categories_mut(|categories| categories.remove(&node_to_remove));
        }

        if ui.button("New Category").clicked() {
            self.category_editor = Some(CategoryEditor {
                autofocus: true,
                ..Default::default()
            });
        }

        let mut is_open = true;
        let mut clicked_create = false;
        if let Some(category_editor) = &mut self.category_editor {
            let (title, button_text) = if category_editor.id.is_some() {
                ("Edit Category", "Save")
            } else {
                ("New Category", "Create")
            };
            Window::new(title)
                .open(&mut is_open)
                .collapsible(false)
                .show(ctx, |ui| {
                    Grid::new("category-editor-grid")
                        .num_columns(2)
                        .spacing([40.0, 4.0])
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label("Name");
                            let resp = ui.text_edit_singleline(&mut category_editor.name);
                            if category_editor.autofocus {
                                category_editor.autofocus = false;
                                ui.memory_mut(|m| m.request_focus(resp.id));
                            }
                            ui.end_row();

                            ui.label("Parent category");
                            ui.add(CategoryPicker::new(
                                "new-category-parent-picker",
                                &mut category_editor.parent_id,
                                true,
                                &category_editor.id,
                                app_data,
                            ));
                            ui.end_row();
                        });
                    if ui
                        .add_enabled(category_editor.name.len() > 0, Button::new(button_text))
                        .clicked()
                    {
                        clicked_create = true;
                    }
                });
        }

        if clicked_create {
            let CategoryEditor {
                id,
                name,
                parent_id,
                default_amortization_type,
                default_amortization_length,
                autofocus: _,
            } = self.category_editor.take().unwrap();
            let id = match id {
                Some(id) => id,
                None => next_id(app_data.categories()),
            };
            app_data.categories_mut(|categories| {
                categories.insert(
                    id,
                    Category {
                        id,
                        name,
                        parent_id,
                        default_amortization_type,
                        default_amortization_length,
                    },
                )
            });
        }

        if !is_open || clicked_create {
            self.category_editor = None;
        }
    }
}
