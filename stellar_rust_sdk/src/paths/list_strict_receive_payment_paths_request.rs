use crate::BuildQueryParametersExt;
use crate::models::Request;
use crate::paths::{AssetType, IssuedOrNative};

#[derive(Default, Clone, Debug)]
pub struct SourceAccount(String);
#[derive(Default, Clone, Debug)]
pub struct NoSourceAccount;
#[derive(Default, Clone, Debug)]
pub struct SourceAssets(Vec<IssuedOrNative>);

#[derive(Default, Clone)]
pub struct NoSourceAssets;

#[derive(Default, Clone, Debug)]
pub struct DestinationAmount(String);

#[derive(Default, Clone)]
pub struct NoDestinationAmount;

#[derive(Debug)]
pub struct DestinationAsset(AssetType);
pub struct NoDestinationAsset;

#[derive(Default, Clone)]
pub struct ListStrictReceivePaymentPathRequest<SAc, SAs, DAm, DAs> {
    source_account: SAc,
    source_assets: SAs,
    destination_amount: DAm,
    destination_asset_type: DAs,
}

impl ListStrictReceivePaymentPathRequest<NoSourceAccount, NoSourceAssets, NoDestinationAmount, NoDestinationAsset> {
    pub fn new() -> Self {
        ListStrictReceivePaymentPathRequest::default()
    }

    pub fn set_source_account(
        self,
        source_account: SourceAccount,
    ) -> Result<ListStrictReceivePaymentPathRequest<SourceAccount, NoSourceAssets, NoDestinationAmount, NoDestinationAsset>, String> {
        Ok(ListStrictReceivePaymentPathRequest {
            source_account,
            ..Default::default()
        })
    }

    pub fn set_source_assets(
        self,
        source_assets: SourceAssets,
    ) -> Result<ListStrictReceivePaymentPathRequest<NoSourceAccount, SourceAssets, NoDestinationAmount, NoDestinationAsset>, String> {
        Ok(ListStrictReceivePaymentPathRequest {
            source_assets,
            ..Default::default()
        })
    }

    pub fn set_destination_amount(
        self,
        destination_amount: DestinationAmount,
    ) -> Result<ListStrictReceivePaymentPathRequest<NoSourceAccount, NoSourceAssets, DestinationAmount, NoDestinationAsset>, String> {
        Ok(ListStrictReceivePaymentPathRequest {
            destination_amount,
            ..Default::default()
        })
    }

    pub fn set_destination_asset_type(
        self,
        destination_asset_type: DestinationAsset,
    ) -> Result<ListStrictReceivePaymentPathRequest<NoSourceAccount, NoSourceAssets, NoDestinationAmount, DestinationAsset>, String> {
        Ok(ListStrictReceivePaymentPathRequest {
            destination_asset_type,
            ..Default::default()
        })
    }
}

impl Request for ListStrictReceivePaymentPathRequest<NoSourceAccount, SourceAssets, DestinationAmount, DestinationAsset> {
    fn get_query_parameters(&self) -> String {
        vec![
            Some(format!("source_assets={:?}", self.source_assets)),
            Some(format!("destination_amount={:?}", self.destination_amount)),
            Some(format!("destination_asset_type={:?}", self.destination_asset_type)),
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

impl Request for ListStrictReceivePaymentPathRequest<SourceAccount, NoSourceAssets, DestinationAmount, NoDestinationAsset> {
    fn get_query_parameters(&self) -> String {
        vec![
            Some(format!("source_account={:?}", self.source_account.0)),
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