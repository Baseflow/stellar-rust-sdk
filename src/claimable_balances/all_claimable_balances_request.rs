use crate::{BuildQueryParametersExt, models::*};

use super::super::Order;
use super::super::AssetType;

/// AllClaimableBalancesRequest is the request type for the /claimable_balances/all endpoint
/// [More Details] (https://www.stellar.org/developers/horizon/reference/endpoints/claimable_balances-all.html) "All Claimable Balances")
pub struct AllClaimableBalancesRequest {
    /// Account ID of the sponsor. Every account in the response will either be sponsored by the
    /// given account ID or have a subentry (trustline, offer, or data entry) which is sponsored by
    /// the given account ID.
    sponsor: Option<String>,
    /// Account ID of the signer. Every account in the response will have the given account ID as a
    /// signer.
    asset: Option<AssetType>,
    /// An object that holds both the destination account that can claim the ClaimableBalanceEntry 
    /// and a ClaimPredicate that must evaluate to true for the claim to succeed.
    claimant: Option<String>,
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
}

impl Request for AllClaimableBalancesRequest {
    fn new() -> Self {
        AllClaimableBalancesRequest {
            sponsor: None,
            asset: None,
            claimant: None,
            cursor: None,
            limit: None,
            order: None,
        }
    }

    fn get_query_parameters(&self) -> String {
        vec![
            self.sponsor.as_ref().map(|s| format!("sponsor={}", s)),
            self.asset.as_ref().map(|a| format!("asset={}", a)),
            self.claimant.as_ref().map(|c| format!("claimant={}", c)),
            self.cursor.as_ref().map(|c| format!("cursor={}", c)),
            self.limit.as_ref().map(|l| format!("limit={}", l)),
            self.order.as_ref().map(|o| format!("order={}", o)),
        ].build_query_parameters()
    }

    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}/{}/{}",
            base_url,
            super::CLAIMABLE_BALANCES_PATH,            
            self.get_query_parameters()
        )
    }

    fn validate(&self) -> Result<(), String> {
        if let Some(sponsor) = &self.sponsor {
            let is_valid = is_public_key(sponsor);
            if is_valid.is_err() {
                return Err(is_valid.unwrap_err());
            }
        }

        if let Some(claimant) = &self.claimant {
            let is_valid = is_public_key(claimant);
            if is_valid.is_err() {
                return Err(is_valid.unwrap_err());
            }
        }

        Ok(())
    }
}

impl AllClaimableBalancesRequest {
    /// Sets the sponsor for the request
    /// # Arguments
    /// * `self` - The request object
    /// * `sponsor` - The sponsor for the request
    /// # Returns
    /// The request object
    /// [AllClaimableBalancesRequest](struct.AllClaimableBalancesRequest.html)
    pub fn set_sponsor(&mut self, sponsor: String) -> &mut Self {
        self.sponsor = Some(sponsor);
        self
    }

    /// Sets the asset for the request
    /// # Arguments
    /// * `self` - The request object
    /// * `asset` - The asset for the request
    /// # Returns
    /// The request object
    /// [AllClaimableBalancesRequest](struct.AllClaimableBalancesRequest.html)
    pub fn set_asset(&mut self, asset: AssetType) -> &mut Self {
        self.asset = Some(asset);
        self
    }

    /// Sets the claimant for the request
    /// # Arguments
    /// * `self` - The request object
    /// * `claimant` - The claimant for the request
    /// # Returns
    /// The request object
    /// [AllClaimableBalancesRequest](struct.AllClaimableBalancesRequest.html)
    pub fn set_claimant(&mut self, claimant: String) -> &mut Self {
        self.claimant = Some(claimant);
        self
    }

    /// Sets the cursor for the request
    /// # Arguments
    /// * `self` - The request object
    /// * `cursor` - The cursor for the request
    /// # Returns
    /// The request object
    /// [AllClaimableBalancesRequest](struct.AllClaimableBalancesRequest.html)
    pub fn set_cursor(&mut self, cursor: u32) -> &mut Self {
        self.cursor = Some(cursor);
        self
    }

    /// Sets the limit for the request
    /// # Arguments
    /// * `self` - The request object
    /// * `limit` - The limit for the request
    /// # Returns
    /// The request object
    /// [AllClaimableBalancesRequest](struct.AllClaimableBalancesRequest.html)
    pub fn set_limit(&mut self, limit: u32) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    /// Sets the order for the request
    /// # Arguments
    /// * `self` - The request object
    /// * `order` - The order for the request
    /// # Returns
    /// The request object
    /// [AllClaimableBalancesRequest](struct.AllClaimableBalancesRequest.html)
    pub fn set_order(&mut self, order: Order) -> &mut Self {
        self.order = Some(order);
        self
    }
}
