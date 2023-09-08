use crate::models::*;

/// The asset type
/// Native - The native asset
/// Issued - An issued asset
/// [AccountsRequest](struct.AccountsRequest.html)
pub enum AssetType {
    Native,
    Issued,
}

impl std::fmt::Display for AssetType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AssetType::Native => write!(f, "native"),
            AssetType::Issued => write!(f, "issued"),
        }
    }
}

/// The order of the records
/// Asc - Ascending order
/// Desc - Descending order
/// [AccountsRequest](struct.AccountsRequest.html)
pub enum Order {
    Asc,
    Desc,
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Order::Asc => write!(f, "asc"),
            Order::Desc => write!(f, "desc"),
        }
    }
}

/// AccountsRequest is the request object for the /accounts endpoint
/// [More Details](https://www.stellar.org/developers/horizon/reference/endpoints/accounts.html "Accounts")
pub struct AccountsRequest {
    /// Account ID of the sponsor. Every account in the response will either be sponsored by the
    /// given account ID or have a subentry (trustline, offer, or data entry) which is sponsored by
    /// the given account ID.
    sponsor: Option<String>,
    /// Account ID of the signer. Every account in the response will have the given account ID as a
    /// signer.
    signer: Option<String>,
    /// An issued asset represented as “Code:IssuerAccountID”. Every account in the response will
    /// have a trustline for the given asset.
    asset: Option<AssetType>,
    /// Account ID of the signer. Every account in the response will have the given account ID as a
    /// signer.
    cursor: Option<u32>,
    /// The maximum number of records returned. The limit can range from 1 to 200 - an upper limit
    /// that is hardcoded in Horizon for performance reasons. If this argument isn’t designated, it
    /// defaults to 10.
    limit: Option<u32>,
    /// A designation of the order in which records should appear. Options include asc (ascending)
    /// or desc (descending). If this argument isn’t set, it defaults to asc.
    order: Option<Order>,
    /// The liquidity pool ID. Every account in the response will have a trustline for the given
    liquidity_pool: Option<String>,
}

impl Request for AccountsRequest {
    /// Creates a new request object
    /// # Returns
    /// A new request object
    /// [AccountsRequest](struct.AccountsRequest.html)
    fn new() -> Self {
        AccountsRequest {
            sponsor: None,
            signer: None,
            asset: None,
            cursor: None,
            limit: None,
            order: None,
            liquidity_pool: None,
        }
    }

    /// Gets the relative URL for the request
    fn get_path(&self) -> &str {
        "/accounts"
    }

    // Gets the query parameters for the request
    fn get_query_parameters(&self) -> String {
        let mut query = String::new();
        if let Some(sponsor) = &self.sponsor {
            query.push_str(&format!("sponsor={}&", sponsor));
        }
        if let Some(signer) = &self.signer {
            query.push_str(&format!("signer={}&", signer));
        }
        if let Some(asset) = &self.asset {
            query.push_str(&format!("asset={}&", asset));
        }
        if let Some(cursor) = &self.cursor {
            query.push_str(&format!("cursor={}&", cursor));
        }
        if let Some(limit) = &self.limit {
            query.push_str(&format!("limit={}&", limit));
        }
        if let Some(order) = &self.order {
            query.push_str(&format!("order={}&", order));
        }
        if let Some(liquidity_pool) = &self.liquidity_pool {
            query.push_str(&format!("liquidity_pool={}&", liquidity_pool));
        }
        query.trim_end_matches('&').to_string()
    }

    // Build the URL for the request
    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}{}?{}",
            base_url,
            self.get_path(),
            self.get_query_parameters()
        )
    }

    // Gets the body parameters for the request
    fn validate(&self) -> Result<(), String> {
        if let Some(sponsor) = &self.sponsor {
            let is_valid = is_public_key(sponsor);
            if is_valid.is_err() {
                return Err(is_valid.unwrap_err());
            }
        }

        if let Some(signer) = &self.signer {
            let is_valid = is_public_key(signer);
            if is_valid.is_err() {
                return Err(is_valid.unwrap_err());
            }
        }

        if let Some(cursor) = &self.cursor {
            if *cursor < 1 {
                return Err("cursor must be greater than or equal to 1".to_string());
            }
        }

        if let Some(limit) = &self.limit {
            if *limit < 1 {
                return Err("limit must be greater than or equal to 1".to_string());
            }
            if *limit > 200 {
                return Err("limit must be less than or equal to 200".to_string());
            }
        }

        if self.signer.is_none() && self.sponsor.is_none() && self.asset.is_none() {
            return Err("Exactly one filter is required. Please ensure that you are including a signer, sponsor, asset, or liquidity pool filter.".to_string());
        }

        Ok(())
    }
}

impl AccountsRequest {
    /// Sets the public key of the sponsor
    /// # Arguments
    /// * `sponsor` - The public key of the sponsor
    /// # Returns
    /// The request object
    /// [AccountsRequest](struct.AccountsRequest.html)
    pub fn set_sponsor(&mut self, sponsor: &str) -> &mut Self {
        self.sponsor = Some(sponsor.to_owned());
        self
    }

    /// Sets the public key of the signer
    /// # Arguments
    /// * `signer` - The public key of the signer
    /// # Returns
    /// The request object
    /// [AccountsRequest](struct.AccountsRequest.html)
    pub fn set_signer(&mut self, signer: &str) -> &mut Self {
        self.signer = Some(signer.to_owned());
        self
    }

    /// Sets the asset type
    /// # Arguments
    /// * `asset` - The asset type
    /// # Returns
    /// [AccountsRequest](struct.AccountsRequest.html)
    pub fn set_asset(&mut self, asset: AssetType) -> &mut Self {
        self.asset = Some(asset);
        self
    }

    /// Sets the cursor for the page
    /// # Arguments
    /// * `cursor` - The cursor for the page
    /// # Returns
    /// The request object
    /// [AccountsRequest](struct.AccountsRequest.html)
    pub fn set_cursor(&mut self, cursor: u32) -> &mut Self {
        self.cursor = Some(cursor);
        self
    }

    /// Sets the maximum number of records to return
    /// # Arguments
    /// * `limit` - The maximum number of records to return
    /// # Returns
    /// The request object
    /// [AccountsRequest](struct.AccountsRequest.html)
    pub fn set_limit(&mut self, limit: u32) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    /// Sets the order of the records
    /// # Arguments
    /// * `order` - The order of the records
    /// # Returns
    /// The request object
    /// [AccountsRequest](struct.AccountsRequest.html)
    pub fn set_order(&mut self, order: Order) -> &mut Self {
        self.order = Some(order);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accounts_request() {
        let request = AccountsRequest::new();
        assert_eq!(request.get_path(), "/accounts");
    }

    #[test]
    fn test_accounts_request_set_sponsor() {
        let mut request = AccountsRequest::new();
        request.set_sponsor("sponsor");
        assert_eq!(request.sponsor, Some("sponsor".to_string()));
        // check that the validate throws an error
        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Public key must be 56 characters long");
    }
}
