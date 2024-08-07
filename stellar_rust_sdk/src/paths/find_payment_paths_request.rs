use crate::BuildQueryParametersExt;
use crate::models::Request;
use crate::paths::AssetType;

#[derive(Default, Clone)]
pub struct NoDestinationAssetType;

#[derive(Default, Clone, Debug)]
pub struct SourceAccount(String);

#[derive(Default, Clone)]
pub struct NoSourceAccount;

#[derive(Default)]
pub struct FindPaymentsPathRequest<S, T, A> {
    source_account: S,
    source_assets: String,
    destination_asset: T,
    destination_amount: A,
}

#[derive(Default, Clone, Debug)]
pub struct DestinationAmount(String);
#[derive(Default, Clone, Debug)]
pub struct NoDestinationAmount;

impl FindPaymentsPathRequest<NoSourceAccount, NoDestinationAssetType, NoDestinationAmount> {
    pub fn new() -> Self {
        FindPaymentsPathRequest::default()
    }

    pub fn set_source_account(
        self,
        source_account: SourceAccount,
    ) -> Result<FindPaymentsPathRequest<SourceAccount, NoDestinationAssetType, NoDestinationAmount>, String> {
        Ok(FindPaymentsPathRequest {
            source_account,
            ..Default::default()
        })
    }

    pub fn set_destination_asset_type(
        self,
        destination_asset_type: AssetType,
    ) -> Result<FindPaymentsPathRequest<NoSourceAccount, AssetType, NoDestinationAmount>, String> {
        Ok(FindPaymentsPathRequest {
            destination_asset: destination_asset_type,
            ..Default::default()
        })
    }

    pub fn set_destination_amount(
        self,
        destination_amount: DestinationAmount,
    ) -> Result<FindPaymentsPathRequest<NoSourceAccount, NoDestinationAssetType, DestinationAmount>, String> {
        Ok(FindPaymentsPathRequest {
            destination_amount,
            ..Default::default()
        })
    }
}

impl Request for FindPaymentsPathRequest<SourceAccount, AssetType, DestinationAmount> {
    fn get_query_parameters(&self) -> String {
        vec![
            Some(format!("source_account={:?}", self.source_account.0)),
            Some(format!("destination_asset_type={:?}", self.destination_asset)),
            Some(format!("destination_amount={:?}", self.destination_amount)),
        ]
            .build_query_parameters()
    }

    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}/{}/{}",
            base_url,
            super::PATHS_STRICT_RECEIVE_PAYMENT_PATH,
            self.get_query_parameters()
        )
    }
}