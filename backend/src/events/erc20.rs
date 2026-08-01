use alloy::primitives::{Address, U256};
use alloy::sol;
use alloy::sol_types::SolEvent;

// alloy macros
// here we are defining an ERC20 Transfer event
sol! {
    // Transfer event from ERC20 contract
    event Transfer(address indexed from, address indexed to, uint256 value);
}

// a clean model for ERC20 transfer event
#[derive(Debug, PartialEq, Eq)]
pub struct DecodedTransfer {
    pub from: Address,
    pub to: Address,
    pub value: U256,
}

// parse raw log topics and data into a DecodedTransfer struct
// returns None if the log does not match the Transfer event signature
impl DecodedTransfer {
    pub fn from_log(log: &alloy::rpc::types::Log) -> Option<Self> {
        // decode log
        let decoded = Transfer::decode_raw_log(log.topics(), log.data().data.as_ref());

        match decoded {
            Ok(decoded) => Some(Self {
                from: decoded.from,
                to: decoded.to,
                value: decoded.value,
            }),
            Err(_) => None,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_erc20_transfer_signature() {
        // verify that the computed event signature hash matches the ERC20 standard
        let expected_signature = "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef";
        assert_eq!(Transfer::SIGNATURE_HASH.to_string(), expected_signature);
    }
}