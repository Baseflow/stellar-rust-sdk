use derive_getters::Getters;
use url::Url;

use crate::models::Response;

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

impl SingleLedgerRequest {
    pub fn build_url(&self, base_url: &BaseUrl) -> String {
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
    pub async fn send<TResponse: Response + std::fmt::Debug>(
        &self,
        request: SingleLedgerRequest
    ) -> Result<TResponse, String> {
        match request.method() {
            HttpMethod::GET => Self::get(&self, request).await,
            HttpMethod::POST => todo!()
        }
    }

    async fn get<TResponse: Response + std::fmt::Debug>(
        &self,
        request: SingleLedgerRequest
    ) -> Result<TResponse, String> {
        let url = request.build_url(&self.base_url);
        let response = reqwest::get(&url).await.map_err(|e| e.to_string())?;
        println!("\n\nREQWEST RESPONSE: {:?}", response);
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
        
        let response: Result<SingleLedgerResponse, String> = horizon_client.send(request).await;

        assert!(response.clone().is_ok());
    }
}