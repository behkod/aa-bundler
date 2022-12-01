pub mod bundler;

use std::str::FromStr;

use clap::Parser;
use ethers::types::{Address, U256};

#[derive(Debug, Parser, PartialEq)]
pub struct Opts {
    #[clap(long, value_parser=parse_address)]
    pub beneficiary: Address,

    #[clap(long, default_value = "1", value_parser=parse_u256)]
    pub gas_factor: U256,

    #[clap(long, value_parser=parse_u256)]
    pub min_balance: U256,

    #[clap(long, value_parser=parse_address)]
    pub entry_point: Address,

    #[clap(long, value_parser=parse_address)]
    pub helper: Address,
}

fn parse_address(s: &str) -> Result<Address, String> {
    Address::from_str(s).map_err(|_| format!("Adress {} is not a valid address", s))
}

fn parse_u256(s: &str) -> Result<U256, String> {
    U256::from_str_radix(s, 10).map_err(|_| format!("{} is not a valid U256", s))
}

#[cfg(test)]
mod test {
    use super::Opts;
    use clap::Parser;
    use ethereum_types::{Address, U256};
    use std::str::FromStr;

    #[test]
    fn bundle_opt() {
        let args = vec![
            "bundleropts",
            "--beneficiary",
            "0x690B9A9E9aa1C9dB991C7721a92d351Db4FaC990",
            "--gas-factor",
            "600",
            "--min-balance",
            "1",
            "--entry-point",
            "0x0000000000000000000000000000000000000000",
            "--helper",
            "0x0000000000000000000000000000000000000000",
        ];
        assert_eq!(
            Opts {
                beneficiary: Address::from_str("0x690B9A9E9aa1C9dB991C7721a92d351Db4FaC990")
                    .unwrap(),
                gas_factor: U256::from(600),
                min_balance: U256::from(1),
                entry_point: Address::from([0; 20]),
                helper: Address::from([0; 20])
            },
            Opts::try_parse_from(args).unwrap()
        );
    }
}