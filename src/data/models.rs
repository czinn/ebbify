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
    pub accounts: BTreeMap<u32, Account>,
    pub balances: BTreeMap<u32, Balance>,
    pub categories: BTreeMap<u32, Category>,
    pub currencies: BTreeMap<u32, Currency>,
    pub flows: BTreeMap<u32, Flow>,
    pub transactions: BTreeMap<u32, Transaction>,
}

impl FileData {
    pub fn sample_data() -> Self {
        let data = r#"
        {
            "accounts":{},
            "balances":{},
            "categories":{
                "0": {"id": 0, "name": "Food", "parent_id": null},
                "1": {"id": 1, "name": "Groceries", "parent_id": 0},
                "2": {"id": 2, "name": "Restaurants", "parent_id": 0},
                "3": {"id": 3, "name": "Transit", "parent_id": null}
            },
            "currencies":{},
            "flows":{},
            "transactions":{}
        }"#;

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

#[derive(Default)]
pub struct AppData {
    pub data: FileData,
    pub category_trees: Vec<CategoryNode>,
}

impl AppData {
    pub fn from_file(data: FileData) -> Self {
        let mut t = Self {
            data,
            category_trees: Vec::new(),
        };
        t.recompute_category_trees();
        t
    }

    pub fn recompute_category_trees(&mut self) {
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
}
