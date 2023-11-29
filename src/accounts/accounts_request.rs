use crate::{BuildQueryParametersExt, models::*, AssetType, Order};

// region: States
#[derive(Default, Clone)]
pub struct Sponsor(String);
#[derive(Default, Clone)]
pub struct NoSponsor;

#[derive(Default, Clone)]
pub struct Signer(String);
#[derive(Default, Clone)]
pub struct NoSigner;

#[derive(Clone)]
pub struct Asset(AssetType);
#[derive(Default, Clone)]
pub struct NoAsset;

#[derive(Default, Clone)]
pub struct LiquidityPool(String);
#[derive(Default, Clone)]
pub struct NoLiquidityPool;
// endregion

pub trait ValidAccountsRequest: Request {}
impl ValidAccountsRequest for AccountsRequest<Sponsor, NoSigner, NoAsset, NoLiquidityPool> {}
impl ValidAccountsRequest for AccountsRequest<NoSponsor, Signer, NoAsset, NoLiquidityPool> {}
impl ValidAccountsRequest for AccountsRequest<NoSponsor, NoSigner, Asset, NoLiquidityPool> {}
impl ValidAccountsRequest for AccountsRequest<NoSponsor, NoSigner, NoAsset, LiquidityPool> {}

/// AccountsRequest is the request object for the /accounts endpoint
/// [More Details](https://www.stellar.org/developers/horizon/reference/endpoints/accounts.html "Accounts")
#[derive(Default)]
pub struct AccountsRequest<Sp, Si, A, L> {
    /// Account ID of the sponsor. Every account in the response will either be sponsored by the
    /// given account ID or have a subentry (trustline, offer, or data entry) which is sponsored by
    /// the given account ID.
    sponsor: Sp,
    /// Account ID of the signer. Every account in the response will have the given account ID as a
    /// signer.
    signer: Si,
    /// An issued asset represented as “Code:IssuerAccountID”. Every account in the response will
    /// have a trustline for the given asset.
    asset: A,
    /// The liquidity pool ID. Every account in the response will have a trustline for the given
    liquidity_pool: L,

    /// the cursor
    cursor: Option<u32>,
    /// The maximum number of records returned. The limit can range from 1 to 200 - an upper limit
    /// that is hardcoded in Horizon for performance reasons. If this argument isn’t designated, it
    /// defaults to 10.
    limit: Option<u32>,
    /// A designation of the order in which records should appear. Options include asc (ascending)
    /// or desc (descending). If this argument isn’t set, it defaults to asc.
    order: Option<Order>,

}

impl<Sp, Si, A, L> AccountsRequest<Sp, Si, A, L> {
    /// Sets the cursor
    /// # Arguments
    /// * `cursor` - The cursor
    /// # Returns
    /// The request object
    /// [AllAccountsRequest](struct.AllAccountsRequest.html)
    pub fn set_cursor(self, cursor: u32) -> Result<AccountsRequest<Sp, Si, A, L>, String>{
        if cursor < 1 {
            return Err("cursor must be greater than or equal to 1".to_string());
        }

        Ok(
            AccountsRequest {
                cursor: Some(cursor),
                ..self
            }
        )
    }

    /// Sets the limit
    /// # Arguments
    /// * `limit` - The limit
    /// # Returns
    /// The request object
    /// [AllAssetsRequest](struct.AllAssetsRequest.html)
    pub fn set_limit(self, limit: u32) -> Result<AccountsRequest<Sp, Si, A, L>, String> {
        if limit < 1 || limit > 200 {
            return Err("limit must be between 1 and 200".to_string());
        }

        Ok(
            AccountsRequest {
                limit: Some(limit),
                ..self
            }
        )
    }

    /// Sets the order
    /// # Arguments
    /// * `order` - The order
    /// # Returns
    /// The request object
    /// [AllAssetsRequest](struct.AllAssetsRequest.html)
    pub fn set_order(self, order: Order) -> AccountsRequest<Sp, Si, A, L> {
        AccountsRequest {
            order: Some(order),
            ..self
        }
    }
}

impl AccountsRequest<NoSponsor, NoSigner, NoAsset, NoLiquidityPool> {
    pub fn new() -> Self {
        AccountsRequest::default()
    }
    
    /// Sets the public key of the sponsor
    /// # Arguments
    /// * `sponsor` - The public key of the sponsor
    /// # Returns
    /// The request object
    /// [AccountsRequest](struct.AccountsRequest.html)
    pub fn set_sponsor(self, sponsor: String) -> Result<AccountsRequest<Sponsor, NoSigner, NoAsset, NoLiquidityPool>, String> {
        if let Err(e) = is_public_key(&sponsor) {
            return Err(e.to_string());
        }

        Ok(AccountsRequest {
            sponsor: Sponsor(sponsor.into()),
            signer: self.signer,
            asset: self.asset,
            liquidity_pool: self.liquidity_pool,
            cursor: self.cursor,
            limit: self.limit,
            order: self.order,
        })
    }

    /// Sets the public key of the signer
    /// # Arguments
    /// * `signer` - The public key of the signer
    /// # Returns
    /// The request object
    /// [AccountsRequest](struct.AccountsRequest.html)
    pub fn set_signer(self, signer: &str) -> Result<AccountsRequest<NoSponsor, Signer, NoAsset, NoLiquidityPool>, String> {
        if let Err(e) = is_public_key(&signer) {
            return Err(e.to_string());
        }

        Ok(AccountsRequest {
            sponsor: self.sponsor,
            signer: Signer(signer.to_string()),
            asset: self.asset,
            liquidity_pool: self.liquidity_pool,
            cursor: self.cursor,
            limit: self.limit,
            order: self.order,
        })
    }

    /// Sets the asset type
    /// # Arguments
    /// * `asset` - The asset type
    /// # Returns
    /// [AccountsRequest](struct.AccountsRequest.html)
    pub fn set_asset(self, asset: AssetType) -> AccountsRequest<NoSponsor, NoSigner, Asset, NoLiquidityPool> {
        AccountsRequest {
            sponsor: self.sponsor,
            signer: self.signer,
            asset: Asset(asset),
            liquidity_pool: self.liquidity_pool,
            cursor: self.cursor,
            limit: self.limit,
            order: self.order,
        }
    }

    /// Sets the liquidity pool
    /// # Arguments
    /// * `liquidity_pool` - The liquidity pool
    /// # Returns
    /// The request object
    /// [AccountsRequest](struct.AccountsRequest.html)
    pub fn set_liquidity_pool(self, liquidity_pool: impl Into<String>) -> AccountsRequest<NoSponsor, NoSigner, NoAsset, LiquidityPool> {
        AccountsRequest {
            sponsor: self.sponsor,
            signer: self.signer,
            asset: self.asset,
            liquidity_pool: LiquidityPool(liquidity_pool.into()),
            cursor: self.cursor,
            limit: self.limit,
            order: self.order,
        }
    }
}

impl Request for AccountsRequest<Sponsor, NoSigner, NoAsset, NoLiquidityPool> {
    fn get_query_parameters(&self) -> String {
        vec![
            self.cursor.as_ref().map(|c| format!("cursor={}", c)),
            self.limit.as_ref().map(|l| format!("limit={}", l)),
            self.order.as_ref().map(|o| format!("order={}", o)),
            Some(format!("sponsor={}", self.sponsor.0))
        ].build_query_parameters()
    }

    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}/{}{}",
            base_url,
            super::ACCOUNTS_PATH,
            self.get_query_parameters()
        )
    }
}

impl Request for AccountsRequest<NoSponsor, Signer, NoAsset, NoLiquidityPool> {
    fn get_query_parameters(&self) -> String {
        vec![
            self.cursor.as_ref().map(|c| format!("cursor={}", c)),
            self.limit.as_ref().map(|l| format!("limit={}", l)),
            self.order.as_ref().map(|o| format!("order={}", o)),
            Some(format!("signer={}", self.signer.0))
        ].build_query_parameters()
    }

    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}/{}{}",
            base_url,
            super::ACCOUNTS_PATH,
            self.get_query_parameters()
        )
    }
}

impl Request for AccountsRequest<NoSponsor, NoSigner, Asset, NoLiquidityPool> {
    fn get_query_parameters(&self) -> String {
        vec![
            self.cursor.as_ref().map(|c| format!("cursor={}", c)),
            self.limit.as_ref().map(|l| format!("limit={}", l)),
            self.order.as_ref().map(|o| format!("order={}", o)),
            Some(format!("asset={}", self.asset.0))
        ].build_query_parameters()
    }

    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}/{}{}",
            base_url,
            super::ACCOUNTS_PATH,
            self.get_query_parameters()
        )
    }
}

impl Request for AccountsRequest<NoSponsor, NoSigner, NoAsset, LiquidityPool> {
    fn get_query_parameters(&self) -> String {
        vec![
            self.cursor.as_ref().map(|c| format!("cursor={}", c)),
            self.limit.as_ref().map(|l| format!("limit={}", l)),
            self.order.as_ref().map(|o| format!("order={}", o)),
            Some(format!("liquidity_pool={}", self.liquidity_pool.0))
        ].build_query_parameters()
    }

    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}/{}{}",
            base_url,
            super::ACCOUNTS_PATH,
            self.get_query_parameters()
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_accounts_request() {
    //     let request = AccountsRequest::new()
    //         .set_sponsor("GDQJUTQYK2MQX2VGDR2FYWLIYAQIEGXTQVTFEMGH2BEWFG4BRUY4CKI7".to_string()).unwrap();
    //     assert_eq!(request.get_path(), "/accounts");
    // }

    #[test]
    fn test_accounts_request_set_sponsor() {
        let request = AccountsRequest::new()
            .set_sponsor("sponsor".to_string());

        assert!(request.is_err());
        // assert_eq!(request.unwrap_err(), "Public key must be 56 characters long");
    }
}
