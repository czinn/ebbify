use chrono::naive::NaiveDate as Date;
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub id: u32,
    pub name: String,
    pub currency_id: u32,
    pub debit_account: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Balance {
    pub id: u32,
    pub account_id: u32,
    pub date: Date,
    pub amount: i32,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum AmortizationType {
    Linear,
    Declining,
}

#[derive(Serialize, Deserialize)]
pub struct Category {
    pub id: u32,
    pub name: String,
    pub parent_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub default_amortization_type: Option<AmortizationType>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub default_amortization_length: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct Currency {
    pub id: u32,
    pub code: String,
    pub major: i32,
    pub equivalent_usd: f32,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct TransactionGroup {
    pub id: u32,
}

#[derive(Serialize, Deserialize)]
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
    accounts: BTreeMap<u32, Account>,
    balances: BTreeMap<u32, Balance>,
    categories: BTreeMap<u32, Category>,
    currencies: BTreeMap<u32, Currency>,
    flows: BTreeMap<u32, Flow>,
    transactions: BTreeMap<u32, Transaction>,
}

impl FileData {
    pub fn sample_data() -> Self {
        let data = r#"
{
  "accounts": {
    "0": {
      "id": 0,
      "name": "Debit Account",
      "currency_id": 0,
      "debit_account": true
    },
    "1": {
      "id": 1,
      "name": "Credit Account",
      "currency_id": 0,
      "debit_account": false
    },
    "2": {
      "id": 2,
      "name": "CAD Credit",
      "currency_id": 1,
      "debit_account": false
    }
  },
  "balances": {},
  "categories": {
    "0": {
      "id": 0,
      "name": "Food",
      "parent_id": null
    },
    "1": {
      "id": 1,
      "name": "Groceries",
      "parent_id": 0
    },
    "2": {
      "id": 2,
      "name": "Restaurants",
      "parent_id": 0
    },
    "3": {
      "id": 3,
      "name": "Transit",
      "parent_id": null
    }
  },
  "currencies": {
    "0": {
      "id": 0,
      "code": "USD",
      "major": 100,
      "equivalent_usd": 1
    },
    "1": {
      "id": 1,
      "code": "CAD",
      "major": 100,
      "equivalent_usd": 0.73214
    }
  },
  "flows": {},
  "transactions": {}
}
        "#;

        serde_json::from_str(data).unwrap()
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
    data: FileData,
    modified: bool,
    category_trees: Vec<CategoryNode>,
}

#[allow(dead_code)]
impl AppData {
    pub fn new() -> Self {
        Self {
            data: Default::default(),
            modified: true,
            category_trees: Vec::new(),
        }
    }

    pub fn from_file(data: FileData) -> Self {
        let mut t = Self {
            data,
            modified: false,
            category_trees: Vec::new(),
        };
        t.recompute_category_trees();
        t
    }

    fn recompute_category_trees(&mut self) {
        let mut roots: Vec<u32> = Vec::new();
        let mut children_map: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
        for (id, category) in self.data.categories.iter() {
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

    pub fn file_data(&self) -> &FileData {
        &self.data
    }

    pub fn accounts(&self) -> &BTreeMap<u32, Account> {
        &self.data.accounts
    }

    pub fn accounts_mut<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut BTreeMap<u32, Account>) -> R,
    {
        self.modified = true;
        f(&mut self.data.accounts)
    }

    pub fn balances(&self) -> &BTreeMap<u32, Balance> {
        &self.data.balances
    }

    pub fn balances_mut<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut BTreeMap<u32, Balance>) -> R,
    {
        self.modified = true;
        f(&mut self.data.balances)
    }

    pub fn categories(&self) -> &BTreeMap<u32, Category> {
        &self.data.categories
    }

    pub fn categories_mut<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut BTreeMap<u32, Category>) -> R,
    {
        self.modified = true;
        let result = f(&mut self.data.categories);
        self.recompute_category_trees();
        result
    }

    pub fn currencies(&self) -> &BTreeMap<u32, Currency> {
        &self.data.currencies
    }

    pub fn currencies_mut<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut BTreeMap<u32, Currency>) -> R,
    {
        self.modified = true;
        f(&mut self.data.currencies)
    }

    pub fn flows(&self) -> &BTreeMap<u32, Flow> {
        &self.data.flows
    }

    pub fn flows_mut<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut BTreeMap<u32, Flow>) -> R,
    {
        self.modified = true;
        f(&mut self.data.flows)
    }

    pub fn transactions(&self) -> &BTreeMap<u32, Transaction> {
        &self.data.transactions
    }

    pub fn transactions_mut<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut BTreeMap<u32, Transaction>) -> R,
    {
        self.modified = true;
        f(&mut self.data.transactions)
    }

    pub fn category_trees(&self) -> &Vec<CategoryNode> {
        &self.category_trees
    }
}
