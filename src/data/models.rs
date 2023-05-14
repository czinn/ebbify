use chrono::naive::NaiveDate as Date;
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Balance {
    pub date: Date,
    pub amount: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Account {
    pub id: u32,
    pub name: String,
    pub currency_id: u32,
    pub debit_account: bool,
    pub balances: Vec<Balance>,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum AmortizationType {
    Linear,
    Declining,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Category {
    pub id: u32,
    pub name: String,
    pub parent_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub default_amortization_type: Option<AmortizationType>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub default_amortization_length: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Currency {
    pub id: u32,
    pub code: String,
    pub major: i32,
    pub equivalent_usd: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Flow {
    pub id: u32,
    pub category_id: u32,
    pub date: Date,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<String>,
    pub amount: i32,
    pub currency_id: u32,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub amortization_type: Option<AmortizationType>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub amortization_length: Option<i32>,
    pub transaction_group_id: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TransactionGroup {
    pub id: u32,
    pub transaction_ids: Vec<u32>,
    pub flow_ids: Vec<u32>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: u32,
    pub account_id: u32,
    pub date: Date,
    pub description: String,
    pub amount: i32,
    pub transaction_group_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct FileData {
    accounts: Vec<Account>,
    categories: Vec<Category>,
    currencies: Vec<Currency>,
    flows: Vec<Flow>,
    transactions: Vec<Transaction>,
    transaction_groups: Vec<TransactionGroup>,
}

#[derive(Serialize)]
pub struct FileDataBorrowed<'a> {
    accounts: Vec<&'a Account>,
    categories: Vec<&'a Category>,
    currencies: Vec<&'a Currency>,
    flows: Vec<&'a Flow>,
    transactions: Vec<&'a Transaction>,
    transaction_groups: Vec<&'a TransactionGroup>,
}


impl FileData {
    pub fn sample_data() -> Self {
        let data = r#"
{
  "accounts": [
    {
      "id": 0,
      "name": "Debit Account",
      "currency_id": 0,
      "debit_account": true,
      "balances": []
    },
    {
      "id": 1,
      "name": "Credit Account",
      "currency_id": 0,
      "debit_account": false,
      "balances": []
    },
    {
      "id": 2,
      "name": "CAD Credit",
      "currency_id": 1,
      "debit_account": false,
      "balances": []
    }
  ],
  "categories": [
    {
      "id": 0,
      "name": "Food",
      "parent_id": null
    },
    {
      "id": 1,
      "name": "Groceries",
      "parent_id": 0
    },
    {
      "id": 2,
      "name": "Restaurants",
      "parent_id": 0
    },
    {
      "id": 3,
      "name": "Transit",
      "parent_id": null
    }
  ],
  "currencies": [
    {
      "id": 0,
      "code": "USD",
      "major": 100,
      "equivalent_usd": 1
    },
    {
      "id": 1,
      "code": "CAD",
      "major": 100,
      "equivalent_usd": 0.73214
    }
  ],
  "flows": [],
  "transactions": [],
  "transaction_groups": []
}
        "#;

        let mut data: FileData = serde_json::from_str(data).unwrap();

        for id in 0..1000 {
            data.transactions.push(
                Transaction {
                    id,
                    account_id: id % 3,
                    date: Date::from_ymd_opt(2023, 5, id % 31 + 1).unwrap(),
                    description: format!("Transaction {}", id).into(),
                    amount: ((id as i32) % 10) * 10 - 50,
                    transaction_group_id: None,
                },
            );
        }

        data
    }
}

#[derive(Debug)]
pub struct CategoryNode {
    pub id: u32,
    pub children: Vec<CategoryNode>,
}

impl CategoryNode {
    fn new(id: u32, children_map: &BTreeMap<u32, Vec<u32>>) -> Self {
        Self {
            id,
            children: children_map.get(&id).map_or_else(
                || Vec::new(),
                |children| {
                    children
                        .iter()
                        .map(|id| CategoryNode::new(*id, children_map))
                        .collect()
                },
            ),
        }
    }
}

pub struct AppData {
    accounts: BTreeMap<u32, Account>,
    categories: BTreeMap<u32, Category>,
    currencies: BTreeMap<u32, Currency>,
    flows: BTreeMap<u32, Flow>,
    transactions: BTreeMap<u32, Transaction>,
    transaction_groups: BTreeMap<u32, TransactionGroup>,
    modified: bool,
    category_trees: Vec<CategoryNode>,
}

#[allow(dead_code)]
impl AppData {
    pub fn new() -> Self {
        Self {
            accounts: Default::default(),
            categories: Default::default(),
            currencies: Default::default(),
            flows: Default::default(),
            transactions: Default::default(),
            transaction_groups: Default::default(),
            modified: true,
            category_trees: Vec::new(),
        }
    }

    pub fn from_file(data: FileData) -> Self {
        let mut t = Self {
            accounts: data.accounts.into_iter().map(|x| (x.id, x)).collect(),
            categories: data.categories.into_iter().map(|x| (x.id, x)).collect(),
            currencies: data.currencies.into_iter().map(|x| (x.id, x)).collect(),
            flows: data.flows.into_iter().map(|x| (x.id, x)).collect(),
            transactions: data.transactions.into_iter().map(|x| (x.id, x)).collect(),
            transaction_groups: data.transaction_groups.into_iter().map(|x| (x.id, x)).collect(),
            modified: false,
            category_trees: Vec::new(),
        };
        t.recompute_category_trees();
        t
    }

    fn recompute_category_trees(&mut self) {
        let mut roots: Vec<u32> = Vec::new();
        let mut children_map: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
        for (id, category) in self.categories.iter() {
            match category.parent_id {
                Some(parent_id) => children_map
                    .entry(parent_id)
                    .or_insert_with(|| Vec::new())
                    .push(*id),
                None => roots.push(*id),
            }
        }
        self.category_trees = roots
            .into_iter()
            .map(|id| CategoryNode::new(id, &children_map))
            .collect();
    }

    pub fn is_modified(&self) -> bool {
        self.modified
    }

    pub fn mark_saved(&mut self) {
        self.modified = false;
    }

    pub fn file_data<'a>(&'a self) -> FileDataBorrowed<'a> {
        FileDataBorrowed {
            accounts: self.accounts.values().collect(),
            categories: self.categories.values().collect(),
            currencies: self.currencies.values().collect(),
            flows: self.flows.values().collect(),
            transactions: self.transactions.values().collect(),
            transaction_groups: self.transaction_groups.values().collect(),
        }
    }

    pub fn accounts(&self) -> &BTreeMap<u32, Account> {
        &self.accounts
    }

    pub fn accounts_mut<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut BTreeMap<u32, Account>) -> R,
    {
        self.modified = true;
        f(&mut self.accounts)
    }

    pub fn categories(&self) -> &BTreeMap<u32, Category> {
        &self.categories
    }

    pub fn categories_mut<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut BTreeMap<u32, Category>) -> R,
    {
        self.modified = true;
        let result = f(&mut self.categories);
        self.recompute_category_trees();
        result
    }

    pub fn currencies(&self) -> &BTreeMap<u32, Currency> {
        &self.currencies
    }

    pub fn currencies_mut<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut BTreeMap<u32, Currency>) -> R,
    {
        self.modified = true;
        f(&mut self.currencies)
    }

    pub fn flows(&self) -> &BTreeMap<u32, Flow> {
        &self.flows
    }

    pub fn flows_mut<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut BTreeMap<u32, Flow>) -> R,
    {
        self.modified = true;
        f(&mut self.flows)
    }

    pub fn transactions(&self) -> &BTreeMap<u32, Transaction> {
        &self.transactions
    }

    pub fn transactions_mut<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut BTreeMap<u32, Transaction>) -> R,
    {
        self.modified = true;
        f(&mut self.transactions)
    }

    pub fn transaction_groups(&self) -> &BTreeMap<u32, TransactionGroup> {
        &self.transaction_groups
    }

    pub fn transaction_groups_mut<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut BTreeMap<u32, TransactionGroup>) -> R,
    {
        self.modified = true;
        f(&mut self.transaction_groups)
    }

    pub fn category_trees(&self) -> &Vec<CategoryNode> {
        &self.category_trees
    }
}
