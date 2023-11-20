use url::Url;

use crate::{models::Response, ledgers::single_ledger_response::SingleLedgerResponse};

pub trait Req {
    fn get_path(&self) -> &str;
    fn build_url(&self, base_url: &str) -> String;
}

// pub trait RequestBuilder {
//     fn build(self) -> Result<impl RequestNew, String>;
// }


// region: --- SingleLedgerRequest
#[derive(Default, Clone)]
pub struct Sequence(u32);
#[derive(Default, Clone)]
pub struct NoSequence;

#[derive(Default)]
pub struct SingleLedgerRequest<S> {
    sequence: S,
}

impl SingleLedgerRequest<NoSequence> {
    pub fn new() -> Self {
        SingleLedgerRequest::default()
    }
}

impl<S> SingleLedgerRequest<S> {
    pub fn set_sequence(
        self,
        sequence: u32,
    ) -> Result<SingleLedgerRequest<Sequence>, String> {
        if sequence < 1 {
            return Err("sequence must be greater than or equal to 1".to_string());
        }

        Ok(
            SingleLedgerRequest {    
                sequence: Sequence(sequence),
            }
        )
    }
}

impl SingleLedgerRequest<Sequence> {
    fn get_sequence(&self) -> String {
        self.sequence.0.to_string()
    }
}

impl Req for SingleLedgerRequest<Sequence> {
    fn get_path(&self) -> &str {
        "/ledgers"
    }

    fn build_url(&self, base_url: &str) -> String {
        format!(
            "{}{}/{}",
            base_url,
            self.get_path(),
            self.get_sequence()
        )
    }
}
// endregion


// region: --- horizon-client
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
        request: &SingleLedgerRequest<Sequence>,
    ) -> Result<SingleLedgerResponse, String> {
        self.get::<SingleLedgerResponse>(request).await
    }

    async fn get<TResponse: Response + std::fmt::Debug>(
        &self,
        request: &impl Req
    ) -> Result<TResponse, String> {
        let url = request.build_url(&self.base_url.0);
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
// endregion


#[cfg(test)]
mod tests {
    use base64::encode;

    use super::*;

    #[tokio::test]
    async fn test_get_single_ledger() {
        let horizon_client = HorizonClientNew::new()
            .base_url("https://horizon-testnet.stellar.org")
            .unwrap();

        let request = SingleLedgerRequest::new()
            .set_sequence(2).unwrap();
        
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