use egui::{Context, Ui};

use crate::components::{AccountManager, CategoryManager, CurrencyManager};
use crate::data::AppData;

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum Tab {
    #[default]
    CategoryManager,
    CurrencyManager,
    AccountManager,
}

#[derive(Default)]
pub struct UiState {
    pub current_tab: Tab,
    pub category_manager: CategoryManager,
    pub currency_manager: CurrencyManager,
    pub account_manager: AccountManager,
}

impl UiState {
    pub fn add_tab_selector(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            for (tab, name) in &[
                (Tab::CategoryManager, "Categories"),
                (Tab::CurrencyManager, "Currencies"),
                (Tab::AccountManager, "Accounts"),
            ] {
                ui.selectable_value(&mut self.current_tab, *tab, *name);
            }
        });
    }

    pub fn add_current_tab(&mut self, ui: &mut Ui, ctx: &Context, app_data: &mut AppData) {
        match self.current_tab {
            Tab::CategoryManager => self.category_manager.add(ui, ctx, app_data),
            Tab::CurrencyManager => self.currency_manager.add(ui, ctx, app_data),
            Tab::AccountManager => self.account_manager.add(ui, ctx, app_data),
        }
    }
}
