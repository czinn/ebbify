use crate::components::MenuBar;
use crate::data::SaveFile;
use crate::ui_state::UiState;

pub const APP_NAME: &str = "Ebbify";

pub struct App {
    cached_title: String,
    save_file: Option<SaveFile>,
    ui_state: UiState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            cached_title: APP_NAME.into(),
            save_file: None,
            ui_state: Default::default(),
        }
    }
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let target_title = match &self.save_file {
            Some(save_file) => format!(
                "{} - {}{}",
                APP_NAME,
                save_file
                    .path
                    .as_path()
                    .file_name()
                    .unwrap_or(std::ffi::OsStr::new("Sample"))
                    .to_string_lossy(),
                if save_file.is_modified() {
                    "*"
                } else {
                    ""
                }
            ),
            None => APP_NAME.into(),
        };
        if self.cached_title != target_title {
            frame.set_window_title(&target_title);
            self.cached_title = target_title;
        }

        egui::TopBottomPanel::top("top-panel").show(ctx, |ui| {
            let menu_bar_response = MenuBar::add(ui, ctx, &mut self.save_file);
            if menu_bar_response.save_file_changed {
                self.ui_state = Default::default();
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| match &mut self.save_file {
            Some(save_file) => {
                self.ui_state.add_tab_selector(ui);
                ui.separator();
                self.ui_state
                    .add_current_tab(ui, ctx, &mut save_file.app_data);
            }
            None => {
                ui.heading("Load a budget");
            }
        });
    }
}
