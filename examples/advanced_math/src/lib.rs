use scrypto::prelude::*;
use scrypto_math::*;

fn calculate_output(amount: BalancedDecimal) -> Option<BalancedDecimal> {
    // the ?-operator is used for convenience
    // it returns None if the intermediate result is None
    let output_1 = amount.pow(bdec!("2.54"))?;
    let output_2 = amount.exp()?.log10()?;
    Some(output_1 + output_2)
}

#[blueprint]
mod advanced_math {
    struct AdvancedMathDemo {
        oci_vault: Vault,
    }

    impl AdvancedMathDemo {
        pub fn instantiate() -> Global<AdvancedMathDemo> {
            let oci_bucket: Bucket =
                ResourceBuilder::new_fungible(OwnerRole::None).mint_initial_supply(1000);
            Self {
                oci_vault: Vault::with_bucket(oci_bucket),
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize()
        }

        pub fn free_tokens(&mut self, amount: Decimal) -> (Bucket, Decimal) {
            let amount = BalancedDecimal::try_from(amount).expect("Value too large.");
            let output_amount = calculate_output(amount).expect("Failed output calculation.");
            let output = self.oci_vault.take(output_amount.floor_to_decimal());
            (output, output_amount.into())
        }
    }
}
