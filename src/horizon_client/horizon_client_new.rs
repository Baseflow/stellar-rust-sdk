use derive_getters::Getters;
use url::Url;

use crate::{models::{Response, Request}, ledgers::single_ledger_response::SingleLedgerResponse};

pub trait RequestNew {
    fn build_url(&self, base_url: &BaseUrl) -> String;
}

// pub trait RequestBuilder {
//     fn build(self) -> Result<impl RequestNew, String>;
// }


#[derive(Clone)]
pub enum HttpMethod {
    GET,
    POST
}

// --------------------------------SINGLE REQUEST--------------------------------
const SINGLE_LEDGERS_PATH: &str = "/ledgers";

#[derive(Getters)]
pub struct SingleLedgerRequest {
    sequence: u32,
    path: String,
    method: HttpMethod
}

impl RequestNew for SingleLedgerRequest {
    fn build_url(&self, base_url: &BaseUrl) -> String {
        format!(
            "{}{}/{}",
            base_url.0,
            self.path(),
            self.sequence()
        )
    }
}

// region: --- States
#[derive(Default, Clone)]
pub struct Sequence(u32);
#[derive(Default, Clone)]
pub struct NoSequence;
// endregion: --- States

#[derive(Clone)]
pub struct SingleLedgerRequestBuilder<S> {
    sequence: S,
    path: String,
    method: HttpMethod,
}

impl Default for SingleLedgerRequestBuilder<NoSequence> {
    fn default() -> Self {
        SingleLedgerRequestBuilder { 
            sequence: NoSequence,
            path: SINGLE_LEDGERS_PATH.into(),
            method: HttpMethod::GET
        }
    }
}

impl SingleLedgerRequestBuilder<NoSequence> {
    pub fn new() -> Self {
        SingleLedgerRequestBuilder::default()
    }
}

impl<S> SingleLedgerRequestBuilder<S> {
    pub fn sequence(
        self,
        sequence: impl Into<u32>,
    ) -> SingleLedgerRequestBuilder<Sequence> {

        SingleLedgerRequestBuilder {    
            sequence: Sequence(sequence.into()),
            path: SINGLE_LEDGERS_PATH.into(),
            method: HttpMethod::GET
        }
    }
}

impl SingleLedgerRequestBuilder<Sequence> {
    pub fn build(self) -> Result<SingleLedgerRequest, String> {
        Ok(SingleLedgerRequest { 
            sequence: self.sequence.0,
            path: self.path,
            method: self.method
        })
    }
}
// --------------------------------------------------------------------------------



// --------------------------------HORIZON CLIENT--------------------------------
#[derive(Default, Clone)]
pub struct NoBaseUrl;
#[derive(Default, Clone)]
pub struct BaseUrl(String);

#[derive(Default, Clone)]
pub struct HorizonClientNew<U> {
    /// The base URL for the Horizon server
    base_url: U,
}

impl HorizonClientNew<NoBaseUrl> {
    pub fn new () -> Self {
        HorizonClientNew::default()
    }

    pub fn base_url(
        self,
        base_url: impl Into<String>
    ) -> Result<HorizonClientNew<BaseUrl>, String> {
        let url: String = base_url.into();

        url_validate(url.clone())?;

        Ok(HorizonClientNew { base_url: BaseUrl(url) })
    }
}

impl HorizonClientNew<BaseUrl> {
    /// Gets a single ledger from the server
    /// # Arguments
    /// * `self` - The Horizon client
    /// * request - The single ledger request
    /// # Returns
    /// The single ledger response
    /// # Errors
    /// Returns an error if the request fails
    /// [GET /ledgers/{ledger_id}](https://www.stellar.org/developers/horizon/reference/endpoints/ledgers-single.html)
    pub async fn get_single_ledger(
        &self,
        request: &SingleLedgerRequest,
    ) -> Result<SingleLedgerResponse, String> {
        self.get::<SingleLedgerResponse>(request).await
    }

    async fn get<TResponse: Response + std::fmt::Debug>(
        &self,
        request: &impl RequestNew
    ) -> Result<TResponse, String> {
        let url = request.build_url(&self.base_url);
        let response = reqwest::get(&url).await.map_err(|e| e.to_string())?;
        // println!("\n\nREQWEST RESPONSE: {:?}", response);
        let result: TResponse = handle_response(response).await?;

        Ok(result)
    }
}

async fn handle_response<TResponse: Response>(
    response: reqwest::Response,
) -> Result<TResponse, String> {
    // println!("\n Response: {:?}", response);
    match response.status() {
        reqwest::StatusCode::OK => {
            let _response = response.text().await.map_err(|e| e.to_string())?;
            TResponse::from_json(_response)
        }
        _ => {
            let response = response.text().await.map_err(|e| e.to_string())?;
            Err(response)
        }
    }
}

fn url_validate(url_to_validate: impl Into<String>) -> Result<(), String> {
    // check if starts with http:// or https://
    let url = url_to_validate.into();
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(format!("URL must start with http:// or https://: {}", url));
    }
    Url::parse(&url).map_err(|e| e.to_string())?;

    Ok(())
}
// --------------------------------------------------------------------------------------



#[cfg(test)]
mod tests {
    use base64::encode;

    use crate::ledgers::single_ledger_response::SingleLedgerResponse;

    use super::*;

    #[test]
    fn test_horizon_client() {
        let horizon_client = HorizonClientNew::new()
                .base_url("https://horizon-testnet.stellar.org");
    }

    #[test]
    fn test_ledgers_request() {
        let request = SingleLedgerRequestBuilder::new()
            .sequence(2 as u32)
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_single_ledger() {
        let horizon_client = HorizonClientNew::new()
            .base_url("https://horizon-testnet.stellar.org")
            .unwrap();

        let request = SingleLedgerRequestBuilder::new()
            .sequence(2 as u32)
            .build()
            .unwrap();
        
        let _single_ledger_response = horizon_client.get_single_ledger(&request).await;

        assert!(_single_ledger_response.clone().is_ok());

        assert_eq!(
            _single_ledger_response.clone().unwrap().id(),
            "eca856e0073dc2087249dc929ed31c09c3babfef2e687b685d0513dbe6489a18"
        );

        assert_eq!(
            _single_ledger_response.clone().unwrap().paging_token(),
            "8589934592"
        );

        assert_eq!(
            _single_ledger_response.clone().unwrap().hash(),
            "eca856e0073dc2087249dc929ed31c09c3babfef2e687b685d0513dbe6489a18"
        );

        assert_eq!(
            _single_ledger_response.clone().unwrap().prev_hash(),
            "63d98f536ee68d1b27b5b89f23af5311b7569a24faf1403ad0b52b633b07be99"
        );

        assert_eq!(*_single_ledger_response.clone().unwrap().sequence(), 2);

        assert_eq!(
            *_single_ledger_response
                .clone()
                .unwrap()
                .successful_transaction_count(),
            0
        );

        assert_eq!(
            *_single_ledger_response
                .clone()
                .unwrap()
                .failed_transaction_count(),
            0
        );

        assert_eq!(
            *_single_ledger_response.clone().unwrap().operation_count(),
            0
        );

        assert_eq!(
            *_single_ledger_response
                .clone()
                .unwrap()
                .tx_set_operation_count(),
            0
        );

        assert_eq!(
            _single_ledger_response.clone().unwrap().closed_at(),
            "2023-06-14T09:19:48Z"
        );

        assert_eq!(
            _single_ledger_response.clone().unwrap().total_coins(),
            "100000000000.0000000"
        );

        assert_eq!(
            _single_ledger_response.clone().unwrap().fee_pool(),
            "0.0000000"
        );

        assert_eq!(
            *_single_ledger_response
                .clone()
                .unwrap()
                .base_fee_in_stroops(),
            100
        );

        assert_eq!(
            *_single_ledger_response
                .clone()
                .unwrap()
                .base_reserve_in_stroops(),
            100000000
        );

        assert_eq!(
            *_single_ledger_response.clone().unwrap().max_tx_set_size(),
            100
        );

        assert_eq!(
            *_single_ledger_response.clone().unwrap().protocol_version(),
            0
        );

        let decoded_xdr_header = _single_ledger_response
            .unwrap()
            .decoded_header_xdr()
            .unwrap();

        assert_eq!(
            decoded_xdr_header.bucket_list_hash.to_string(),
            "735227ed398461291237687b08446aa2c9b096e0c98a462dadda569f05dd2484"
        );

        assert_eq!(decoded_xdr_header.ledger_seq, 2);

        assert_eq!(decoded_xdr_header.total_coins, 1000000000000000000);

        assert_eq!(decoded_xdr_header.fee_pool, 0);

        assert_eq!(decoded_xdr_header.inflation_seq, 0);

        assert_eq!(decoded_xdr_header.id_pool, 0);

        assert_eq!(decoded_xdr_header.base_fee, 100);

        assert_eq!(decoded_xdr_header.base_reserve, 100000000);

        assert_eq!(decoded_xdr_header.max_tx_set_size, 100);

        let tx_set_hash = decoded_xdr_header.scp_value.tx_set_hash.to_string();
        let tx_set_hash_bytes = hex::decode(tx_set_hash).expect("Failed to decode hex");
        let tx_set_hash_base64 = encode(&tx_set_hash_bytes);

        assert_eq!(
            tx_set_hash_base64,
            "uZRHr9UdXKbTKiclfOjy72YZFJUkJPVcKT5htvorm1Q="
        );

        assert_eq!(
            decoded_xdr_header.scp_value.close_time,
            stellar_xdr::TimePoint(1686734388)
        );
    }
}