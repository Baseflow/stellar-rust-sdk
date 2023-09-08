use crate::models::{is_public_key, Request};

pub struct SingleAccountRequest {
    account_id: Option<String>,
}

impl Request for SingleAccountRequest {
    fn new() -> Self {
        SingleAccountRequest { account_id: None }
    }

    fn get_path(&self) -> &str {
        "/accounts/"
    }

    fn get_query_parameters(&self) -> String {
        let mut query = String::new();
        if let Some(account_id) = &self.account_id {
            query.push_str(&format!("{}", account_id));
        }

        query.trim_end_matches('&').to_string()
    }

    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}{}{}",
            base_url,
            self.get_path(),
            self.get_query_parameters()
        )
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(account_id) = &self.account_id {
            let is_valid = is_public_key(account_id);
            if is_valid.is_err() {
                return Err(is_valid.unwrap_err());
            }
        }

        Ok(())
    }
}

impl SingleAccountRequest {
    pub fn set_account_id(&mut self, account_id: String) {
        self.account_id = Some(account_id);
    }
}
