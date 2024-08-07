use crate::BuildQueryParametersExt;
use crate::models::Request;
use crate::paths::{AssetType, IssuedOrNative};

#[derive(Default, Clone, Debug)]
pub struct SourceAmount(String);
#[derive(Default, Clone, Debug)]
pub struct NoSourceAmount;
#[derive(Default, Clone, Debug)]
pub struct SourceAssets(Vec<IssuedOrNative>);
#[derive(Default, Clone, Debug)]
pub struct NoSourceAsset;
#[derive(Default, Clone, Debug)]
pub struct DestinationAmount(String);
#[derive(Default, Clone, Debug)]
pub struct NoDestinationAmount;
#[derive(Default, Clone, Debug)]
pub struct DestinationAsset(AssetType);
#[derive(Default, Clone, Debug)]
pub struct NoDestinationAsset;

pub struct ListStrictSendPaymentPathsRequest<
    SAmount,
    SAsset,
    DAmount,
    DAsset, > {
    source_amount: SAmount,
    source_asset: SAsset,
    destination_amount: DAmount,
    destination_asset: DAsset,
}

impl ListStrictSendPaymentPathsRequest<NoSourceAmount, NoSourceAsset, NoDestinationAmount, NoDestinationAsset> {
    pub fn new() -> ListStrictSendPaymentPathsRequest<NoSourceAmount, NoSourceAsset, NoDestinationAmount, NoDestinationAsset> {
        ListStrictSendPaymentPathsRequest {
            ..Default::default()
        }
    }

    pub fn set_source_amount(
        self,
        source_amount: SourceAmount,
    ) -> ListStrictSendPaymentPathsRequest<SourceAmount, NoSourceAsset, NoDestinationAmount, NoDestinationAsset> {
        ListStrictSendPaymentPathsRequest {
            source_amount,
            ..Default::default()
        }
    }

    pub fn set_source_asset(
        self,
        source_asset: SourceAssets,
    ) -> ListStrictSendPaymentPathsRequest<NoSourceAmount, SourceAssets, NoDestinationAmount, NoDestinationAsset> {
        ListStrictSendPaymentPathsRequest {
            source_asset,
            ..Default::default()
        }
    }

    pub fn set_destination_amount(
        self,
        destination_amount: DestinationAmount,
    ) -> ListStrictSendPaymentPathsRequest<NoSourceAmount, NoSourceAsset, DestinationAmount, NoDestinationAsset> {
        ListStrictSendPaymentPathsRequest {
            destination_amount,
            ..Default::default()
        }
    }

    pub fn set_destination_asset(
        self,
        destination_asset: DestinationAsset,
    ) -> ListStrictSendPaymentPathsRequest<NoSourceAmount, NoSourceAsset, NoDestinationAmount, DestinationAsset> {
        ListStrictSendPaymentPathsRequest {
            destination_asset,
            ..Default::default()
        }
    }
}

impl Request for ListStrictSendPaymentPathsRequest<SourceAmount, SourceAssets, DestinationAmount, NoDestinationAsset> {
    fn get_query_parameters(&self) -> String {
        vec![
            Some(format!("source_amount={:?}", self.source_amount)),
            Some(format!("source_asset={:?}", self.source_asset)),
            Some(format!("destination_amount={:?}", self.destination_amount)),
        ]
            .build_query_parameters()
    }

    fn build_url(&self, base_url: &str) -> String {
        format!("{}/paths/strict-send?{}", base_url, self.get_query_parameters())
    }
}

impl Request for ListStrictSendPaymentPathsRequest<SourceAmount, SourceAssets, NoDestinationAmount, DestinationAsset> {
    fn get_query_parameters(&self) -> String {
        vec![
            Some(format!("source_amount={:?}", self.source_amount)),
            Some(format!("source_asset={:?}", self.source_asset)),
            Some(format!("destination_asset={:?}", self.destination_asset)),
        ]
            .build_query_parameters()
    }

    fn build_url(&self, base_url: &str) -> String {
        format!("{}/paths/strict-send?{}", base_url, self.get_query_parameters())
    }
}