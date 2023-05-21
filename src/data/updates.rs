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

macro_rules! set_or_delete_option {
    ( $old_op:ident, $id:ident, $set:ident, $delete:ident ) => {
        match $old_op {
            Some($old_op) => Self::$set($old_op),
            None => Self::$delete($id),
        }
    };
}

impl Update {
    /// `apply` performs the update on `app_data` and returns the reverse update.
    pub fn apply(self, app_data: &mut AppData) -> Self {
        match self {
            Self::SetAccount(account) => {
                let id = account.id;
                let old_account = app_data.accounts.insert(id, account);
                set_or_delete_option!(old_account, id, SetAccount, DeleteAccount)
            }
            Self::DeleteAccount(id) => {
                let old_account = app_data.accounts.remove(&id);
                set_or_delete_option!(old_account, id, SetAccount, DeleteAccount)
            }
            Self::SetCategory(category) => {
                let id = category.id;
                let old_category = app_data.categories.insert(id, category);
                set_or_delete_option!(old_category, id, SetCategory, DeleteCategory)
            }
            Self::DeleteCategory(id) => {
                let old_category = app_data.categories.remove(&id);
                set_or_delete_option!(old_category, id, SetCategory, DeleteCategory)
            }
            Self::SetCurrency(currency) => {
                let id = currency.id;
                let old_currency = app_data.currencies.insert(id, currency);
                set_or_delete_option!(old_currency, id, SetCurrency, DeleteCurrency)
            }
            Self::DeleteCurrency(id) => {
                let old_currency = app_data.currencies.remove(&id);
                set_or_delete_option!(old_currency, id, SetCurrency, DeleteCurrency)
            }
            Self::SetFlow(flow) => {
                let id = flow.id;
                let old_flow = app_data.flows.insert(id, flow);
                set_or_delete_option!(old_flow, id, SetFlow, DeleteFlow)
            }
            Self::DeleteFlow(id) => {
                let old_flow = app_data.flows.remove(&id);
                set_or_delete_option!(old_flow, id, SetFlow, DeleteFlow)
            }
            Self::SetTransactionGroup(transaction_group) => {
                let id = transaction_group.id;
                let old_transaction_group =
                    app_data.transaction_groups.insert(id, transaction_group);
                set_or_delete_option!(
                    old_transaction_group,
                    id,
                    SetTransactionGroup,
                    DeleteTransactionGroup
                )
            }
            Self::DeleteTransactionGroup(id) => {
                let old_transaction_group = app_data.transaction_groups.remove(&id);
                set_or_delete_option!(
                    old_transaction_group,
                    id,
                    SetTransactionGroup,
                    DeleteTransactionGroup
                )
            }
            Self::SetTransaction(transaction) => {
                let id = transaction.id;
                let old_transaction = app_data.insert_transaction(transaction);
                set_or_delete_option!(old_transaction, id, SetTransaction, DeleteTransaction)
            }
            Self::DeleteTransaction(id) => {
                let old_transaction = app_data.remove_transaction(id);
                set_or_delete_option!(old_transaction, id, SetTransaction, DeleteTransaction)
            }
        }
    }
}

pub struct Updates {
    #[allow(dead_code)]
    time: SystemTime,
    updates: Vec<Update>,
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
        let mut reverse_updates = Vec::new();
        for update in self.updates.into_iter() {
            reverse_updates.push(update.apply(app_data));
        }
        reverse_updates.reverse();
        Self {
            time: SystemTime::now(),
            updates: reverse_updates,
        }
    }
}
