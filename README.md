# Stellar-rs - Empowering Stellar Developers with Rust's Performance and Security

![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/Baseflow/stellar-rust-sdk/.github%2Fworkflows%2Fcargo-build-and-test.yaml)
[![Crates.io](https://img.shields.io/crates/v/stellar-sdk.svg)](https://crates.io/crates/stellar-rs)
[![Documentation](https://img.shields.io/badge/documentation-1)](https://docs.rs/stellar-rs/latest/stellar_rs/index.html)
[![GitHub Issues](https://img.shields.io/github/issues/Baseflow/stellar-rust-sdk)]()
[![GitHub Stars](https://img.shields.io/github/stars/Baseflow/stellar-rust-sdk)]()
[![GitHub License](https://img.shields.io/github/license/Baseflow/stellar-rust-sdk)]()

API documentation is available [here](https://docs.rs/stellar-rs/1.0.0).

The Rust Stellar SDK enables efficient and safe communication with [Stellar's
Horizon API](https://developers.stellar.org/docs/data/horizon). Our goal is to
provide an optimal developer experience by utilizing Rust's performance and
safety features. By employing the [Type State Builder
Pattern](https://www.youtube.com/watch?v=pwmIQzLuYl0), we prevent incomplete or
invalid requests from being made, reducing the possibility of runtime exceptions
and improving API request reliability.

## Installation

To add `stellar-rs` to your project, run the following Cargo command in your
project directory:

```bash
cargo add stellar-rs
```

Alternatively, add the following line to your `Cargo.toml`:

```toml
stellar-rs = "1.0.0"
```

## Getting Started

To begin communicating with the Horizon API, initialize a new Horizon client.
You can specify whether to use the testnet or the production environment.

```rust
use stellar_rs::horizon_client::HorizonClient;

async fn example() -> Result<(), Box<dyn std::error::Error>> {
    let horizon_client = HorizonClient::new("https://horizon-testnet.stellar.org")?;
    Ok(())
}
```

After initializing the Horizon client, specify the request you want to execute.
The request builders ensure that you construct valid requests, preventing
invalid parameter combinations.

```rust
use stellar_rs::assets::prelude::{AllAssetsRequest, AllAssetsResponse};

static ACCOUNT_ID: &str = "GDIGRW2H37U3O5WPMQFWGN35DDVZAYYTIMGLYVQI4XTATZBW4FXEATRE";

// Construct the request
let accounts_request = AccountsRequest::new()
    .set_signer_filter(ACCOUNT_ID)
    .unwrap()
    .set_limit(10)
    .unwrap();
```

Once the `signer_filter` is set, it cannot be changed again. The state
transitions from one where no filter is applied to one where the `signer_filter`
is set. Similarly, methods like `set_asset_filter`, `set_liquidity_pool_filter`,
and `set_sponsor_filter` become unavailable because the Horizon API allows only
one of these filters per request. This is enforced using the type state builder
pattern.

Once the request is constructed, you can execute it using the previously
initialized Horizon client:

```rust
use stellar_rs::assets::prelude::{AllAssetsRequest, AllAssetsResponse};

let accounts_response = horizon_client.get_account_list(&accounts_request).await?;
```

## Supported Endpoints

As of version 1.0, `stellar-rs` supports all endpoints and models for the Horizon API:

* [Accounts](https://developers.stellar.org/docs/data/horizon/api-reference/resources/accounts)
* [Assets](https://developers.stellar.org/docs/data/horizon/api-reference/resources/assets)
* [Claimable Balances](https://developers.stellar.org/docs/data/horizon/api-reference/resources/claimablebalances)
* [Effects](https://developers.stellar.org/docs/data/horizon/api-reference/resources/effects)
* [Fee Stats](https://developers.stellar.org/docs/data/horizon/api-reference/aggregations/fee-stats)
* [Ledgers](https://developers.stellar.org/docs/data/horizon/api-reference/resources/ledgers)
* [Liquidity Pools](https://developers.stellar.org/docs/data/horizon/api-reference/resources/liquiditypools)
* [Operations](https://developers.stellar.org/docs/data/horizon/api-reference/resources/operations)
* [Offers](https://developers.stellar.org/docs/data/horizon/api-reference/resources/offers)
* [Order Books](https://developers.stellar.org/docs/data/horizon/api-reference/aggregations/order-books)
* [Paths](https://developers.stellar.org/docs/data/horizon/api-reference/aggregations/paths)
* [Payments](https://developers.stellar.org/docs/data/horizon/api-reference/resources/payments)
* [Trades](https://developers.stellar.org/docs/data/horizon/api-reference/resources/trades)
* [Trade Aggregations](https://developers.stellar.org/docs/data/horizon/api-reference/aggregations/trade-aggregations)
* [Transactions](https://developers.stellar.org/docs/data/horizon/api-reference/resources/transactions)

## Contributing

Contributions are welcome! If you find a bug or have a feature request, please
[open an issue](https://github.com/Baseflow/stellar-rust-sdk/issues). If you'd
like to contribute code, feel free to open a pull request.

## License

This project is licensed under the MIT License. See [LICENSE-MIT](./LICENSE) or
visit [https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT)
for more information.

