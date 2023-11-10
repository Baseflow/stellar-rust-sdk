#[cfg(test)]
mod tests {

    use ::stellar_xdr::{LedgerHeader, ReadXdr};
    use stellar_xdr::curr as stellar_xdr;

    // TODO, add vice versa.
    // https://developers.stellar.org/docs/encyclopedia/xdr#parsing-xdr
    // See if we can use an XDR generator to generate structs for us.
    // Possible solution: https://github.com/stellar/xdrgen
    #[test]
    fn decode_ledger_header() {
        // Decode online at : https://stellar.github.io/xdr-viewer/?type=LedgerHeader&network=public
        let encoded: &[u8] = "AAAAAGPZj1Nu5o0bJ7W4nyOvUxG3Vpok+vFAOtC1K2M7B76ZuZRHr9UdXKbTKiclfOjy72YZFJUkJPVcKT5htvorm1QAAAAAZImGNAAAAAAAAAABAAAAAKgkzRi8nXUGTSmaW1uspDvDqi8yaTgVPYwvm7XLbfAzAAAAQLuRQK8ocAjytwfQelkpwZQa5+jReIO0pbr/9BbUbIffRxQN4Z76J3qDIn5lSJpn0OkYL8ZLPGP0W/S1vlTj5w/fP2GYBKkv20BXGS3EPddI6neK3FK8SYzoBSTAFLgRGXNSJ+05hGEpEjdoewhEaqLJsJbgyYpGLa3aVp8F3SSEAAAAAg3gtrOnZAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABkBfXhAAAAAGQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=".as_bytes();

        let decoded = stellar_xdr::LedgerHeader::from_xdr_base64(encoded).unwrap();

        assert_eq!(decoded.ledger_version, 0);
        assert_eq!(
            decoded.previous_ledger_hash.to_string(),
            "63d98f536ee68d1b27b5b89f23af5311b7569a24faf1403ad0b52b633b07be99"
        );
        assert_eq!(decoded.scp_value.upgrades.len(), 0);
        assert_eq!(
            decoded.scp_value.tx_set_hash.to_string(),
            "b99447afd51d5ca6d32a27257ce8f2ef661914952424f55c293e61b6fa2b9b54"
        );
        assert_eq!(decoded.scp_value.close_time.0, 1686734388);
        assert_eq!(
            decoded.tx_set_result_hash.to_string(),
            "df3f619804a92fdb4057192dc43dd748ea778adc52bc498ce80524c014b81119"
        );

        match decoded.scp_value.ext {
            ::stellar_xdr::StellarValueExt::Signed(signed) => {
                assert_eq!(
                    signed.node_id.0.discriminant().to_string(),
                    "PublicKeyTypeEd25519"
                );
                assert_eq!(signed.node_id.0.name().to_string(), "PublicKeyTypeEd25519");
                // todo check node-id public key
                // todo check signature
            }
            _ => panic!("Expected signed"),
        }

        assert_eq!(
            decoded.bucket_list_hash.to_string(),
            "735227ed398461291237687b08446aa2c9b096e0c98a462dadda569f05dd2484"
        );
        assert_eq!(decoded.ledger_seq, 2);
        assert_eq!(decoded.total_coins, 1000000000000000000);
        assert_eq!(decoded.fee_pool, 0);
        assert_eq!(decoded.inflation_seq, 0);
        assert_eq!(decoded.id_pool, 0);
        assert_eq!(decoded.base_fee, 100);
        assert_eq!(decoded.base_reserve, 100000000);
        assert_eq!(decoded.max_tx_set_size, 100);
        assert_eq!(decoded.ext, stellar_xdr::LedgerHeaderExt::V0);
        for decoded in decoded.skip_list {
            assert_eq!(
                decoded.to_string(),
                "0000000000000000000000000000000000000000000000000000000000000000"
            );
        }
    }
}
