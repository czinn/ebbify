use std::time::SystemTime;

use super::models::*;

pub enum Update {
    SetAccount(Account),
    DeleteAccount(u32),
    SetCategory(Category),
    DeleteCategory(u32),
    SetCurrency(Currency),
    DeleteCurrency(u32),
    SetFlow(Flow),
    DeleteFlow(u32),
    SetTransactionGroup(TransactionGroup),
    DeleteTransactionGroup(u32),
    SetTransaction(Transaction),
    DeleteTransaction(u32),
}

macro_rules! update_apply {
    ( $( ($name:ident, $plural:ident, $set:ident, $delete:ident) ),* ) => {
        /// `apply` performs the update on `app_data` and returns the reverse update.
        pub fn apply(self, app_data: &mut AppData) -> Self {
            match self {
                $(
                    Self::$set($name) => {
                        let id = $name.id;
                        let $name = app_data.$plural.insert(id, $name);
                        match $name {
                            Some($name) => Self::$set($name),
                            None => Self::$delete(id),
                        }
                    }
                    Self::$delete(id) => {
                        let $name = app_data.$plural.remove(&id);
                        match $name {
                            Some($name) => Self::$set($name),
                            None => Self::$delete(id),
                        }
                    }
                )*
            }
        }
    };
}

impl Update {
    update_apply![
        (account, accounts, SetAccount, DeleteAccount),
        (category, categories, SetCategory, DeleteCategory),
        (currency, currencies, SetCurrency, DeleteCurrency),
        (flow, flows, SetFlow, DeleteFlow),
        (
            transaction_group,
            transaction_groups,
            SetTransactionGroup,
            DeleteTransactionGroup
        ),
        (transaction, transactions, SetTransaction, DeleteTransaction)
    ];

    fn modifies_categories(&self) -> bool {
        match self {
            Self::SetCategory(_) | Self::DeleteCategory(_) => true,
            _ => false,
        }
    }
}

pub struct Updates {
    pub time: SystemTime,
    pub updates: Vec<Update>,
}

impl Updates {
    pub fn new(updates: Vec<Update>) -> Self {
        Self {
            time: SystemTime::now(),
            updates,
        }
    }

    /// `apply` performs the updates on `app_data` and returns the reverse updates.
    pub fn apply(self, app_data: &mut AppData) -> Self {
        let modifies_categories = self.updates.iter().any(|x| x.modifies_categories());

        let mut reverse_updates = Vec::new();
        for update in self.updates.into_iter() {
            reverse_updates.push(update.apply(app_data));
        }
        reverse_updates.reverse();

        if modifies_categories {
            app_data.recompute_category_trees();
        }

        Self {
            time: SystemTime::now(),
            updates: reverse_updates,
        }
    }
}
