use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct QueryPage {
    #[serde(default = "QueryPage::default_page_no")]
    pub page_no: i64,
    #[serde(default = "QueryPage::default_page_size")]
    pub page_size: i64,
    #[serde(default = "QueryPage::default_order_by")]
    pub order_by: String,
    #[serde(default = "QueryPage::default_asc")]
    pub asc: bool,
}

impl QueryPage {
    fn default_page_no() -> i64 {
        1
    }

    fn default_page_size() -> i64 {
        30
    }

    fn default_order_by() -> String {
        "".to_string()
    }

    fn default_asc() -> bool {
        true
    }

    pub fn check(&mut self, order_by: (&str, &[&str])) -> Result<(), &str> {
        if self.page_no == 0 {
            self.page_no = Self::default_page_no();
        }

        if self.page_size > 50 {
            self.page_size = 50;
        } else if self.page_size == 0 {
            self.page_size = Self::default_page_size();
        }

        if self.order_by.is_empty() {
            self.order_by = order_by.0.to_string();
            return Ok(());
        }

        if !order_by.1.contains(&self.order_by.as_str()) {
            return Err("invalid order_by");
        }

        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct QueryResult<T> {
    pub total: i64,
    pub items: Vec<T>,
}

impl<T> QueryResult<T> {
    pub fn new() -> Self {
        Self { total: 0, items: Vec::new() }
    }
}

impl<T> Default for QueryResult<T> {
    fn default() -> Self {
        Self::new()
    }
}
