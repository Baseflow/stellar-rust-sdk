use xdr_codec;

use self::stub_data::LedgerHeader;

/// Encodes a value to XDR.
/// # Arguments
/// * `value` - The value to encode
/// # Returns
/// The encoded value
/// # Errors
/// Returns an error if the encoding fails
pub fn encode<T>(value: &T) -> Result<String, String> {
    todo!();
}

/// decodes a value from XDR.
/// # Arguments
/// * `bytes` - The bytes to decode
/// # Returns
/// The decoded value
/// # Errors
/// Returns an error if the decoding fails
/// # Remarks
pub fn decode<T>(mut bytes: &[u8]) -> Result<LedgerHeader, String>
where
    T: Default,
{
    println!("bytes: {:?}", bytes);
    let xdr: Result<LedgerHeader, xdr_codec::Error> = xdr_codec::unpack(&mut bytes);
    match xdr {
        Ok(v) => Ok(v),
        Err(e) => {
            Err(e.to_string())
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stub_data::*;

    #[test]
    fn encode_ledger_header() {
        let ledger_header = "";
    }

    // TODO, add vice versa.
    // https://developers.stellar.org/docs/encyclopedia/xdr#parsing-xdr
    // See if we can use an XDR generator to generate structs for us.
    // Possible solution: https://github.com/stellar/xdrgen
    #[test]
    fn decode_ledger_header() {
        // Decode online at : https://stellar.github.io/xdr-viewer/?type=LedgerHeader&network=public
        let encoded = "AAAAAGPZj1Nu5o0bJ7W4nyOvUxG3Vpok+vFAOtC1K2M7B76ZuZRHr9UdXKbTKiclfOjy72YZFJUkJPVcKT5htvorm1QAAAAAZImGNAAAAAAAAAABAAAAAKgkzRi8nXUGTSmaW1uspDvDqi8yaTgVPYwvm7XLbfAzAAAAQLuRQK8ocAjytwfQelkpwZQa5+jReIO0pbr/9BbUbIffRxQN4Z76J3qDIn5lSJpn0OkYL8ZLPGP0W/S1vlTj5w/fP2GYBKkv20BXGS3EPddI6neK3FK8SYzoBSTAFLgRGXNSJ+05hGEpEjdoewhEaqLJsJbgyYpGLa3aVp8F3SSEAAAAAg3gtrOnZAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABkBfXhAAAAAGQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=".as_bytes();
        
        let decoded = decode::<LedgerHeader>(&encoded).unwrap();

        assert_eq!(decoded.ledger_version, 0);
        assert_eq!(decoded.previous_ledger_hash, "Y9mPU27mjRsntbifI69TEbdWmiT68UA60LUrYzsHvpk=".to_string());
        assert_eq!(decoded.tx_set_hash, "uZRHr9UdXKbTKiclfOjy72YZFJUkJPVcKT5htvorm1Q=".to_string());
        assert_eq!(decoded.close_time, 1686734388);

        // let decoded = decode::<LedgerHeader>(&encoded).unwrap();
        // assert_eq!(decoded, LedgerHeader::default());
        // assert_eq!(decoded.ledger_version, 0);
        // assert_eq!(
        //     decoded.previous_ledger_hash,
        //     "7KhW4Ac9wghySdySntMcCcO6v+8uaHtoXQUT2+ZImhg="
        // );
        // assert_eq!(decoded.scp_value, "");
        // assert_eq!(
        //     decoded.tx_set_hash,
        //     "jqmROBaaGtCUsskwAA+EA0DXG49VbiW3E/OP5gIanTc="
        // );
        // assert_eq!(decoded.close_time, 1686734395);
        // assert_eq!(decoded.upgrades.len(), 0);
        // assert_eq!(decoded.ext, vec!["stellarValueSigned"]);
        // assert_eq!(decoded.lc_value_signature, "");
        // assert_eq!(decoded.node_id, vec!["publicKeyTypeEd25519"]);
        // assert_eq!(
        //     decoded.ed25519,
        //     "7KhW4Ac9wghySdySntMcCcO6v+8uaHtoXQUT2+ZImhg="
        // );
        // assert_eq!(
        //     decoded.signature,
        //     "jqmROBaaGtCUsskwAA+EA0DXG49VbiW3E/OP5gIanTc="
        // );
        // assert_eq!(
        //     decoded.tx_set_result_hash,
        //     "jqmROBaaGtCUsskwAA+EA0DXG49VbiW3E/OP5gIanTc="
        // );
        // assert_eq!(
        //     decoded.bucket_list_hash,
        //     "jqmROBaaGtCUsskwAA+EA0DXG49VbiW3E/OP5gIanTc="
        // );
        // assert_eq!(decoded.ledger_seq, 3);
        // assert_eq!(decoded.total_coins, 1000000000000000000);
        // assert_eq!(decoded.fee_pool, 0);
        // assert_eq!(decoded.inflation_seq, 0);
        // assert_eq!(decoded.id_pool, 0);
        // assert_eq!(decoded.base_fee, 100);
        // assert_eq!(decoded.base_reserve, 100000000);
        // assert_eq!(decoded.max_tx_set_size, 100);
        // // assert_eq!(decoded.skip_list, vec![
        // //            "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
        // //            "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
        // //            "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
        // //            "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="]);
    }
}

mod stub_data {
    use xdr_codec::Unpack;

    #[derive(Debug)]
    pub struct LedgerHeader {
        pub ledger_version: i32,
        pub previous_ledger_hash: String,
        pub scp_value: String,
        pub tx_set_hash: String,
        pub close_time: i64,
        pub upgrades: Vec<String>,
        pub ext: Vec<String>,
        pub lc_value_signature: String,
        pub node_id: Vec<String>,
        pub ed25519: String,
        pub signature: String,
        pub tx_set_result_hash: String,
        pub bucket_list_hash: String,
        pub ledger_seq: i32,
        pub total_coins: i64,
        pub fee_pool: i64,
        pub inflation_seq: i32,
        pub id_pool: i64,
        pub base_fee: i32,
        pub base_reserve: i64,
        pub max_tx_set_size: i32,
        pub skip_list: Vec<String>,
    }

    impl PartialEq for LedgerHeader {
        fn eq(&self, other: &Self) -> bool {
            true
        }
    }


    impl Unpack<&[u8]> for LedgerHeader {
        fn unpack(input: &mut &[u8]) -> xdr_codec::Result<(Self, usize)> {
            let mut size = 0;
            let ledger_header = LedgerHeader {
                ledger_version: {
                    let (v, sz) = Unpack::unpack(input)?;
                    size += sz;
                    print!("ledger_version: {:?}\n", v);
                    v
                },
                previous_ledger_hash: {
                    let (v, sz) = Unpack::unpack(input)?;
                    size += sz;
                    v
                },
                scp_value: {
                    let (v, sz) = Unpack::unpack(input)?;
                    size += sz;
                    v
                },
                tx_set_hash: {
                    let (v, sz) = Unpack::unpack(input)?;
                    size += sz;
                    v
                },
                close_time: {
                    let (v, sz) = Unpack::unpack(input)?;
                    size += sz;
                    v
                },
                upgrades: {
                    let (v, sz) = Unpack::unpack(input)?;
                    size += sz;
                    v
                },
                ext: {
                    let (v, sz) = Unpack::unpack(input)?;
                    size += sz;
                    v
                },
                lc_value_signature: {
                    let (v, sz) = Unpack::unpack(input)?;
                    size += sz;
                    v
                },
                node_id: {
                    let (v, sz) = Unpack::unpack(input)?;
                    size += sz;
                    v
                },
                ed25519: {
                    let (v, sz) = Unpack::unpack(input)?;
                    size += sz;
                    v
                },
                signature: {
                    let (v, sz) = Unpack::unpack(input)?;
                    size += sz;
                    v
                },
                tx_set_result_hash: {
                    let (v, sz) = Unpack::unpack(input)?;
                    size += sz;
                    v
                },
                bucket_list_hash: {
                    let (v, sz) = Unpack::unpack(input)?;
                    size += sz;
                    v
                },
                ledger_seq: {
                    let (v, sz) = Unpack::unpack(input)?;
                    size += sz;
                    v
                },
                total_coins: {
                    let (v, sz) = Unpack::unpack(input)?;
                    size += sz;
                    v
                },
                fee_pool: {
                    let (v, sz) = Unpack::unpack(input)?;
                    size += sz;
                    v
                },
                inflation_seq: {
                    let (v, sz) = Unpack::unpack(input)?;
                    size += sz;
                    v
                },
                id_pool: {
                    let (v, sz) = Unpack::unpack(input)?;
                    size += sz;
                    v
                },
                base_fee: {
                    let (v, sz) = Unpack::unpack(input)?;
                    size += sz;
                    v
                },
                base_reserve: {
                    let (v, sz) = Unpack::unpack(input)?;
                    size += sz;
                    v
                },
                max_tx_set_size: {
                    let (v, sz) = Unpack::unpack(input)?;
                    size += sz;
                    v
                },
                skip_list: {
                    let (v, sz) = Unpack::unpack(input)?;
                    size += sz;
                    v
                },
            };
            
            println!("-------------LEDGER HEADER: {:?}", ledger_header);
            Ok((ledger_header, size))
        }
    }

    impl Default for LedgerHeader {
        fn default() -> Self {
            Self {
                ledger_version: 0,
                previous_ledger_hash: "".to_string(),
                scp_value: "".to_string(),
                tx_set_hash: "".to_string(),
                close_time: 0,
                upgrades: vec![],
                ext: vec![],
                lc_value_signature: "".to_string(),
                node_id: vec![],
                ed25519: "".to_string(),
                signature: "".to_string(),
                tx_set_result_hash: "".to_string(),
                bucket_list_hash: "".to_string(),
                ledger_seq: 0,
                total_coins: 0,
                fee_pool: 0,
                inflation_seq: 0,
                id_pool: 0,
                base_fee: 0,
                base_reserve: 0,
                max_tx_set_size: 0,
                skip_list: vec![],
            }
        }
    }
}
