use egui::{Button, Key, KeyboardShortcut, Modifiers, Widget};

use crate::data::SaveFile;

pub struct MenuBarResponse {
    pub save_file_changed: bool,
}

pub struct MenuBar;

impl MenuBar {
    fn button_with_shortcut(
        ui: &mut egui::Ui,
        ctx: &egui::Context,
        name: &str,
        modifiers: Modifiers,
        key: Key,
    ) -> (impl Widget, bool) {
        let shortcut = KeyboardShortcut::new(modifiers, key);
        let shortcut_pressed = ui.input_mut(|i| i.consume_shortcut(&shortcut));
        (
            Button::new(name).shortcut_text(ctx.format_shortcut(&shortcut)),
            shortcut_pressed,
        )
    }

    pub fn add(
        ui: &mut egui::Ui,
        ctx: &egui::Context,
        save_file: &mut Option<SaveFile>,
    ) -> MenuBarResponse {
        let (new_button, mut new_pressed) =
            Self::button_with_shortcut(ui, ctx, "New", Modifiers::COMMAND, Key::N);
        let (open_button, mut open_pressed) =
            Self::button_with_shortcut(ui, ctx, "Open", Modifiers::COMMAND, Key::O);
        let (save_button, mut save_pressed) =
            Self::button_with_shortcut(ui, ctx, "Save", Modifiers::COMMAND, Key::S);
        let (load_sample_button, mut load_sample_pressed) = Self::button_with_shortcut(
            ui,
            ctx,
            "Load sample",
            Modifiers::COMMAND.plus(Modifiers::SHIFT),
            Key::L,
        );

        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.add(new_button).clicked() {
                    new_pressed = true;
                    ui.close_menu();
                }
                if ui.add(open_button).clicked() {
                    open_pressed = true;
                    ui.close_menu();
                }
                ui.add_enabled_ui(save_file.is_some(), |ui| {
                    if ui.add(save_button).clicked() {
                        save_pressed = true;
                        ui.close_menu();
                    }
                });
                if ui.add(load_sample_button).clicked() {
                    load_sample_pressed = true;
                    ui.close_menu();
                }
            });
        });

        let mut save_file_changed = false;

        if new_pressed {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("ebbify", &["ebb"])
                .save_file()
            {
                *save_file = Some(SaveFile::new(path));
                save_file_changed = true;
            }
        }

        if open_pressed {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("ebbify", &["ebb"])
                .pick_file()
            {
                match SaveFile::load(path) {
                    Ok(file) => {
                        *save_file = Some(file);
                        save_file_changed = true;
                    }
                    Err(err) => {
                        println!("Failed to load budget file: {:?}", err);
                    }
                }
            }
        }

        if save_pressed {
            match save_file {
                Some(save_file) => {
                    let _ = save_file.save();
                }
                None => (),
            }
        }

        if load_sample_pressed {
            *save_file = Some(SaveFile::load_sample());
            save_file_changed = true;
        }

        MenuBarResponse { save_file_changed }
    }
}
