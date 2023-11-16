use url::Url;

// --------------------------------SINGLE REQUEST--------------------------------

const SINGLE_LEDGERS_PATH: &str = "/ledgers";

pub struct SingleLedgerRequest {
    /// The sequence of the ledger
    sequence: u32,
    base_path: String
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
}

impl Default for SingleLedgerRequestBuilder<NoSequence> {
    fn default() -> Self {
        SingleLedgerRequestBuilder { 
            sequence: NoSequence,
            path: SINGLE_LEDGERS_PATH.into(),
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
            path: SINGLE_LEDGERS_PATH.into()
        }
    }
}

impl SingleLedgerRequestBuilder<Sequence> {
    pub fn build(self) -> Result<SingleLedgerRequest, String> {
        Ok(SingleLedgerRequest { 
            sequence: self.sequence.0,
            base_path: self.path
        })
    }
}
// --------------------------------------------------------------------------------




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
    pub fn test() {
        println!("yay")
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


#[cfg(test)]
mod tests {
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
}